use std::ops::*;

/// An angle, in radians.
///
/// This type is marked as `#[repr(C)]`.
#[repr(C)]
#[derive(Copy, Clone, PartialEq, PartialOrd, Debug)]
pub struct Rad(pub f32);

/// An angle, in degrees.
///
/// This type is marked as `#[repr(C)]`.
#[repr(C)]
#[derive(Copy, Clone, PartialEq, PartialOrd, Debug)]
pub struct Deg(pub f32);

impl From<Rad> for Deg {
    #[inline]
    fn from(rad: Rad) -> Deg {
        Deg(rad.0 * 180.0 / std::f32::consts::PI)
    }
}

impl From<Deg> for Rad {
    #[inline]
    fn from(deg: Deg) -> Rad {
        Rad(deg.0 * std::f32::consts::PI / 180.0)
    }
}

impl Rad {
    #[inline]
    pub fn full_turn() -> Rad {
        Rad(std::f32::consts::PI * 2.0)
    }

    #[inline]
    pub fn sin(self) -> f32 {
        Rad::from(self).0.sin()
    }
    #[inline]
    pub fn cos(self) -> f32 {
        Rad::from(self).0.cos()
    }
    #[inline]
    pub fn tan(self) -> f32 {
        Rad::from(self).0.tan()
    }
    #[inline]
    pub fn sin_cos(self) -> (f32, f32) {
        Rad::from(self).0.sin_cos()
    }

    #[inline]
    pub fn asin(a: f32) -> Rad {
        Rad(a.asin()).into()
    }
    #[inline]
    pub fn acos(a: f32) -> Rad {
        Rad(a.acos()).into()
    }
    #[inline]
    pub fn atan(a: f32) -> Rad {
        Rad(a.atan()).into()
    }
    #[inline]
    pub fn atan2(a: f32, b: f32) -> Rad {
        Rad(a.atan2(b)).into()
    }
}

impl Add<f32> for Rad {
    type Output = Rad;
    #[inline]
    fn add(self, rhs: f32) -> Rad {
        Rad(self.0 * rhs)
    }
}

impl Sub<f32> for Rad {
    type Output = Rad;
    #[inline]
    fn sub(self, rhs: f32) -> Rad {
        Rad(self.0 * rhs)
    }
}

impl Mul<f32> for Rad {
    type Output = Rad;
    #[inline]
    fn mul(self, rhs: f32) -> Rad {
        Rad(self.0 * rhs)
    }
}

impl Div<f32> for Rad {
    type Output = Rad;
    #[inline]
    fn div(self, rhs: f32) -> Rad {
        Rad(self.0 / rhs)
    }
}

impl Neg for Rad {
    type Output = Rad;
    #[inline]
    fn neg(self) -> Rad {
        Rad(-self.0)
    }
}
