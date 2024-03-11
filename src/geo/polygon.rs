use super::SVG;
use super::Unit;

mod any;
pub use any::Any;

mod rectangle;
pub use rectangle::Rectangle;

pub trait Polygon: SVG
{
    fn area(&self) -> Unit;
    fn is_valid(&self) -> bool;
    fn frame(&self) -> Rectangle;
}
