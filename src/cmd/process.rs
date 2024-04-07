use super::super::tree::Node;
use super::super::{IndexedNode, IndexedNodes};

use super::super::geo::polygon::Any;
use super::super::geo::{Container, Polygon};
#[cfg(feature = "dac")] use super::super::geo::polygon::frame_of;
#[cfg(feature = "dac")] use super::super::geo::Segment;
#[cfg(feature = "frames")] use super::super::geo::polygon::Rectangle;

use std::collections::VecDeque;
use std::rc::Rc;

/*************/
/* FUNCTIONS */
/*************/

#[cfg(feature = "dac")]
pub fn process_dac(nodes: &IndexedNodes<Any>) -> IndexedNode<Any>
{
    let frames = generate_frames(nodes);

    let big_frame = {
        let points =
            frames
                .iter()
                .map(
                    |frame| {
                        let value = &frame.value().1;

                        [value.bottom_left(), value.top_right()]
                    }
                )
                .flatten();

        frame_of(points)
    };

    let from = divide(frames.clone(), big_frame);

    transpose_rec_to_any(&frames, nodes, from)
}

#[cfg(feature = "frames")]
pub fn process_frames(nodes: &IndexedNodes<Any>) -> IndexedNode<Any>
{
    let frames = generate_frames(nodes);
    let from = build_tree_from_polygons(frames.iter());

    transpose_rec_to_any(&frames, nodes, from)
}

#[cfg(feature = "naive")]
pub fn process_naive(nodes: &IndexedNodes<Any>) -> IndexedNode<Any>
    { build_tree_from_polygons(nodes.iter()) }

#[cfg(any(feature = "frames", feature = "naive"))]
fn build_tree_from_polygons<'a, T:'a , U>(nodes: U) -> IndexedNode<T>
    where
        T: Container + Default + Polygon,
        U: Iterator<Item = &'a Rc<Node<(isize, T)>>>
{
    let ret = Node::new((-1, T::default()));

    let mut placement_queue = VecDeque::new();

    for node in nodes {
        ret.adopt(node);
        placement_queue.push_back(node.clone());
    }

    while !placement_queue.is_empty() {
        let selected = placement_queue.pop_front().unwrap();
        let parent = selected.parent().unwrap();
        let brothers = parent.children().clone();

        let brothers =
            brothers
                .into_iter()
                .filter(|child| !Rc::ptr_eq(child, &selected))
                .filter(
                    |brother| {
                        selected.value().1.contains(&brother.value().1)
                    }
                );

        for brother in brothers {
            selected.adopt(&brother);

            if selected.children().len() != 1 {
                placement_queue.push_back(brother);
            }
        }
    }

    ret
}

#[cfg(feature = "dac")]
fn split(
    frames: IndexedNodes<Rectangle>,
    separator: Segment
) -> (IndexedNodes<Rectangle>, IndexedNodes<Rectangle>)
{
    use super::super::geo::Vector;

    let u: Vector = separator.into();

    let ret_a =
        frames
            .iter()

            .filter(
                |frame| {
                    frame
                        .value()
                        .1
                        .polygon()
                        .points()
                        .map(|point| (*separator.start(), *point).into())
                        .any(|v| u.det(&v) > 0.)
                }
            )

            .cloned()
            .collect();

    let ret_b =
        frames
            .into_iter()

            .filter(
                |frame| {
                    frame
                        .value()
                        .1
                        .polygon()
                        .points()
                        .map(|point| (*separator.start(), *point).into())
                        .any(|v| u.det(&v) < 0.)
                }
            )

            .collect();

    (ret_a, ret_b)
}

#[cfg(feature = "dac")]
fn conquer(
    a: IndexedNode<Rectangle>,
    b: IndexedNode<Rectangle>
) -> IndexedNode<Rectangle>
{
    'a : while !a.is_leaf() {
        let a = a.children()[0].clone();

        for i in 0..(b.children().len()) {
            if b.children()[i].value().1.contains(&a.value().1) {
                b.children()[i].adopt(&a);
                continue 'a;
            }
        }

        b.adopt(&a);
    }

    b
}

#[cfg(feature = "dac")]
fn divide(
    frames: IndexedNodes<Rectangle>,
    big_frame: Rectangle
) -> IndexedNode<Rectangle>
{
    let mut root = Node::new((-1, Rectangle::default()));

    match frames.len() {
        0 => (),
        1 => root.adopt(&frames[0]),

        2 => {
            let a = &frames[0].value().1;
            let b = &frames[1].value().1;

            if a.contains(b) {
                root.adopt(&frames[0]);
                frames[0].adopt(&frames[1]);
            } else if b.contains(a) {
                root.adopt(&frames[1]);
                frames[1].adopt(&frames[0]);
            } else {
                root.adopt(&frames[0]);
                root.adopt(&frames[1]);
            }
        }

        _ => {
            let n = frames.len();

            let (mut rect_a, mut separator, mut rect_b) =
                big_frame.divide_vertically();

            let (mut a, mut b) = split(frames.clone(), separator);

            if (a.len() == n) || (b.len() == n) {
                (rect_a, separator, rect_b) = big_frame.divide_horizontally();

                (a, b) = split(frames, separator);
            }

            let to_order = {
                if a.len() == n {
                    Some(&a)
                } else if b.len() == n {
                    Some(&b)
                } else {
                    None
                }
            };

            root =
                match to_order {
                    Some(to_order)
                        => build_tree_from_polygons(to_order.iter()),

                    None => conquer(divide(a, rect_a), divide(b, rect_b))
                }
        }
    }

    root
}

#[cfg(feature = "frames")]
fn generate_frames(nodes: &IndexedNodes<Any>) -> IndexedNodes<Rectangle>
{
    nodes
        .iter()
        .map(|node| node.value())
        .map(|item| (item.0, item.1.frame()))
        .map(|item| Node::new(item))
        .collect()
}

#[cfg(feature = "frames")]
fn transpose_rec_to_any(
    frames: &IndexedNodes<Rectangle>,
    nodes: &IndexedNodes<Any>,
    from: IndexedNode<Rectangle>
) -> IndexedNode<Any>
{
    // Copy the hierarchy to a tree of `Any`s.

    let ret = Node::new((-1, Any::default()));

    let mut parents_to_place = VecDeque::new();
    parents_to_place.push_back(&ret);

    let mut children_to_place = VecDeque::new();
    children_to_place.push_back(from.children());

    while !parents_to_place.is_empty() {
        let parent = parents_to_place.pop_front().unwrap();
        let children = children_to_place.pop_front().unwrap();

        for child in children.iter() {
            let index = child.value().0 as usize;
            let new_parent = &nodes[index];

            parent.adopt(new_parent);
            parents_to_place.push_back(new_parent);
            children_to_place.push_back(frames[index].children());
        }
    }

    // Fix false inclusions.

    let nodes =
        ret
            .bfs_iter()
            .skip(1)
            .filter(|node| !Rc::ptr_eq(&node.parent().unwrap(), &ret));

    for node in nodes {
        let child = &node.value().1;
        let parent = node.parent().unwrap();

        if !parent.value().1.contains(child) {
            node.upgrade();

            let brothers =
                node
                    .parent()
                    .unwrap()
                    .children()
                    .iter()
                    .filter(|brother| !Rc::ptr_eq(brother, &node))
                    .filter(|brother| !Rc::ptr_eq(brother, &parent))
                    .filter(|brother| brother.value().1.contains(child))
                    .cloned()
                    .next();

            match brothers {
                Some(brother) => brother.adopt(&node),
                None => ()
            }
        }
    }

    ret
}
