// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.

use {
    super::{
        conversions::ViewportPercentageLengthConversion,
        ViewportPercentageLength::*,
    },
    crate::{
        domain::numbers::{CssNumber, CssNumberNewType},
        serializers::serialize_dimension::serialize_dimension,
    },
    cssparser::ToCss,
    std::{fmt, ops::*},
};

/// A viewport-relative length.
///
/// <https://drafts.csswg.org/css-values/#viewport-relative-lengths>
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum ViewportPercentageLength<Number: CssNumber> {
    /// A vw unit: <https://drafts.csswg.org/css-values/#vw>
    vw(Number),

    /// A vh unit: <https://drafts.csswg.org/css-values/#vh>
    vh(Number),

    /// <https://drafts.csswg.org/css-values/#vmin>
    vmin(Number),

    /// <https://drafts.csswg.org/css-values/#vmax>
    vmax(Number),
}

impl<Number: CssNumber> ToCss for ViewportPercentageLength<Number> {
    fn to_css<W: fmt::Write>(&self, dest: &mut W) -> fmt::Result {
        use self::ViewportPercentageLength::*;

        match *self {
            vw(length) => serialize_dimension(length, "vw", dest),
            vh(length) => serialize_dimension(length, "vh", dest),
            vmin(length) => serialize_dimension(length, "vmin", dest),
            vmax(length) => serialize_dimension(length, "vmax", dest),
        }
    }
}

impl<Number: CssNumber> Default for ViewportPercentageLength<Number> {
    #[inline(always)]
    fn default() -> Self {
        ViewportPercentageLength::vw(Number::Zero)
    }
}

impl<Number: CssNumber> Add<Number> for ViewportPercentageLength<Number> {
    type Output = ViewportPercentageLength<Number>;

    #[inline(always)]
    fn add(self, rhs: Number) -> Self::Output {
        match self {
            vw(length) => vw(length + rhs),
            vh(length) => vh(length + rhs),
            vmin(length) => vmin(length + rhs),
            vmax(length) => vmax(length + rhs),
        }
    }
}

impl<Number: CssNumber> AddAssign<Number> for ViewportPercentageLength<Number> {
    #[inline(always)]
    fn add_assign(&mut self, rhs: Number) {
        match *self {
            vw(ref mut length) => *length = *length + rhs,
            vh(ref mut length) => *length = *length + rhs,
            vmin(ref mut length) => *length = *length + rhs,
            vmax(ref mut length) => *length = *length + rhs,
        }
    }
}

impl<Number: CssNumber> Sub<Number> for ViewportPercentageLength<Number> {
    type Output = ViewportPercentageLength<Number>;

    #[inline(always)]
    fn sub(self, rhs: Number) -> Self::Output {
        match self {
            vw(length) => vw(length - rhs),
            vh(length) => vh(length - rhs),
            vmin(length) => vmin(length - rhs),
            vmax(length) => vmax(length - rhs),
        }
    }
}

impl<Number: CssNumber> SubAssign<Number> for ViewportPercentageLength<Number> {
    #[inline(always)]
    fn sub_assign(&mut self, rhs: Number) {
        match *self {
            vw(ref mut length) => *length = *length - rhs,
            vh(ref mut length) => *length = *length - rhs,
            vmin(ref mut length) => *length = *length - rhs,
            vmax(ref mut length) => *length = *length - rhs,
        }
    }
}

impl<Number: CssNumber> Mul<Number> for ViewportPercentageLength<Number> {
    type Output = ViewportPercentageLength<Number>;

    #[inline(always)]
    fn mul(self, rhs: Number) -> Self::Output {
        match self {
            vw(length) => vw(length * rhs),
            vh(length) => vh(length * rhs),
            vmin(length) => vmin(length * rhs),
            vmax(length) => vmax(length * rhs),
        }
    }
}

impl<Number: CssNumber> MulAssign<Number> for ViewportPercentageLength<Number> {
    #[inline(always)]
    fn mul_assign(&mut self, rhs: Number) {
        match *self {
            vw(ref mut length) => *length = *length * rhs,
            vh(ref mut length) => *length = *length * rhs,
            vmin(ref mut length) => *length = *length * rhs,
            vmax(ref mut length) => *length = *length * rhs,
        }
    }
}

impl<Number: CssNumber> Div<Number> for ViewportPercentageLength<Number> {
    type Output = ViewportPercentageLength<Number>;

    #[inline(always)]
    fn div(self, rhs: Number) -> Self::Output {
        match self {
            vw(length) => vw(length / rhs),
            vh(length) => vh(length / rhs),
            vmin(length) => vmin(length / rhs),
            vmax(length) => vmax(length / rhs),
        }
    }
}

impl<Number: CssNumber> DivAssign<Number> for ViewportPercentageLength<Number> {
    #[inline(always)]
    fn div_assign(&mut self, rhs: Number) {
        match *self {
            vw(ref mut length) => *length = *length / rhs,
            vh(ref mut length) => *length = *length / rhs,
            vmin(ref mut length) => *length = *length / rhs,
            vmax(ref mut length) => *length = *length / rhs,
        }
    }
}

impl<Number: CssNumber> Rem<Number> for ViewportPercentageLength<Number> {
    type Output = ViewportPercentageLength<Number>;

    #[inline(always)]
    fn rem(self, rhs: Number) -> Self::Output {
        match self {
            vw(length) => vw(length % rhs),
            vh(length) => vh(length % rhs),
            vmin(length) => vmin(length % rhs),
            vmax(length) => vmax(length % rhs),
        }
    }
}

impl<Number: CssNumber> RemAssign<Number> for ViewportPercentageLength<Number> {
    #[inline(always)]
    fn rem_assign(&mut self, rhs: Number) {
        match *self {
            vw(ref mut length) => *length = *length % rhs,
            vh(ref mut length) => *length = *length % rhs,
            vmin(ref mut length) => *length = *length % rhs,
            vmax(ref mut length) => *length = *length % rhs,
        }
    }
}

impl<Number: CssNumber> Neg for ViewportPercentageLength<Number> {
    type Output = ViewportPercentageLength<Number>;

    #[inline(always)]
    fn neg(self) -> Self::Output {
        match self {
            vw(length) => vw(-length),
            vh(length) => vh(-length),
            vmin(length) => vmin(-length),
            vmax(length) => vmax(-length),
        }
    }
}

impl<Number: CssNumber> CssNumberNewType<Number>
    for ViewportPercentageLength<Number>
{
    #[inline(always)]
    fn to_f32(&self) -> f32 {
        self.to_CssNumber().to_f32()
    }

    #[inline(always)]
    fn as_CssNumber(&self) -> &Number {
        match *self {
            vw(ref length) => length,
            vh(ref length) => length,
            vmin(ref length) => length,
            vmax(ref length) => length,
        }
    }
}

impl<Number: CssNumber> ViewportPercentageLength<Number> {
    /// Convert this into a pixel value.
    #[inline(always)]
    pub fn to_px(
        &self,
        viewportPercentageLengthConversion: &dyn ViewportPercentageLengthConversion<Number>,
    ) -> Number {
        match *self {
            vw(length) => length * viewportPercentageLengthConversion.vw(),
            vh(length) => length * viewportPercentageLengthConversion.vh(),
            vmin(length) => length * viewportPercentageLengthConversion.vmin(),
            vmax(length) => length * viewportPercentageLengthConversion.vmax(),
        }
    }

    /// Convert this into AppUnits.
    #[inline]
    pub fn to_app_units(
        &self,
        viewportPercentageLengthConversion: &dyn ViewportPercentageLengthConversion<Number>,
    ) -> Number {
        self.to_px(viewportPercentageLengthConversion) * Number::AppUnitsPerPX
    }
}
