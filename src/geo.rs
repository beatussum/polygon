mod point;
pub use point::Point;

pub mod polygon;
pub use polygon::Polygon;

mod segment;
pub use segment::Segment;

mod vector;
pub use vector::Vector;

/*********/
/* TYPES */
/*********/

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

pub trait Intersecter<Other = Self>
{
    fn intersects(&self, other: &Other) -> bool;
}

pub trait SVG { fn to_svg(&self) -> String; }

/*************/
/* FUNCTIONS */
/*************/

fn are_ccw(&a: &Point, &b: &Point, &c: &Point) -> bool
{
    Vector::from((a, b)).det(&(a, c).into()) > 0.
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
