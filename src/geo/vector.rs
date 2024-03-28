use super::{Point, Segment, Unit};

use derive_more::{Add, AddAssign};
use derive_more::{Sub, SubAssign};
use derive_more::{Display, Into, From, Neg, Sum};

use symm_impl::symmetric;

use std::error::Error;
use std::hash::{Hash, Hasher};
use std::ops::{Mul, MulAssign};
use std::ops::{Div, DivAssign};

/**************/
/* STRUCTURES */
/**************/

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

/*******************/
/* IMPLEMENTATIONS */
/*******************/

impl Vector {
    /*************/
    /* OPERATORS */
    /*************/

    pub fn det(self, rhs: &Self) -> Unit { self.x * rhs.y - self.y * rhs.x }

    pub fn dot(&self, rhs: &Self) -> Unit
    {
        (self.x * rhs.x) + (self.y * rhs.y)
    }

    pub fn is_collinear_with(&self, rhs: &Self) -> bool
    {
        self.det(rhs).abs() < Unit::EPSILON
    }

    pub fn is_orthogonal_to(&self, rhs: &Self) -> bool
    {
        self.dot(rhs).abs() < Unit::EPSILON
    }

    /***********/
    /* GETTERS */
    /***********/

    pub fn is_horizontal(&self) -> bool { self.y.abs() < Unit::EPSILON }
    pub fn is_vertical(&self) -> bool { self.x.abs() < Unit::EPSILON }

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

impl Hash for Vector {
    fn hash<H: Hasher>(&self, state: &mut H)
    {
        self.x.to_bits().hash(state);
        self.y.to_bits().hash(state);
    }
}

impl Error for ZeroNormError {}

/*************/
/* OPERATORS */
/*************/

impl PartialEq for Vector {
    fn eq(&self, other: &Self) -> bool
    {
        self.is_collinear_with(other) &&
            ((*self - *other).squared_norm() < Unit::EPSILON)
    }
}

#[symmetric]
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

/***************/
/* CONVERSIONS */
/***************/

impl From<Point> for Vector {
    fn from(value: Point) -> Self { Self { x: value.x, y: value.y } }
}

impl From<(Point, Point)> for Vector {
    fn from((a, b): (Point, Point)) -> Self
    {
        Self { x: (b.x - a.x), y: (b.y - a.y) }
    }
}

impl From<Segment> for Vector {
    fn from(value: Segment) -> Self { <(Point, Point)>::from(value).into() }
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn test_det()
    {
        let u = Vector { x: 1., y: 0.};
        let v = Vector { x: 1., y: 1.};

        assert!(u.det(&v) > 0.);
        assert_eq!(u.det(&v), -v.det(&u));
    }

    #[test]
    fn test_dot_negative()
    {
        let u = Vector { x: 1., y: 0.};
        let v = Vector { x: -1., y: 1.};

        assert_eq!(u.dot(&v), -1.);
        assert_eq!(u.dot(&v), v.dot(&u));
    }

    #[test]
    fn test_dot_positive()
    {
        let u = Vector { x: 1., y: 0.};
        let v = Vector { x: 1., y: 1.};

        assert_eq!(u.dot(&v), 1.);
        assert_eq!(u.dot(&v), v.dot(&u));
    }

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
    fn test_is_vertical()
    {
        let testing = Vector { x: 0., y: 1. };

        assert!(testing.is_vertical());
    }

    #[test]
    fn test_is_not_vertical()
    {
        let testing = Vector { x: 1., y: 0. };

        assert!(!testing.is_vertical());
    }

    #[test]
    fn test_is_horizontal()
    {
        let testing = Vector { x: 1., y: 0. };

        assert!(testing.is_horizontal());
    }

    #[test]
    fn test_is_not_horizontal()
    {
        let testing = Vector { x: 1., y: 1. };

        assert!(!testing.is_horizontal());
    }

    #[test]
    fn test_norm()
    {
        let testing = Vector { x: 2., y: 0. };

        assert_eq!(testing.norm(), 2.);
    }

    #[test]
    fn test_unit()
    {
        let testing = Vector { x: 4., y: 2. };
        let testing = (testing.unit().unwrap().norm() - 1.).abs();

        assert!(testing < Unit::EPSILON);
    }

    #[test]
    #[should_panic]
    fn test_unit_zero() { Vector::default().unit().unwrap(); }

    #[test]
    fn test_eq_below_epsilon()
    {
        let a = Vector { x: 0., y: 1. };
        let b = Vector { x: Unit::EPSILON / 10., y: 1. };

        assert_eq!(a, b);
    }

    #[test]
    fn test_eq_epsilon()
    {
        let a = Vector { x: 0., y: 1. };
        let b = Vector { x: Unit::EPSILON, y: 1. };

        assert_ne!(a, b);
    }

    #[test]
    fn test_eq_above_epsilon()
    {
        let a = Vector { x: 0., y: 1. };
        let b = Vector { x: Unit::EPSILON * 10., y: 1. };

        assert_ne!(a, b);
    }
}
