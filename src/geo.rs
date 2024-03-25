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

#[cfg(feature = "stupid")]
pub fn generate_tree_from_polygons(polygons: Vec<Any>)
    -> Rc<Node<(isize, Any)>>
{
    let ret = Node::new((-1, Any::default()));

    let mut placement_queue = VecDeque::new();

    for (i, p) in polygons.into_iter().enumerate() {
        let node = Node::new((i as isize, p));

        ret.adopt(&node);
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
