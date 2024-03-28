mod point;
pub use point::Point;

pub mod polygon;
pub use polygon::Polygon;

mod segment;
pub use segment::Segment;

mod vector;
pub use vector::Vector;

use polygon::Any;
use super::tree::Node;

use std::collections::VecDeque;
use std::rc::Rc;

/*********/
/* TYPES */
/*********/

pub type IndexedNode = Rc<Node<(isize, Any)>>;
pub type IndexedNodes = Vec<IndexedNode>;

pub type Unit = f64;

/**********/
/* TRAITS */
/**********/

pub trait Container<Other = Self>
{
    fn contains(&self, other: &Other) -> bool;
}

pub trait Distance<Other = Self>
{
    fn distance_from(&self, other: &Other) -> Unit
    {
        self.squared_distance_from(other).sqrt()
    }

    fn squared_distance_from(&self, other: &Other) -> Unit;
}

pub trait SVG { fn to_svg(&self) -> String; }

/*************/
/* FUNCTIONS */
/*************/

fn are_ccw(&a: &Point, &b: &Point, &c: &Point) -> bool
{
    Vector::from((a, b)).det(&(a, c).into()) > 0.
}

fn build_tree_from_polygons<'a, T:'a , U>(nodes: U) -> Rc<Node<(isize, T)>>
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
pub fn generate_tree_from_polygons(nodes: &IndexedNodes) -> IndexedNode
{
    // Build associated-frames hierarchy.

    let frames =
        nodes
            .iter()
            .map(|node| node.value())
            .map(|item| (item.0, item.1.frame()))
            .map(|item| Node::new(item))
            .collect::<Vec<_>>();

    let root_of_frames = build_tree_from_polygons(frames.iter());

    // Copy the hierarchy to a tree of `Any`s.

    let ret = Node::new((-1, Any::default()));

    let mut parents_to_place = VecDeque::new();
    parents_to_place.push_back(&ret);

    let mut children_to_place = VecDeque::new();
    children_to_place.push_back(root_of_frames.children());

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

            let new_parent = node.parent().unwrap();
            let children = new_parent.children();

            let mut brothers =
                children
                    .iter()
                    .filter(|brother| !Rc::ptr_eq(brother, &node))
                    .filter(|brother| !Rc::ptr_eq(brother, &parent))
                    .filter(|brother| brother.value().1.contains(child));

            match brothers.next() {
                Some(brother) => brother.adopt(&node),
                None => ()
            }
        }
    }

    ret
}

#[cfg(feature = "naive")]
pub fn generate_tree_from_polygons(nodes: &IndexedNodes) -> IndexedNode
{
    build_tree_from_polygons(nodes.iter())
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    pub fn test_are_ccw()
    {
        let a = Point { x: 0., y: -1. };
        let b = Point { x: 1., y: 0. };
        let c = Point { x: 0., y: 1. };

        assert!(are_ccw(&a, &b, &c));
        assert!(are_ccw(&b, &c, &a));
        assert!(are_ccw(&c, &a, &b));
    }

    #[test]
    pub fn test_are_not_ccw()
    {
        let a = Point { x: 0., y: -1. };
        let b = Point { x: 1., y: 0. };
        let c = Point { x: 0., y: 1. };

        assert!(!are_ccw(&c, &b, &a));
        assert!(!are_ccw(&a, &c, &b));
        assert!(!are_ccw(&b, &a, &c));
    }
}
