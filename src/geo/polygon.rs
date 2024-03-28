use super::SVG;
use super::Unit;

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
