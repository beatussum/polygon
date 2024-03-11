mod point;
pub use point::Point;

pub mod polygon;
pub use polygon::Polygon;

mod segment;
pub use segment::Segment;

mod vector;
pub use vector::Vector;

use symm_impl::symmetric;

const EPSILON: f32 = 1e-5;
pub type Unit = f32;

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

fn are_ccw(&a: &Point, &b: &Point, &c: &Point) -> bool
{
    Vector::from((a, b)).det(&(a, c).into()) > 0.
}

#[symmetric]
impl Distance<Point> for Segment {
    fn squared_distance_from(&self, other: &Point) -> Unit
    {
        let (start, stop) = (*self).into();

        if self.contains(other) {
            0.
        } else {
            let projection =
                Vector::from((start, *other))
                .dot(&(*self).into());

            let oh =
                Vector::from((Point::default(), start)) +
                (projection / Vector::from(*self).squared_norm()) *
                Vector::from(*self);

            if self.contains(&Point::from(oh)) {
                (oh - (*other).into()).squared_norm()
            } else if projection < 0. {
                start.squared_distance_from(other)
            } else {
                stop.squared_distance_from(other)
            }
        }
    }
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
