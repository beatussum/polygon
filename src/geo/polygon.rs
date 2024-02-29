use super::{Point, Segment, Unit, Vector};

#[derive(Clone)]
#[derive(Debug, Default)]
pub struct Polygon { pub points: Vec<Point> }

impl Polygon {
    pub fn square(bottom_left: Point, side: Unit) -> Self
    {
        assert_ne!(side, 0.);

        let (a, b) = bottom_left.into();

        Self {
            points: vec![
                bottom_left,
                Point { x: a + side, y: b },
                Point { x: a + side, y: b + side },
                Point { x: a, y: b + side }
            ]
        }
    }

    pub fn is_valid(&self) -> bool
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

    pub fn area(&self) -> Unit
    {
        self
            .pairs_of_points()
            .map(|(&x, &y)| Vector::from(x).det(&y.into()))
            .sum::<Unit>() / 2.
    }

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

    pub fn segments(&self) -> impl Clone + Iterator<Item = Segment> + '_
    {
        self.pairs_of_points().map(|(&x, &y)| Segment::new(x, y))
    }

    pub fn revert(&mut self) { self.points.reverse(); }
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn test_is_valid()
    {
        assert!(Polygon::square(Point::default(), 2.).is_valid());
    }

    #[test]
    fn test_is_not_valid_not_distinct()
    {
        assert!(
            !Polygon {
                points: vec![Point::default(), Point::default()]
            }.is_valid()
        );
    }

    #[test]
    fn test_is_not_valid_not_enough()
    {
        assert!(!Polygon { points: vec![Point::default()] }.is_valid());
    }

    #[test]
    fn test_area()
    {
        assert_eq!(Polygon::square(Point::default(), 2.).area(), 4.)
    }

    #[test]
    fn test_is_clockwise()
    {
        assert!(
            Polygon {
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
        assert!(!Polygon::square(Point::default(), 2.).is_clockwise());
    }
}
