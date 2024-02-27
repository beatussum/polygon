extern crate derive_more;

use derive_more::{Add, AddAssign};
use derive_more::{Sub, SubAssign};
use derive_more::{Display, From, Neg, Sum};

use std::ops::{Mul, MulAssign};
use std::ops::{Div, DivAssign};

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

#[derive(Add, AddAssign)]
#[derive(Sub, SubAssign)]
#[derive(Copy, Clone)]
#[derive(Debug, Default, From, Neg, Sum)]
#[derive(Display)]
#[display(fmt = "({} ; {})", x, y)]
pub struct Vector { pub x: Unit, pub y: Unit }

impl Vector {
    pub fn det(self, rhs: &Self) -> Unit { self.x * rhs.y - self.y * rhs.x }

    pub fn dot(&self, rhs: &Self) -> Unit
    {
        (self.x * rhs.x) + (self.y * rhs.y)
    }

    pub fn norm(&self) -> Unit { (self.x.powf(2.) + self.y.powf(2.)).sqrt() }
    pub fn unit(self) -> Self { self / self.norm() }
}

impl Mul<Unit> for Vector {
    type Output = Vector;

    fn mul(self, rhs: Unit) -> Self::Output
    {
        Self{ x: self.x * rhs, y: self.y * rhs }
    }
}

impl MulAssign<Unit> for Vector {
    fn mul_assign(&mut self, rhs: Unit)
    {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl Div<Unit> for Vector {
    type Output = Vector;

    fn div(self, rhs: Unit) -> Self::Output
    {
        Self{ x: self.x / rhs, y: self.y / rhs }
    }
}

impl DivAssign<Unit> for Vector {
    fn div_assign(&mut self, rhs: Unit)
    {
        self.x /= rhs;
        self.y /= rhs;
    }
}

impl From<(&Point, &Point)> for Vector {
    fn from((a, b): (&Point, &Point)) -> Self
    {
        Vector{ x: (b.x - a.x), y: (b.y - a.y) }
    }
}
