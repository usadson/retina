// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use std::ops::{
    Add,
    AddAssign,
    Div,
    DivAssign,
    Mul,
    MulAssign,
    Sub,
    SubAssign,
};

use crate::CssDecimal;

/// The [___reference pixel___][rp] is the pixel unit that is independent of
/// scale by zooming.
///
/// [rp]: https://www.w3.org/TR/css-values-3/#reference-pixel
#[derive(Copy, Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct CssReferencePixels {
    value: CssDecimal,
}

impl CssReferencePixels {
    pub fn as_abs(&self) -> Self {
        if self.value < 0.0 {
            Self { value: 0.0 }
        } else {
            *self
        }
    }

    /// Ensure the value is absolute, by making it 0 if it is below 0.
    pub fn ensure_abs(&mut self) {
        *self = self.as_abs();
    }
}

impl CssReferencePixels {
    pub fn new(value: CssDecimal) -> Self {
        Self { value }
    }

    pub fn value(&self) -> CssDecimal {
        self.value
    }
}

impl Add<CssDecimal> for CssReferencePixels {
    type Output = Self;

    fn add(self, rhs: CssDecimal) -> Self::Output {
        Self::new(self.value + rhs)
    }
}

impl Add<CssReferencePixels> for CssReferencePixels {
    type Output = Self;

    fn add(self, rhs: CssReferencePixels) -> Self::Output {
        Self::new(self.value + rhs.value)
    }
}

impl AddAssign<CssDecimal> for CssReferencePixels {
    fn add_assign(&mut self, rhs: CssDecimal) {
        self.value += rhs;
    }
}


impl AddAssign<CssReferencePixels> for CssReferencePixels {
    fn add_assign(&mut self, rhs: CssReferencePixels) {
        self.value += rhs.value;
    }
}

impl Div<CssDecimal> for CssReferencePixels {
    type Output = Self;

    fn div(self, rhs: CssDecimal) -> Self::Output {
        Self::new(self.value / rhs)
    }
}

impl Div<CssReferencePixels> for CssReferencePixels {
    type Output = Self;

    fn div(self, rhs: CssReferencePixels) -> Self::Output {
        Self::new(self.value / rhs.value)
    }
}

impl DivAssign<CssDecimal> for CssReferencePixels {
    fn div_assign(&mut self, rhs: CssDecimal) {
        self.value /= rhs;
    }
}

impl DivAssign<CssReferencePixels> for CssReferencePixels {
    fn div_assign(&mut self, rhs: CssReferencePixels) {
        self.value /= rhs.value;
    }
}

impl Mul<CssDecimal> for CssReferencePixels {
    type Output = Self;

    fn mul(self, rhs: CssDecimal) -> Self::Output {
        Self::new(self.value * rhs)
    }
}

impl Mul<CssReferencePixels> for CssReferencePixels {
    type Output = Self;

    fn mul(self, rhs: CssReferencePixels) -> Self::Output {
        Self::new(self.value * rhs.value)
    }
}

impl MulAssign<CssDecimal> for CssReferencePixels {
    fn mul_assign(&mut self, rhs: CssDecimal) {
        self.value *= rhs;
    }
}

impl MulAssign<CssReferencePixels> for CssReferencePixels {
    fn mul_assign(&mut self, rhs: CssReferencePixels) {
        self.value *= rhs.value;
    }
}

impl Sub<CssDecimal> for CssReferencePixels {
    type Output = Self;

    fn sub(self, rhs: CssDecimal) -> Self::Output {
        Self::new(self.value - rhs)
    }
}

impl Sub<CssReferencePixels> for CssReferencePixels {
    type Output = Self;

    fn sub(self, rhs: CssReferencePixels) -> Self::Output {
        Self::new(self.value - rhs.value)
    }
}

impl SubAssign<CssDecimal> for CssReferencePixels {
    fn sub_assign(&mut self, rhs: CssDecimal) {
        self.value -= rhs;
    }
}

impl SubAssign<CssReferencePixels> for CssReferencePixels {
    fn sub_assign(&mut self, rhs: CssReferencePixels) {
        self.value -= rhs.value;
    }
}
