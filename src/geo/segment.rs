use super::{Distance, Point, SVG, Unit};
use super::are_ccw;
use super::EPSILON;

use derive_more::{Display, Into};

#[derive(Eq, PartialEq)]
#[derive(Copy, Clone)]
#[derive(Debug, Default, Into)]
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

    pub fn is_strictly_secant_with(&self, rhs: &Self) -> bool
    {
        let (a, b) = (*self).into();
        let (c, d) = (*rhs).into();

        (are_ccw(&a, &c, &d) != are_ccw(&b, &c, &d)) &&
            (are_ccw(&a, &b, &c) != are_ccw(&a, &b, &d))
    }

    pub fn is_secant_with(&self, rhs: &Self) -> bool
    {
        (self == rhs) || self.is_strictly_secant_with(rhs)
    }

    pub fn length(&self) -> Unit { self.start.distance_from(&self.stop) }
}

impl Distance for Segment {
    fn squared_distance_from(&self, other: &Self) -> Unit
    {
        fn dist(a: &Segment, b: &Segment) -> Unit
        {
            std::cmp::min_by(
                a.squared_distance_from(&b.start),
                a.squared_distance_from(&b.stop),
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
}

impl SVG for Segment {
    fn to_svg(&self) -> String
    {
        format!(
            r#"<line x1="{}" y1="{}" x2="{}" y2="{}""#,
            self.start.x,
            self.start.y,
            self.stop.x,
            self.stop.y
        )
    }
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
            ).distance_from(&Point::default()),
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
            ).distance_from(&Point { x: -1., y: 1. }),
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
            ).distance_from(&Point { x: -1., y: 0. }),
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
            ).distance_from(
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
            ).distance_from(
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

        assert_eq!(u.distance_from(&u), 0.);
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
        let u = Segment::new(Point { x: -1., y: -1. },Point { x: 1., y: 1. });

        assert!(u.is_secant_with(&u));
        assert!(!u.is_strictly_secant_with(&u));
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
