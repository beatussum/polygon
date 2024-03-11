use super::{Container, SVG};
use super::{Point, Polygon};

#[derive(Clone, Copy)]
#[derive(Debug, Default, PartialEq)]
pub struct Rectangle { pub bottom_left: Point, pub top_right: Point }

impl Rectangle {
    pub fn is_square(&self) -> bool
    {
        let (xmin, ymin) = self.bottom_left.into();
        let (xmax, ymax) = self.top_right.into();

        (xmax - xmin) == (ymax - ymin)
    }

    pub fn polygon(&self) -> Polygon
    {
        let (xmin, ymin) = self.bottom_left.into();
        let (xmax, ymax) = self.top_right.into();

        Polygon {
            points: vec! [
                self.bottom_left,
                (xmax, ymin).into(),
                self.top_right,
                (xmin, ymax).into()
            ]
        }
    }
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
            }.to_svg();

        assert_eq!(testing, expected);
    }
}
