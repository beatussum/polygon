extern crate derive_more;
use derive_more::{Display, From};

type Unit = f32;

#[derive(Copy, Clone)]
#[derive(Debug, Default, From)]
#[derive(Display)]
#[display(fmt = "({} ; {})", x, y)]
pub struct Point { pub x: Unit, pub y: Unit }

impl Point {
    pub fn distance_from(&self, rhs: &Self) -> Unit
    {
        ((self.x - rhs.x).powf(2.) + (self.y - rhs.y).powf(2.)).sqrt()
    }
}
