use super::{Any, Polygon};

use super::super::{Container, SVG};
use super::super::{Point, Unit};

/**************/
/* STRUCTURES */
/**************/

#[derive(Clone, Copy)]
#[derive(Debug, Default, PartialEq)]
pub struct Rectangle { bottom_left: Point, top_right: Point }

/*******************/
/* IMPLEMENTATIONS */
/*******************/

impl Rectangle {
    /****************/
    /* CONSTRUCTORS */
    /****************/

    pub fn new(bottom_left: Point, top_right: Point) -> Self
    {
        assert_ne!(bottom_left, top_right);

        Self { bottom_left, top_right }
    }

    pub fn square(bottom_left: Point, side: Unit) -> Self
    {
        let (x, y) = bottom_left.into();

        Self { bottom_left, top_right: Point { x: x + side, y: y + side } }
    }

    /***********/
    /* ACTIONS */
    /***********/

    pub fn divide_horizontally(&self) -> (Self, Self)
    {
        let h = self.height() / 2.;
        let (x, y) = self.top_right.into();
        let top = Rectangle::new(self.bottom_left, Point { x, y: y - h });
        let (x, y) = self.bottom_left.into();
        let bottom = Rectangle::new(Point { x, y: y + h }, self.top_right);

        (top, bottom)
    }

    pub fn divide_vertically(&self) -> (Self, Self)
    {
        let w = self.width() / 2.;
        let (x, y) = self.top_right.into();
        let left = Rectangle::new(self.bottom_left, Point { x: x - w, y });
        let (x, y) = self.bottom_left.into();
        let right = Rectangle::new(Point { x: x + w, y }, self.top_right);

        (left, right)
    }

    /***********/
    /* GETTERS */
    /***********/

    pub fn bottom_left(&self) -> Point { self.bottom_left }
    pub fn is_square(&self) -> bool { self.height() == self.width() }
    pub fn height(&self) -> Unit { self.top_right.y - self.bottom_left.y }

    pub fn polygon(&self) -> Any
    {
        let (xmin, ymin) = self.bottom_left.into();
        let (xmax, ymax) = self.top_right.into();

        Any {
            points: vec! [
                self.bottom_left,
                (xmax, ymin).into(),
                self.top_right,
                (xmin, ymax).into()
            ]
        }
    }

    pub fn top_right(&self) -> Point { self.top_right }
    pub fn width(&self) -> Unit { self.top_right.x - self.bottom_left.x }
}

impl Container for Rectangle {
    fn contains(&self, other: &Self) -> bool
    {
        let (xmin, ymin) = self.bottom_left.into();
        let (xmax, ymax) = self.top_right.into();

        let (amin, bmin) = other.bottom_left.into();
        let (amax, bmax) = other.top_right.into();

        (xmin <= amin) && (ymin <= bmin) && (xmax >= amax) && (ymax >= bmax)
    }
}

impl Polygon for Rectangle {
    fn len(&self) -> usize { 4 }
    fn area(&self) -> Unit { self.height() * self.width() }
    fn frame(&self) -> Rectangle { *self }
    fn is_valid(&self) -> bool { true }
}

impl SVG for Rectangle {
    fn to_svg(&self) -> String
    {
        let (xmin, ymin) = self.bottom_left.into();
        let (xmax, ymax) = self.top_right.into();

        format!(
            r#"<rect x="{}" y="{}" width="{}" height="{}" />"#,
            xmin,
            ymax,
            xmax - xmin,
            ymax - ymin
        )
    }
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn test_to_svg()
    {
        let expected = r#"<rect x="-1" y="1" width="3" height="2" />"#;

        let testing =
            Rectangle {
                bottom_left: Point { x: -1., y: -1. },
                top_right: Point { x: 2., y: 1. }
            };

        assert_eq!(testing.to_svg(), expected);
    }
}
