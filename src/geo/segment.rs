use super::{Container, Distance, Intersecter, SVG};
use super::{Point, Unit, Vector};
use super::are_ccw;

use derive_more::{Display, Into};
use symm_impl::symmetric;

/**************/
/* STRUCTURES */
/**************/

#[derive(Copy, Clone)]
#[derive(Debug, Default, Into, PartialEq)]
#[derive(Display)]
#[display(fmt = "[{} ; {}]", start, stop)]
pub struct Segment { start: Point, stop: Point }

/*******************/
/* IMPLEMENTATIONS */
/*******************/

impl Segment {
    /****************/
    /* CONSTRUCTORS */
    /****************/

    pub fn new(start: Point, stop: Point) -> Self
    {
        assert_ne!(start, stop);

        Self { start, stop }
    }

    /*************/
    /* OPERATORS */
    /*************/

    pub fn length(&self) -> Unit { self.start.distance_from(&self.stop) }
}

impl Intersecter for Segment {
    fn intersects(&self, other: &Self) -> bool
    {
        let (a, b) = (*self).into();
        let (c, d) = (*other).into();

        ((a == c) || (a == d) || (b == c) || (b == d)) ||
            ((are_ccw(&a, &c, &d) != are_ccw(&b, &c, &d)) &&
            (are_ccw(&a, &b, &c) != are_ccw(&a, &b, &d)))
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

/***************/
/* `Container` */
/***************/

impl Container for Segment {
    fn contains(&self, other: &Self) -> bool { self == other }
}

impl Container<Point> for Segment {
    fn contains(&self, point: &Point) -> bool
    {
        let dist =
            self.start.distance_from(point) +
            self.stop.distance_from(point);

        (dist - self.length()).abs() < Unit::EPSILON
    }
}

/**************/
/* `Distance` */
/**************/

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

        if self.intersects(other) {
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

    /***********/
    /* GETTERS */
    /***********/

    #[test]
    fn test_length()
    {
        let testing =
            Segment::new(
                Point { x: -2., y: -1. },
                Point { x: 2., y: 2. }
            );

        assert_eq!(testing.length(), 5.);
    }

    /***************/
    /* `Container` */
    /***************/

    #[test]
    fn test_contains()
    {
        let point = Point::default();

        let segment =
            Segment::new(
                Point { x: -1., y: -1. },
                Point { x: 1., y: 1. }
            );

        assert!(segment.contains(&point));
    }

    #[test]
    fn test_not_contains()
    {
        let point = Point { x: 2., y: 2. };

        let segment =
            Segment::new(
                Point { x: -1., y: -1. },
                Point { x: 1., y: 1. }
            );

        assert!(!segment.contains(&point));
    }

    /**************/
    /* `Distance` */
    /**************/

    #[test]
    fn test_distance_from_point_contained()
    {
        let point = Point::default();

        let segment =
            Segment::new(
                Point { x: -1., y: -1. },
                Point { x: 1., y: 1. }
            );

        assert_eq!(segment.distance_from(&point), 0.);
    }

    #[test]
    fn test_distance_from_point_in()
    {
        let point = Point { x: -1., y: 1. };

        let segment =
            Segment::new(
                Point { x: -1., y: -1. },
                Point { x: 1., y: 1. }
            );

        assert_eq!(segment.distance_from(&point), Unit::from(2.).sqrt());
    }

    #[test]
    fn test_distance_from_point_out()
    {
        let point = Point { x: -1., y: 0. };

        let segment =
            Segment::new(
                Point::default(),
                Point { x: 1., y: 1. }
            );

        assert_eq!(segment.distance_from(&point), 1.);
    }

    #[test]
    fn test_distance_from_segment_non_secant()
    {
        let a =
            Segment::new(
                Point::default(),
                Point { x: 1., y: 1. }
            );

        let b =
            Segment::new(
                Point { x: 4., y: 5. },
                Point { x: 3., y: 7. }
            );

        assert_eq!(a.distance_from(&b), 5.);
    }

    #[test]
    fn test_distance_from_segment_secant()
    {
        let a =
            Segment::new(
                Point { x: -1., y: -1. },
                Point { x: 1., y: 1. }
            );

        let b =
            Segment::new(
                Point { x: -1., y: 1. },
                Point { x: 1., y: -1. }
            );

        assert_eq!(a.distance_from(&b), 0.);
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

    /*****************/
    /* `Intersecter` */
    /*****************/

    #[test]
    fn test_intersects_with_orthogonal()
    {
        let a = Segment::new(Point { x: 0., y: 1. }, Point { x: 0., y: 0. });
        let b = Segment::new(Point { x: 0., y: 0. }, Point { x: 1., y: 0. });

        assert!(a.intersects(&b));
    }

    #[test]
    fn test_intersects_orthogonal_reversed()
    {
        let a = Segment::new(Point { x: 0., y: 1. }, Point { x: 0., y: 0. });
        let b = Segment::new(Point { x: 0., y: 0. }, Point { x: -1., y: 0. });

        assert!(a.intersects(&b));
    }

    #[test]
    fn test_intersects_non_secant()
    {
        let a = Segment::new(Point { x: -1., y: -1. }, Point { x: 1., y: 1. });

        let b =
            Segment::new(
                Point { x: -1., y: -3. },
                Point { x: -1., y: -1. }
            );

        assert!(a.intersects(&b));
    }

    #[test]
    fn test_intersects_with_secant()
    {
        let a =
            Segment::new(
                Point { x: -1., y: -1. },
                Point { x: 1., y: 1. }
            );

        let b =
            Segment::new(
                Point { x: 1., y: -1. },
                Point { x: -1., y: 1. }
            );

        assert!(a.intersects(&b));
    }
}
