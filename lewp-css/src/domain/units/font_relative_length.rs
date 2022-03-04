// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.

use {
    super::{conversions::FontRelativeLengthConversion, FontRelativeLength::*},
    crate::{
        domain::numbers::{CssNumber, CssNumberNewType},
        serializers::serialize_dimension::serialize_dimension,
    },
    cssparser::ToCss,
    std::{fmt, ops::*},
};

/// A font relative length.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum FontRelativeLength<Number: CssNumber> {
    /// A "em" value: <https://drafts.csswg.org/css-values/#em>
    em(Number),

    /// A "ex" value: <https://drafts.csswg.org/css-values/#ex>
    ex(Number),

    /// A "ch" value: <https://drafts.csswg.org/css-values/#ch>
    ch(Number),

    /// A "rem" value: <https://drafts.csswg.org/css-values/#rem>
    rem(Number),
}

impl<Number: CssNumber> ToCss for FontRelativeLength<Number> {
    fn to_css<W: fmt::Write>(&self, dest: &mut W) -> fmt::Result {
        match *self {
            em(length) => serialize_dimension(length, "em", dest),
            ex(length) => serialize_dimension(length, "ex", dest),
            ch(length) => serialize_dimension(length, "ch", dest),
            rem(length) => serialize_dimension(length, "rem", dest),
        }
    }
}

impl<Number: CssNumber> Default for FontRelativeLength<Number> {
    #[inline(always)]
    fn default() -> Self {
        FontRelativeLength::em(Number::default())
    }
}

impl<Number: CssNumber> Add<Number> for FontRelativeLength<Number> {
    type Output = Self;

    #[inline(always)]
    fn add(self, rhs: Number) -> Self::Output {
        match self {
            em(length) => em(length + rhs),
            ex(length) => ex(length + rhs),
            ch(length) => ch(length + rhs),
            rem(length) => rem(length + rhs),
        }
    }
}

impl<Number: CssNumber> AddAssign<Number> for FontRelativeLength<Number> {
    #[inline(always)]
    fn add_assign(&mut self, rhs: Number) {
        match *self {
            em(ref mut length) => *length = *length + rhs,
            ex(ref mut length) => *length = *length + rhs,
            ch(ref mut length) => *length = *length + rhs,
            rem(ref mut length) => *length = *length + rhs,
        }
    }
}

impl<Number: CssNumber> Sub<Number> for FontRelativeLength<Number> {
    type Output = Self;

    #[inline(always)]
    fn sub(self, rhs: Number) -> Self::Output {
        match self {
            em(length) => em(length - rhs),
            ex(length) => ex(length - rhs),
            ch(length) => ch(length - rhs),
            rem(length) => rem(length - rhs),
        }
    }
}

impl<Number: CssNumber> SubAssign<Number> for FontRelativeLength<Number> {
    #[inline(always)]
    fn sub_assign(&mut self, rhs: Number) {
        match *self {
            em(ref mut length) => *length = *length - rhs,
            ex(ref mut length) => *length = *length - rhs,
            ch(ref mut length) => *length = *length - rhs,
            rem(ref mut length) => *length = *length - rhs,
        }
    }
}

impl<Number: CssNumber> Mul<Number> for FontRelativeLength<Number> {
    type Output = Self;

    #[inline(always)]
    fn mul(self, rhs: Number) -> Self::Output {
        match self {
            em(length) => em(length * rhs),
            ex(length) => ex(length * rhs),
            ch(length) => ch(length * rhs),
            rem(length) => rem(length * rhs),
        }
    }
}

impl<Number: CssNumber> MulAssign<Number> for FontRelativeLength<Number> {
    #[inline(always)]
    fn mul_assign(&mut self, rhs: Number) {
        match *self {
            em(ref mut length) => *length = *length * rhs,
            ex(ref mut length) => *length = *length * rhs,
            ch(ref mut length) => *length = *length * rhs,
            rem(ref mut length) => *length = *length * rhs,
        }
    }
}

impl<Number: CssNumber> Div<Number> for FontRelativeLength<Number> {
    type Output = Self;

    #[inline(always)]
    fn div(self, rhs: Number) -> Self::Output {
        match self {
            em(length) => em(length / rhs),
            ex(length) => ex(length / rhs),
            ch(length) => ch(length / rhs),
            rem(length) => rem(length / rhs),
        }
    }
}

impl<Number: CssNumber> DivAssign<Number> for FontRelativeLength<Number> {
    #[inline(always)]
    fn div_assign(&mut self, rhs: Number) {
        match *self {
            em(ref mut length) => *length = *length / rhs,
            ex(ref mut length) => *length = *length / rhs,
            ch(ref mut length) => *length = *length / rhs,
            rem(ref mut length) => *length = *length / rhs,
        }
    }
}

impl<Number: CssNumber> Rem<Number> for FontRelativeLength<Number> {
    type Output = Self;

    #[inline(always)]
    fn rem(self, rhs: Number) -> Self::Output {
        match self {
            em(length) => em(length % rhs),
            ex(length) => ex(length % rhs),
            ch(length) => ch(length % rhs),
            rem(length) => rem(length % rhs),
        }
    }
}

impl<Number: CssNumber> RemAssign<Number> for FontRelativeLength<Number> {
    #[inline(always)]
    fn rem_assign(&mut self, rhs: Number) {
        match *self {
            em(ref mut length) => *length = *length % rhs,
            ex(ref mut length) => *length = *length % rhs,
            ch(ref mut length) => *length = *length % rhs,
            rem(ref mut length) => *length = *length % rhs,
        }
    }
}

impl<Number: CssNumber> Neg for FontRelativeLength<Number> {
    type Output = Self;

    #[inline(always)]
    fn neg(self) -> Self::Output {
        match self {
            em(length) => em(-length),
            ex(length) => ex(-length),
            ch(length) => ch(-length),
            rem(length) => rem(-length),
        }
    }
}

impl<Number: CssNumber> CssNumberNewType<Number>
    for FontRelativeLength<Number>
{
    #[inline(always)]
    fn to_f32(&self) -> f32 {
        self.to_CssNumber().to_f32()
    }

    #[inline(always)]
    fn as_CssNumber(&self) -> &Number {
        match *self {
            em(ref length) => length,
            ex(ref length) => length,
            ch(ref length) => length,
            rem(ref length) => length,
        }
    }
}

impl<Number: CssNumber> FontRelativeLength<Number> {
    /// Convert this into a pixel value.
    #[inline(always)]
    pub(crate) fn to_px(
        self,
        fontRelativeLengthConversion: &dyn FontRelativeLengthConversion<Number>,
    ) -> Number {
        match self {
            em(length) => length * fontRelativeLengthConversion.em(),
            ex(length) => length * fontRelativeLengthConversion.ex(),
            ch(length) => length * fontRelativeLengthConversion.ch(),
            rem(length) => length * fontRelativeLengthConversion.rem(),
        }
    }

    /// Convert this into AppUnits.
    #[inline]
    pub fn to_app_units(
        &self,
        fontRelativeLengthConversion: &dyn FontRelativeLengthConversion<Number>,
    ) -> Number {
        self.to_px(fontRelativeLengthConversion) * Number::AppUnitsPerPX
    }
}
