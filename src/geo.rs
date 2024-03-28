mod point;
pub use point::Point;

pub mod polygon;
pub use polygon::Polygon;

mod segment;
pub use segment::Segment;

mod vector;
pub use vector::Vector;

use polygon::Any;
#[cfg(feature = "frames")] use polygon::Rectangle;

use super::tree::Node;

use std::collections::VecDeque;
use std::rc::Rc;

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

#[cfg(feature = "frames")]
pub fn generate_tree_from_polygons(nodes: &Vec<Rc<Node<(isize, Any)>>>)
    -> Rc<Node<(isize, Any)>>
{
    let frames =
        nodes
            .iter()
            .map(|node| node.value())
            .map(|item| (item.0, item.1.frame()))
            .map(|item| Node::new(item));

    let mut placement_queue = VecDeque::new();
    let root = Node::new((-1, Rectangle::default()));

    for frame in frames {
        root.adopt(&frame);
        placement_queue.push_back(frame);
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

    let ret = Node::new((-1, Any::default()));

    let mut parents_to_place = VecDeque::new();
    parents_to_place.push_back(ret.clone());

    let mut children_to_place = VecDeque::new();
    children_to_place.push_back(root.children().clone());

    while !parents_to_place.is_empty() {
        let parent = parents_to_place.pop_front().unwrap();
        let children = children_to_place.pop_front().unwrap();

        for child in children.iter() {
            let new_parent = nodes[child.value().0 as usize].clone();

            parent.adopt(&new_parent);
            parents_to_place.push_back(new_parent);
            children_to_place.push_back(child.children().clone());
        }
    }

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
                    .clone()
                    .into_iter()
                    .filter(|brother| !Rc::ptr_eq(brother, &node))
                    .filter(|brother| !Rc::ptr_eq(brother, &parent))
                    .filter(|brother| brother.value().1.contains(child));

            for brother in brothers {
                brother.adopt(&node);
            }
        }
    }

    ret
}

#[cfg(feature = "naive")]
pub fn generate_tree_from_polygons(nodes: &Vec<Rc<Node<(isize, Any)>>>)
    -> Rc<Node<(isize, Any)>>
{
    let ret = Node::new((-1, Any::default()));

    let mut placement_queue = VecDeque::new();

    for node in nodes {
        ret.adopt(node);
        placement_queue.push_back(node);
    }

    while !placement_queue.is_empty() {
        let to_place = placement_queue.pop_front().unwrap();
        let parent = to_place.parent().unwrap();
        let brothers = parent.children().clone();

        let brothers =
            brothers
                .iter()
                .filter(|child| !Rc::ptr_eq(child, &to_place));

        for brother in brothers {
            if brother.value().1.contains(&to_place.value().1) {
                brother.adopt(&to_place);
                placement_queue.push_back(to_place);

                break;
            }
        }
    }

    ret
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
