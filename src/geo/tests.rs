mod point
{
    use super::super::*;

    #[test]
    pub fn test_are_ccw()
    {
        let a = Point { x: 0., y: -1. };
        let b = Point { x: 1., y: 0. };
        let c = Point { x: 0., y: 1. };

        assert!(Point::are_ccw(&a, &b, &c));
        assert!(Point::are_ccw(&b, &c, &a));
        assert!(Point::are_ccw(&c, &a, &b));
    }

    #[test]
    pub fn test_are_not_ccw()
    {
        let a = Point { x: 0., y: -1. };
        let b = Point { x: 1., y: 0. };
        let c = Point { x: 0., y: 1. };

        assert!(!Point::are_ccw(&c, &b, &a));
        assert!(!Point::are_ccw(&a, &c, &b));
        assert!(!Point::are_ccw(&b, &a, &c));
    }

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

mod vector
{
    use super::super::*;

    #[test]
    fn test_collinear_and_orthogonal()
    {
        let u = Vector { x: 1., y: 1.};
        let v = Vector { x: 2., y: 2.};

        assert_eq!(u.det(&v), 0.);
        assert!(u.is_collinear_with(&v));
        assert!(!u.is_orthogonal_to(&v));

        assert_eq!(u.orthogonal().dot(&v), 0.);
        assert!(!u.orthogonal().is_collinear_with(&v));
        assert!(u.orthogonal().is_orthogonal_to(&v));
    }

    #[test]
    fn test_eq_below_epsilon()
    {
        assert_eq!(
            Vector { x: 0., y: 1. },
            Vector { x: EPSILON / 10., y: 1. }
        );
    }

    #[test]
    fn test_eq_epsilon()
    {
        assert_ne!(
            Vector { x: 0., y: 1. },
            Vector { x: EPSILON, y: 1. }
        );
    }

    #[test]
    fn test_eq_above_epsilon()
    {
        assert_ne!(
            Vector { x: 0., y: 1. },
            Vector { x: EPSILON * 10., y: 1. }
        );
    }

    #[test]
    fn test_norm()
    {
        assert_eq!(Vector { x: 2., y: 0. }.norm(), 2.);
    }

    #[test]
    fn test_unit()
    {
        let norm = (Vector { x: 4., y: 2. }.unit().unwrap().norm() - 1.).abs();

        assert!(norm < EPSILON);
    }

    #[test]
    #[should_panic]
    fn test_unit_zero() { Vector::default().unit().unwrap(); }
}

mod segment
{
    use super::super::*;

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
