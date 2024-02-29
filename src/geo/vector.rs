use super::{EPSILON, Point, Unit};

use derive_more::{Add, AddAssign};
use derive_more::{Sub, SubAssign};
use derive_more::{Display, Into, From, Neg, Sum};

use std::ops::{Mul, MulAssign};
use std::ops::{Div, DivAssign};

#[derive(Copy, Clone)]
#[derive(Debug)]
#[derive(Display)]
#[display(fmt = "invalid norm value: it cannot be equal to zero")]
pub struct ZeroNormError;

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

    pub fn squared_norm(&self) -> Unit { self.x.powi(2) + self.y.powi(2) }
    pub fn norm(&self) -> Unit { self.squared_norm().sqrt() }

    pub fn orthogonal(&self) -> Self { Self { x: -self.y, y: self.x } }

    pub fn unit(self) -> Result<Self, ZeroNormError>
    {
        if self.norm() == 0. {
            Err(ZeroNormError)
        } else {
            Ok(self / self.norm())
        }
    }
}

impl PartialEq for Vector {
    fn eq(&self, other: &Self) -> bool
    {
        self.is_collinear_with(other) &&
            ((*self - *other).squared_norm() < EPSILON)
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

impl Mul<Vector> for Unit {
    type Output = Vector;

    fn mul(self, rhs: Vector) -> Self::Output { rhs * self }
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

impl From<(Point, Point)> for Vector {
    fn from((a, b): (Point, Point)) -> Self
    {
        Self { x: (b.x - a.x), y: (b.y - a.y) }
    }
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn test_collinear_and_orthogonal()
    {
        let u = Vector { x: 1., y: 1.};
        let v = Vector { x: 2., y: 2.};

        assert_eq!(u.det(&v), 0.);
        assert!(u.is_collinear_with(&v));
        assert!(!u.is_orthogonal_to(&v));

        assert_eq!(u.orthogonal().dot(&v), 0.);
        assert!(!u.orthogonal().is_collinear_with(&v));
        assert!(u.orthogonal().is_orthogonal_to(&v));
    }

    #[test]
    fn test_eq_below_epsilon()
    {
        assert_eq!(
            Vector { x: 0., y: 1. },
            Vector { x: EPSILON / 10., y: 1. }
        );
    }

    #[test]
    fn test_eq_epsilon()
    {
        assert_ne!(
            Vector { x: 0., y: 1. },
            Vector { x: EPSILON, y: 1. }
        );
    }

    #[test]
    fn test_eq_above_epsilon()
    {
        assert_ne!(
            Vector { x: 0., y: 1. },
            Vector { x: EPSILON * 10., y: 1. }
        );
    }

    #[test]
    fn test_norm()
    {
        assert_eq!(Vector { x: 2., y: 0. }.norm(), 2.);
    }

    #[test]
    fn test_unit()
    {
        let diff = (Vector { x: 4., y: 2. }.unit().unwrap().norm() - 1.).abs();

        assert!(diff < EPSILON);
    }

    #[test]
    #[should_panic]
    fn test_unit_zero() { Vector::default().unit().unwrap(); }
}
