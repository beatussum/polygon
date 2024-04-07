use super::SVG;
use super::{Point, Unit};

/***********/
/* MODULES */
/***********/

mod any;
pub use any::Any;

mod rectangle;
pub use rectangle::Rectangle;

/**********/
/* TRAITS */
/**********/

pub trait Polygon: SVG
{
    fn area(&self) -> Unit;
    fn len(&self) -> usize;
    fn is_valid(&self) -> bool;
    fn frame(&self) -> Rectangle;
}

/*************/
/* FUNCTIONS */
/*************/

pub fn frame_of<T>(mut iter: T) -> Rectangle where T: Iterator<Item = Point>
{
    let first = iter.next().unwrap();

    let (mut xmin, mut ymin) = first.into();
    let (mut xmax, mut ymax) = first.into();

    for p in iter {
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
