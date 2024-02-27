extern crate derive_more;

use derive_more::{Add, AddAssign};
use derive_more::{Sub, SubAssign};
use derive_more::{Display, Into, From, Neg, Sum};

use std::ops::{Mul, MulAssign};
use std::ops::{Div, DivAssign};

const EPSILON: f32 = 1e-5;
type Unit = f32;

#[derive(Copy, Clone)]
#[derive(From, Into)]
#[derive(Debug, Default)]
#[derive(Display)]
#[display(fmt = "({} ; {})", x, y)]
pub struct Point { pub x: Unit, pub y: Unit }

impl Point {
    fn are_ccw(a: &Self, b: &Self, c: &Self) -> bool
    {
        Vector::from((a, b)).det(&(a, c).into()) > 0.
    }

    pub fn distance_from(&self, rhs: &Self) -> Unit
    {
        ((self.x - rhs.x).powf(2.) + (self.y - rhs.y).powf(2.)).sqrt()
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool { self.distance_from(other) < EPSILON }
}

impl Eq for Point {}

#[derive(Add, AddAssign)]
#[derive(Sub, SubAssign)]
#[derive(Copy, Clone)]
#[derive(From, Into)]
#[derive(Debug, Default, Neg, Sum)]
#[derive(Display)]
#[display(fmt = "({} ; {})", x, y)]
pub struct Vector { pub x: Unit, pub y: Unit }

impl Vector {
    pub fn det(self, rhs: &Self) -> Unit { self.x * rhs.y - self.y * rhs.x }

    pub fn dot(&self, rhs: &Self) -> Unit
    {
        (self.x * rhs.x) + (self.y * rhs.y)
    }

    pub fn is_collinear_with(&self, rhs: &Self) -> bool
    {
        self.det(rhs).abs() < EPSILON
    }

    pub fn is_orthogonal_to(&self, rhs: &Self) -> bool
    {
        self.dot(rhs).abs() < EPSILON
    }

    pub fn norm(&self) -> Unit { (self.x.powf(2.) + self.y.powf(2.)).sqrt() }
    pub fn orthogonal(&self) -> Self { Self { x: -self.y, y: self.x } }
    pub fn unit(self) -> Self { self / self.norm() }
}

impl PartialEq for Vector {
    fn eq(&self, other: &Self) -> bool
    {
        self.is_collinear_with(other) && ((*self - *other).norm() < EPSILON)
    }
}

impl Eq for Vector {}

impl Mul<Unit> for Vector {
    type Output = Vector;

    fn mul(self, rhs: Unit) -> Self::Output
    {
        Self { x: self.x * rhs, y: self.y * rhs }
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
        Self { x: self.x / rhs, y: self.y / rhs }
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
        Self { x: (b.x - a.x), y: (b.y - a.y) }
    }
}

impl From<&Segment> for Vector {
    fn from(value: &Segment) -> Self { (&value.start, &value.stop).into() }
}

#[derive(Eq, PartialEq)]
#[derive(Copy, Clone)]
#[derive(From, Into)]
#[derive(Debug, Default)]
#[derive(Display)]
#[display(fmt = "[{} ; {}]", start, stop)]
pub struct Segment { pub start: Point, pub stop: Point }

impl Segment {
    pub fn contains(&self, point: &Point) -> bool
    {
        let dist =
            self.start.distance_from(point) +
            self.stop.distance_from(point);

        (dist - self.length()).abs() < EPSILON
    }

    pub fn distance_from(&self, rhs: &Self) -> Unit
    {
        Vector::from(self).orthogonal().dot(&rhs.into()).abs()
    }

    pub fn is_horizontal(&self) -> bool
    {
        (self.start.y - self.stop.y).abs() < EPSILON
    }

    pub fn is_vertical(&self) -> bool
    {
        (self.start.x - self.stop.x).abs() < EPSILON
    }

    pub fn is_secant_with(&self, rhs: &Self) -> bool
    {
        let (a, b) = (*self).into();
        let (c, d) = (*rhs).into();

        (Point::are_ccw(&a, &c, &d) != Point::are_ccw(&b, &c, &d)) &&
            (Point::are_ccw(&a, &b, &c) != Point::are_ccw(&a, &b, &d))
    }

    pub fn length(&self) -> Unit { self.start.distance_from(&self.stop) }
}
