use super::{Distance, SVG};
use super::{Unit, Vector};

use derive_more::{Display, Into, From};

/**************/
/* STRUCTURES */
/**************/

#[derive(Copy, Clone)]
#[derive(From, Into)]
#[derive(Debug, Default)]
#[derive(Display)]
#[display(fmt = "({} ; {})", x, y)]
pub struct Point { pub x: Unit, pub y: Unit }

/*******************/
/* IMPLEMENTATIONS */
/*******************/

impl Distance for Point {
    fn squared_distance_from(&self, other: &Self) -> Unit {
        (self.x - other.x).powi(2) + (self.y - other.y).powi(2)
    }
}

impl SVG for Point {
    fn to_svg(&self) -> String
    {
        format!(r#"<circle cx="{}" cy="{}" r="1" />"#, self.x, self.y)
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool
    {
        self.distance_from(other) < Unit::EPSILON
    }
}

impl From<Vector> for Point {
    fn from(value: Vector) -> Self { Self { x: value.x, y: value.y } }
}

#[cfg(test)]
mod tests
{
    use super::*;

    /**************/
    /* `Distance` */
    /**************/

    #[test]
    pub fn test_distance_from()
    {
        let a = Point { x: 0., y: -1. };
        let b = Point { x: 0., y: 1. };

        assert_eq!(a.distance_from(&b), 2.);
    }

    /*************/
    /* OPERATORS */
    /*************/

    #[test]
    pub fn test_eq_below_epsilon()
    {
        let testing = Point { x: 0., y: Unit::EPSILON / 10. };

        assert_eq!(testing, Point::default());
    }

    #[test]
    pub fn test_eq_epsilon()
    {
        let testing = Point { x: 0., y: Unit::EPSILON };

        assert_ne!(testing, Point::default());
    }

    #[test]
    pub fn test_eq_above_epsilon()
    {
        let testing = Point { x: 0., y: Unit::EPSILON * 10. };

        assert_ne!(testing, Point::default());
    }
}
