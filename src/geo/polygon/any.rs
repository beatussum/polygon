use super::{Polygon, Rectangle};

use super::super::{Container, SVG};
use super::super::{Point, Segment, Unit, Vector};

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Any { pub points: Vec<Point> }

impl Any {
    pub fn clockwise(&mut self) -> bool
    {
        if self.is_clockwise() {
            false
        } else {
            self.revert();

            true
        }
    }

    pub fn is_clockwise(&self) -> bool { self.area() < 0. }

    pub fn pairs_of_points(&self)
        -> impl Clone + Iterator<Item = (&Point, &Point)>
    {
        self
            .points()
            .zip(self.points.iter().skip(1))
            .chain(
                [(self.points.last().unwrap(), self.points.first().unwrap())]
                .into_iter()
            )
    }

    pub fn pairs_of_segments(&self)
        -> impl Clone + Iterator<Item = (Segment, Segment)> + '_
    {
        self.segments().zip(self.segments().skip(1))
    }


    pub fn points(&self) -> impl Clone + Iterator<Item = &Point>
    {
        self.points.iter()
    }

    pub fn revert(&mut self) { self.points.reverse(); }

    pub fn segments(&self) -> impl Clone + Iterator<Item = Segment> + '_
    {
        self.pairs_of_points().map(|(&x, &y)| Segment::new(x, y))
    }
}

impl Container for Any {
    #[cfg(feature = "stupid")]
    fn contains(&self, other: &Self) -> bool
    {
        self.contains(other.points.first().unwrap())
    }
}

impl Container<Point> for Any {
    #[cfg(feature = "stupid")]
    fn contains(&self, &other: &Point) -> bool
    {
        let segment = Segment::new(other, self.frame().top_right());

        let count =
            self
            .segments()
            .map(|p| p.is_secant_with(&segment) as usize)
            .sum::<usize>();

        (count % 2) == 1
    }
}

impl Polygon for Any {
    fn is_valid(&self) -> bool
    {
        if self.points.len() > 2 {
            let mut iter = self.points.iter();

            while let Some(i) = iter.next() {
                let mut iter = iter.clone();

                while let Some(j) = iter.next() {
                    if i == j {
                        return false;
                    }
                }
            }

            true
        } else {
            false
        }
    }

    fn area(&self) -> Unit
    {
        self
            .pairs_of_points()
            .map(|(&x, &y)| Vector::from(x).det(&y.into()))
            .sum::<Unit>() / 2.
    }

    fn frame(&self) -> Rectangle
    {
        let first = *self.points.first().unwrap();
        let (mut xmin, mut ymin) = first.into();
        let (mut xmax, mut ymax) = first.into();

        for &p in self.points().skip(1) {
            let (x, y) = p.into();

            if x < xmin {
                xmin = x;
            } else if x > xmax {
                xmax = x;
            }

            if y < ymin {
                ymin = y;
            } else if y > ymax {
                ymax = y;
            }
        }

        Rectangle::new((xmin, ymin).into(), (xmax, ymax).into())
    }
}

impl SVG for Any {
    fn to_svg(&self) -> String
    {
        let mut points =
            self
                .points()
                .map(|p| format!("{},{}", p.x, p.y))
                .fold(String::new(), |x, y| x + &y + " ");

        points.pop();

        format!(r#"<polygon points="{}" />"#, points)
    }
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn test_is_valid()
    {
        assert!(
            Any {
                points: vec! [
                    Point { x: -1., y: 0. },
                    Point { x: 0., y: 1. },
                    Point { x: 1., y: 0. }
                ]
            }.is_valid()
        );
    }

    #[test]
    fn test_is_not_valid_not_distinct()
    {
        assert!(
            !Any {
                points: vec! [Point::default(), Point::default()]
            }.is_valid()
        );
    }

    #[test]
    fn test_is_not_valid_not_enough()
    {
        assert!(!Any { points: vec! [Point::default()] }.is_valid());
    }

    #[test]
    fn test_area()
    {
        let testing =
            Any {
                points: vec! [
                    Point { x: -1., y: 0. },
                    Point { x: 0., y: 1. },
                    Point { x: 1., y: 0. }
                ]
            }.area();

        assert_eq!(testing, -1.)
    }

    #[test]
    fn test_frame()
    {
        let expected =
            Rectangle::new(
                Point { x: -1., y: 0. },
                Point { x: 1., y: 1. }
            );

        let testing =
            Any {
                points: vec![
                    Point { x: -1., y: 0. },
                    Point { x: 0., y: 1. },
                    Point { x: 1., y: 0. }
                ]
            }.frame();

        assert_eq!(expected, testing);
    }

    #[test]
    fn test_is_clockwise()
    {
        assert!(
            Any {
                points: vec![
                    Point { x: -1., y: 0. },
                    Point { x: 0., y: 1. },
                    Point { x: 1., y: 0. }
                ]
            }.is_clockwise()
        );
    }

    #[test]
    fn test_is_not_clockwise()
    {
        assert!(
            !Any {
                points: vec![
                    Point { x: 1., y: 0. },
                    Point { x: 0., y: 1. },
                    Point { x: -1., y: 0. }
                ]
            }.is_clockwise()
        );
    }

    #[test]
    fn test_contains_inside()
    {
        let point = Point::default();

        let poly =
            Any {
                points: vec![
                    Point { x: -1., y: 0. },
                    Point { x: 0., y: 1. },
                    Point { x: 1., y: 0. }
                ]
            };

        assert!(poly.contains(&point));
    }

    #[test]
    fn test_contains_on()
    {
        let point = Point { x: -1., y: 0. };

        let poly =
            Any {
                points: vec![
                    Point { x: -1., y: 0. },
                    Point { x: 0., y: 1. },
                    Point { x: 1., y: 0. }
                ]
            };

        assert!(poly.contains(&point));
    }

    #[test]
    fn test_contains_outside()
    {
        let point = Point { x: 10., y: 10. };

        let poly =
            Any {
                points: vec![
                    Point { x: -1., y: 0. },
                    Point { x: 0., y: 1. },
                    Point { x: 1., y: 0. }
                ]
            };

        assert!(!poly.contains(&point));
    }

    #[test]
    fn test_get_svg()
    {
        let testing =
            Any {
                points: vec![
                    Point { x: -1., y: 0. },
                    Point { x: 0., y: 1. },
                    Point { x: 1., y: 0. }
                ]
            }.to_svg();

        assert_eq!(testing, r#"<polygon points="-1,0 0,1 1,0" />"#);
    }
}
