use super::super::tree::Node;
use super::super::{IndexedNode, IndexedNodes};

use super::super::geo::polygon::Any;
use super::super::geo::Container;

#[cfg(feature = "frames")] use super::super::geo::Polygon;
#[cfg(feature = "frames")] use super::super::geo::polygon::Rectangle;

use std::rc::Rc;

/*************/
/* FUNCTIONS */
/*************/

#[cfg(feature = "frames")]
pub fn process_frames(nodes: &IndexedNodes) -> IndexedNode
{
    let ret = Node::new((-1, Any::default()));

    let frames = generate_frames(nodes);

    for node in nodes {
        ret.adopt(node);
    }

    build_tree_from_polygons(nodes, |s, b| contains(&frames, b, s));

    ret
}

#[cfg(feature = "naive")]
pub fn process_naive(nodes: &IndexedNodes) -> IndexedNode
{
    let ret = Node::new((-1, Any::default()));

    for node in nodes {
        ret.adopt(node);
    }

    build_tree_from_polygons(nodes, |s, b| b.value().1.contains(&s.value().1));

    ret
}

fn build_tree_from_polygons<F>(nodes: &IndexedNodes, filter: F)
    where F: Fn(&IndexedNode, &IndexedNode) -> bool
{
    use std::collections::VecDeque;

    let mut placement_queue = VecDeque::new();

    for node in nodes {
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
                .map(|brother| (selected.clone(), brother))
                .filter(|(s, b)| filter(s, b));

        for (s, b) in brothers {
            b.adopt(&s);

            placement_queue.push_back(s);

            break;
        }
    }
}

#[cfg(feature = "frames")]
fn contains(frames: &Vec<Rectangle>, a: &IndexedNode, b: &IndexedNode) -> bool
{
    let a = a.value();
    let b = b.value();

    let is_frame_contained =
        frames[a.0 as usize].contains(&frames[b.0 as usize]);

    is_frame_contained && a.1.contains(&b.1)
}

#[cfg(feature = "frames")]
fn generate_frames(nodes: &IndexedNodes) -> Vec<Rectangle>
{
    nodes
        .iter()
        .map(|node| node.value())
        .map(|item| item.1.frame())
        .collect()
}
