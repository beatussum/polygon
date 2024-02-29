use super::{are_ccw, EPSILON, Point, Unit, Vector};
use derive_more::{Display, Into, From};

#[derive(Eq, PartialEq)]
#[derive(Copy, Clone)]
#[derive(From, Into)]
#[derive(Debug, Default)]
#[derive(Display)]
#[display(fmt = "[{} ; {}]", start, stop)]
pub struct Segment { start: Point, stop: Point }

impl Segment {
    pub fn new(start: Point, stop: Point) -> Self
    {
        assert_ne!(start, stop);

        Self { start, stop }
    }

    pub fn contains(&self, point: &Point) -> bool
    {
        let dist =
            self.start.distance_from(point) +
            self.stop.distance_from(point);

        (dist - self.length()).abs() < EPSILON
    }

    pub fn distance_from_point(&self, other: &Point) -> Unit
    {
        if self.contains(other) {
            0.
        } else {
            let projection =
                Vector::from((self.start, *other))
                .dot(&(*self).into());

            let oh =
                Vector::from((Point::default(), self.start)) +
                (projection / Vector::from(*self).squared_norm()) *
                Vector::from(*self);

            if self.contains(&oh.into()) {
                (oh - (*other).into()).norm()
            } else if projection < 0. {
                self.start.distance_from(other)
            } else {
                self.stop.distance_from(other)
            }
        }
    }

    pub fn distance_from_segment(&self, other: &Self) -> Unit
    {
        fn dist(a: &Segment, b: &Segment) -> Unit
        {
            std::cmp::min_by(
                a.distance_from_point(&b.start),
                a.distance_from_point(&b.stop),
                Unit::total_cmp
            )
        }

        if self.is_secant_with(other) {
            0.
        } else {
            std::cmp::min_by(
                dist(self, other),
                dist(other, self),
                Unit::total_cmp
            )
        }
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
        if self == rhs {
            true
        } else {
            let (a, b) = (*self).into();
            let (c, d) = (*rhs).into();

            (are_ccw(&a, &c, &d) != are_ccw(&b, &c, &d)) &&
                (are_ccw(&a, &b, &c) != are_ccw(&a, &b, &d))
        }
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
            Segment::new(
                Point { x: -1., y: -1. },
                Point { x: 1., y: 1. }
            ).contains(&Point::default())
        );
    }

    #[test]
    fn test_not_contains()
    {
        assert!(
            !Segment::new(
                Point { x: -1., y: -1. },
                Point { x: 1., y: 1. }
            ).contains(&Point { x: 2., y: 2. })
        );
    }

    #[test]
    fn test_distance_from_point_contained()
    {
        assert_eq!(
            Segment::new(
                Point { x: -1., y: -1. },
                Point { x: 1., y: 1. }
            ).distance_from_point(&Point::default()),
            0.
        );
    }

    #[test]
    fn test_distance_from_point_in()
    {
        assert_eq!(
            Segment::new(
                Point { x: -1., y: -1. },
                Point { x: 1., y: 1. }
            ).distance_from_point(&Point { x: -1., y: 1. }),
            2_f32.sqrt()
        );
    }

    #[test]
    fn test_distance_from_point_out()
    {
        assert_eq!(
            Segment::new(
                Point::default(),
                Point { x: 1., y: 1. }
            ).distance_from_point(&Point { x: -1., y: 0. }),
            1.
        );
    }

    #[test]
    fn test_distance_from_segment_non_secant()
    {
        assert_eq!(
            Segment::new(
                Point::default(),
                Point { x: 1., y: 1. }
            ).distance_from_segment(
                &Segment::new(
                    Point { x: 4., y: 5. },
                    Point { x: 3., y: 7. }
                )
            ),
            5.
        );
    }

    #[test]
    fn test_distance_from_segment_secant()
    {
        assert_eq!(
            Segment::new(
                Point { x: -1., y: -1. },
                Point { x: 1., y: 1. }
            ).distance_from_segment(
                &Segment::new(
                    Point { x: -1., y: 1. },
                    Point { x: 1., y: -1. }
                )
            ),
            0.
        );
    }

    #[test]
    fn test_distance_from_segment_self()
    {
        let u =
            Segment::new(
                Point { x: -1., y: -1. },
                Point { x: 1., y: 1. }
            );

        assert_eq!(u.distance_from_segment(&u), 0.);
    }

    #[test]
    fn test_is_vertical()
    {
        assert!(
            Segment::new(
                Point { x: -EPSILON / 10., y: -1. },
                Point { x: EPSILON / 10., y: 1. }
            ).is_vertical()
        );
    }

    #[test]
    fn test_is_not_vertical()
    {
        assert!(
            !Segment::new(
                Point { x: -1., y: -1. },
                Point { x: 1., y: 1. }
            ).is_vertical()
        );
    }

    #[test]
    fn test_is_horizontal()
    {
        assert!(
            Segment::new(
                Point { x: -1., y: -EPSILON / 10. },
                Point { x: 1., y: EPSILON / 10. }
            ).is_horizontal()
        );
    }

    #[test]
    fn test_is_not_horizontal()
    {
        assert!(
            !Segment::new(
                Point { x: -1., y: -1. },
                Point { x: 1., y: 1. }
            ).is_horizontal()
        );
    }

    #[test]
    fn test_is_secant_with_non_secant()
    {
        assert!(
            !Segment::new(
                Point { x: -1., y: -1. },
                Point { x: 1., y: 1. }
            ).is_secant_with(
                &Segment::new(
                    Point { x: -1., y: -3. },
                    Point { x: -1., y: -1. }
                )
            )
        );
    }

    #[test]
    fn test_is_secant_with_secant()
    {
        assert!(
            Segment::new(
                Point { x: -1., y: -1. },
                Point { x: 1., y: 1. }
            ).is_secant_with(
                &Segment::new(
                    Point { x: 1., y: -1. },
                    Point { x: -1., y: 1. }
                )
            )
        );
    }

    #[test]
    fn test_is_secant_with_self()
    {
        assert!(
            Segment::new(
                Point { x: -1., y: -1. },
                Point { x: 1., y: 1. }
            ).is_secant_with(
                &Segment::new(
                    Point { x: -1., y: -1. },
                    Point { x: 1., y: 1. }
                )
            )
        );
    }

    #[test]
    fn test_length()
    {
        assert_eq!(
            Segment::new(
                Point { x: -2., y: -1. },
                Point { x: 2., y: 2. }
            ).length(),
            5.
        );
    }
}
