use super::super::tree::Node;
use super::super::{IndexedNode, IndexedNodes};

use super::super::geo::polygon::Any;
#[cfg(feature = "frames")] use super::super::geo::polygon::Rectangle;
use super::super::geo::{Container, Polygon};

use std::collections::VecDeque;
use std::rc::Rc;

/*************/
/* FUNCTIONS */
/*************/

#[cfg(feature = "frames")]
pub fn process(nodes: &IndexedNodes<Any>) -> IndexedNode<Any>
{
    let frames = generate_frames(nodes);
    let from = build_tree_from_polygons(frames.iter());

    transpose_rec_to_any(&frames, nodes, from)
}

#[cfg(feature = "naive")]
pub fn process(nodes: &IndexedNodes<Any>) -> IndexedNode<Any>
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
