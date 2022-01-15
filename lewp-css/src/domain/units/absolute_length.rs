// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.

use {
    super::AbsoluteLength::*,
    crate::{
        domain::numbers::{CssNumber, CssNumberNewType},
        serializers::serialize_dimension::serialize_dimension,
    },
    cssparser::ToCss,
    std::{fmt, ops::*},
};

/// Represents an absolute length with its unit
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum AbsoluteLength<Number: CssNumber> {
    /// An absolute length in pixels (px)
    px(Number),

    /// An absolute length in inches (in)
    in_(Number),

    /// An absolute length in centimeters (cm)
    cm(Number),

    /// An absolute length in millimeters (mm)
    mm(Number),

    /// An absolute length in quarter-millimeters (q)
    q(Number),

    /// An absolute length in points (pt)
    pt(Number),

    /// An absolute length in pica (pc)
    pc(Number),
}

impl<Number: CssNumber> ToCss for AbsoluteLength<Number> {
    fn to_css<W: fmt::Write>(&self, dest: &mut W) -> fmt::Result {
        match *self {
            px(length) => serialize_dimension(length, "px", dest),
            in_(length) => serialize_dimension(length, "in", dest),
            cm(length) => serialize_dimension(length, "cm", dest),
            mm(length) => serialize_dimension(length, "mm", dest),
            q(length) => serialize_dimension(length, "q", dest),
            pt(length) => serialize_dimension(length, "pt", dest),
            pc(length) => serialize_dimension(length, "pc", dest),
        }
    }
}

impl<Number: CssNumber> Default for AbsoluteLength<Number> {
    #[inline(always)]
    fn default() -> Self {
        AbsoluteLength::px(Number::default())
    }
}

impl<Number: CssNumber> Add<Number> for AbsoluteLength<Number> {
    type Output = AbsoluteLength<Number>;

    #[inline(always)]
    fn add(self, rhs: Number) -> Self::Output {
        match self {
            px(length) => px(length + rhs),
            in_(length) => in_(length + rhs),
            cm(length) => cm(length + rhs),
            mm(length) => mm(length + rhs),
            q(length) => q(length + rhs),
            pt(length) => pt(length + rhs),
            pc(length) => pc(length + rhs),
        }
    }
}

impl<Number: CssNumber> AddAssign<Number> for AbsoluteLength<Number> {
    #[inline(always)]
    fn add_assign(&mut self, rhs: Number) {
        match *self {
            px(ref mut length) => *length = *length + rhs,
            in_(ref mut length) => *length = *length + rhs,
            cm(ref mut length) => *length = *length + rhs,
            mm(ref mut length) => *length = *length + rhs,
            q(ref mut length) => *length = *length + rhs,
            pt(ref mut length) => *length = *length + rhs,
            pc(ref mut length) => *length = *length + rhs,
        }
    }
}

impl<Number: CssNumber> Sub<Number> for AbsoluteLength<Number> {
    type Output = AbsoluteLength<Number>;

    #[inline(always)]
    fn sub(self, rhs: Number) -> Self::Output {
        match self {
            px(length) => px(length - rhs),
            in_(length) => in_(length - rhs),
            cm(length) => cm(length - rhs),
            mm(length) => mm(length - rhs),
            q(length) => q(length - rhs),
            pt(length) => pt(length - rhs),
            pc(length) => pc(length - rhs),
        }
    }
}

impl<Number: CssNumber> SubAssign<Number> for AbsoluteLength<Number> {
    #[inline(always)]
    fn sub_assign(&mut self, rhs: Number) {
        match *self {
            px(ref mut length) => *length = *length - rhs,
            in_(ref mut length) => *length = *length - rhs,
            cm(ref mut length) => *length = *length - rhs,
            mm(ref mut length) => *length = *length - rhs,
            q(ref mut length) => *length = *length - rhs,
            pt(ref mut length) => *length = *length - rhs,
            pc(ref mut length) => *length = *length - rhs,
        }
    }
}

impl<Number: CssNumber> Mul<Number> for AbsoluteLength<Number> {
    type Output = AbsoluteLength<Number>;

    #[inline(always)]
    fn mul(self, rhs: Number) -> Self::Output {
        match self {
            px(length) => px(length * rhs),
            in_(length) => in_(length * rhs),
            cm(length) => cm(length * rhs),
            mm(length) => mm(length * rhs),
            q(length) => q(length * rhs),
            pt(length) => pt(length * rhs),
            pc(length) => pc(length * rhs),
        }
    }
}

impl<Number: CssNumber> MulAssign<Number> for AbsoluteLength<Number> {
    #[inline(always)]
    fn mul_assign(&mut self, rhs: Number) {
        match *self {
            px(ref mut length) => *length = *length * rhs,
            in_(ref mut length) => *length = *length * rhs,
            cm(ref mut length) => *length = *length * rhs,
            mm(ref mut length) => *length = *length * rhs,
            q(ref mut length) => *length = *length * rhs,
            pt(ref mut length) => *length = *length * rhs,
            pc(ref mut length) => *length = *length * rhs,
        }
    }
}

impl<Number: CssNumber> Div<Number> for AbsoluteLength<Number> {
    type Output = AbsoluteLength<Number>;

    #[inline(always)]
    fn div(self, rhs: Number) -> Self::Output {
        match self {
            px(length) => px(length / rhs),
            in_(length) => in_(length / rhs),
            cm(length) => cm(length / rhs),
            mm(length) => mm(length / rhs),
            q(length) => q(length / rhs),
            pt(length) => pt(length / rhs),
            pc(length) => pc(length / rhs),
        }
    }
}

impl<Number: CssNumber> DivAssign<Number> for AbsoluteLength<Number> {
    #[inline(always)]
    fn div_assign(&mut self, rhs: Number) {
        match *self {
            px(ref mut length) => *length = *length / rhs,
            in_(ref mut length) => *length = *length / rhs,
            cm(ref mut length) => *length = *length / rhs,
            mm(ref mut length) => *length = *length / rhs,
            q(ref mut length) => *length = *length / rhs,
            pt(ref mut length) => *length = *length / rhs,
            pc(ref mut length) => *length = *length / rhs,
        }
    }
}

impl<Number: CssNumber> Rem<Number> for AbsoluteLength<Number> {
    type Output = AbsoluteLength<Number>;

    #[inline(always)]
    fn rem(self, rhs: Number) -> Self::Output {
        match self {
            px(length) => px(length % rhs),
            in_(length) => in_(length % rhs),
            cm(length) => cm(length % rhs),
            mm(length) => mm(length % rhs),
            q(length) => q(length % rhs),
            pt(length) => pt(length % rhs),
            pc(length) => pc(length % rhs),
        }
    }
}

impl<Number: CssNumber> RemAssign<Number> for AbsoluteLength<Number> {
    #[inline(always)]
    fn rem_assign(&mut self, rhs: Number) {
        match *self {
            px(ref mut length) => *length = *length % rhs,
            in_(ref mut length) => *length = *length % rhs,
            cm(ref mut length) => *length = *length % rhs,
            mm(ref mut length) => *length = *length % rhs,
            q(ref mut length) => *length = *length % rhs,
            pt(ref mut length) => *length = *length % rhs,
            pc(ref mut length) => *length = *length % rhs,
        }
    }
}

impl<Number: CssNumber> Neg for AbsoluteLength<Number> {
    type Output = AbsoluteLength<Number>;

    #[inline(always)]
    fn neg(self) -> Self::Output {
        match self {
            px(length) => px(-length),
            in_(length) => in_(-length),
            cm(length) => cm(-length),
            mm(length) => mm(-length),
            q(length) => q(-length),
            pt(length) => pt(-length),
            pc(length) => pc(-length),
        }
    }
}

impl<Number: CssNumber> CssNumberNewType<Number> for AbsoluteLength<Number> {
    #[inline(always)]
    fn to_f32(&self) -> f32 {
        self.to_CssNumber().to_f32()
    }

    #[inline(always)]
    fn as_CssNumber(&self) -> &Number {
        match *self {
            px(ref length) => length,
            in_(ref length) => length,
            cm(ref length) => length,
            mm(ref length) => length,
            q(ref length) => length,
            pt(ref length) => length,
            pc(ref length) => length,
        }
    }
}

impl<Number: CssNumber> AbsoluteLength<Number> {
    /// Convert this into a pixel value.
    #[inline]
    pub fn to_px(&self) -> Number {
        match *self {
            px(value) => value,
            in_(value) => {
                value * (Number::AppUnitsPerIN / Number::AppUnitsPerPX)
            }
            cm(value) => {
                value * (Number::AppUnitsPerCM / Number::AppUnitsPerPX)
            }
            mm(value) => {
                value * (Number::AppUnitsPerMM / Number::AppUnitsPerPX)
            }
            q(value) => value * (Number::AppUnitsPerQ / Number::AppUnitsPerPX),
            pt(value) => {
                value * (Number::AppUnitsPerPT / Number::AppUnitsPerPX)
            }
            pc(value) => {
                value * (Number::AppUnitsPerPC / Number::AppUnitsPerPX)
            }
        }
    }

    /// Convert this into AppUnits.
    #[inline]
    pub fn to_app_units(&self) -> Number {
        match *self {
            px(value) => value * Number::AppUnitsPerPX,
            in_(value) => value * Number::AppUnitsPerIN,
            cm(value) => value * Number::AppUnitsPerCM,
            mm(value) => value * Number::AppUnitsPerMM,
            q(value) => value * Number::AppUnitsPerQ,
            pt(value) => value * Number::AppUnitsPerPT,
            pc(value) => value * Number::AppUnitsPerPC,
        }
    }
}
