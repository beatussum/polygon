use super::{are_ccw, EPSILON, Point, Unit, Vector};
use derive_more::{Display, Into, From};

#[derive(Eq, PartialEq)]
#[derive(Copy, Clone)]
#[derive(From, Into)]
#[derive(Debug, Default)]
#[derive(Display)]
#[display(fmt = "[{} ; {}]", start, stop)]
pub struct Segment { pub start: Point, pub stop: Point }

impl Segment {
    pub fn contains(&self, point: &Point) -> bool
    {
        let dist =
            self.start.distance_from(point) +
            self.stop.distance_from(point);

        (dist - self.length()).abs() < EPSILON
    }

    pub fn distance_from(&self, rhs: &Self) -> Unit
    {
        Vector::from(self).orthogonal().dot(&rhs.into()).abs()
    }

    pub fn is_horizontal(&self) -> bool
    {
        (self.start.y - self.stop.y).abs() < EPSILON
    }

    pub fn is_vertical(&self) -> bool
    {
        (self.start.x - self.stop.x).abs() < EPSILON
    }

    pub fn is_secant_with(&self, rhs: &Self) -> bool
    {
        let (a, b) = (*self).into();
        let (c, d) = (*rhs).into();

        (are_ccw(&a, &c, &d) != are_ccw(&b, &c, &d)) &&
            (are_ccw(&a, &b, &c) != are_ccw(&a, &b, &d))
    }

    pub fn length(&self) -> Unit { self.start.distance_from(&self.stop) }
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn test_contains()
    {
        assert!(
            Segment {
                start: Point { x: -1., y: -1. },
                stop: Point { x: 1., y: 1. }
            }.contains(&Point::default())
        );
    }

    #[test]
    fn test_not_contains()
    {
        assert!(
            !Segment {
                start: Point { x: -1., y: -1. },
                stop: Point { x: 1., y: 1. }
            }.contains(&Point { x: 2., y: 2. })
        );
    }

    #[test]
    fn test_is_vertical()
    {
        assert!(
            Segment {
                start: Point { x: -EPSILON / 10., y: -1. },
                stop: Point { x: EPSILON / 10., y: 1. }
            }.is_vertical()
        );
    }

    #[test]
    fn test_is_not_vertical()
    {
        assert!(
            !Segment {
                start: Point { x: -1., y: -1. },
                stop: Point { x: 1., y: 1. }
            }.is_vertical()
        );
    }

    #[test]
    fn test_is_horizontal()
    {
        assert!(
            Segment {
                start: Point { x: -1., y: -EPSILON / 10. },
                stop: Point { x: 1., y: EPSILON / 10. }
            }.is_horizontal()
        );
    }

    #[test]
    fn test_is_not_horizontal()
    {
        assert!(
            !Segment {
                start: Point { x: -1., y: -1. },
                stop: Point { x: 1., y: 1. }
            }.is_horizontal()
        );
    }

    #[test]
    fn test_is_secant_with_non_secant()
    {
        assert!(
            !Segment {
                start: Point { x: -1., y: -1. },
                stop: Point { x: 1., y: 1. }
            }.is_secant_with(
                &Segment {
                    start: Point { x: -1., y: -3. },
                    stop: Point { x: -1., y: -1. }
                }
            )
        );
    }

    #[test]
    fn test_is_secant_with_secant()
    {
        assert!(
            Segment {
                start: Point { x: -1., y: -1. },
                stop: Point { x: 1., y: 1. }
            }.is_secant_with(
                &Segment {
                    start: Point { x: 1., y: -1. },
                    stop: Point { x: -1., y: 1. }
                }
            )
        );
    }

    #[test]
    fn test_is_secant_with_self()
    {
        assert!(
            !Segment {
                start: Point { x: -1., y: -1. },
                stop: Point { x: 1., y: 1. }
            }.is_secant_with(
                &Segment {
                    start: Point { x: -1., y: -1. },
                    stop: Point { x: 1., y: 1. }
                }
            )
        );
    }

    #[test]
    fn test_length()
    {
        assert_eq!(
            Segment {
                start: Point { x: -2., y: -1. },
                stop: Point { x: 2., y: 2. }
            }.length(),
            5.
        );
    }
}
