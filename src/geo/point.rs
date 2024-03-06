use super::{Distance, Unit, Vector};
use super::EPSILON;

use derive_more::{Display, Into, From};

#[derive(Copy, Clone)]
#[derive(From, Into)]
#[derive(Debug, Default)]
#[derive(Display)]
#[display(fmt = "({} ; {})", x, y)]
pub struct Point { pub x: Unit, pub y: Unit }

impl Distance for Point {
    fn distance_from(&self, other: &Self) -> Unit {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool { self.distance_from(other) < EPSILON }
}

impl Eq for Point {}

impl From<Vector> for Point {
    fn from(value: Vector) -> Self { Self { x: value.x, y: value.y } }
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    pub fn test_distance_from()
    {
        assert_eq!(
            Point { x: 0., y: -1. }.distance_from(&Point { x: 0., y: 1. }),
            2.
        );
    }

    #[test]
    pub fn test_eq_below_epsilon()
    {
        assert_eq!(Point::default(), Point { x: 0., y: EPSILON / 10. });
    }

    #[test]
    pub fn test_eq_epsilon()
    {
        assert_ne!(Point::default(), Point { x: 0., y: EPSILON });
    }

    #[test]
    pub fn test_eq_above_epsilon()
    {
        assert_ne!(Point::default(), Point { x: 0., y: EPSILON * 10. });
    }
}
