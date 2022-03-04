// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.

use {
    super::CssNumberConversionError::*,
    crate::{
        domain::{
            numbers::{CssNumberConversionError, CssNumberNewType},
            units::{AppUnitsPer, Unit},
        },
        CustomParseError,
    },
    cssparser::{ParseError, ToCss},
    std::{
        fmt::{Debug, Display},
        hash::Hash,
        ops::*,
    },
};

pub trait CssNumber:
    Sized
    + Copy
    + Clone
    + PartialEq<Self>
    + Eq
    + PartialOrd
    + Ord
    + Hash
    + ToCss
    + Default
    + Add<Self, Output = Self>
    + AddAssign<Self>
    + Sub<Self, Output = Self>
    + SubAssign<Self>
    + Mul<Self, Output = Self>
    + MulAssign<Self>
    + Div<Self, Output = Self>
    + DivAssign<Self>
    + Rem<Self, Output = Self>
    + RemAssign<Self>
    + Neg<Output = Self>
    + Debug
    + Display
    + Deref
    + From<u16>
    + From<u8>
    + CssNumberNewType<Self>
    + Unit<Number = Self>
    + AppUnitsPer
{
    const Zero: Self;

    const One: Self;

    const Maximum: Self;

    const Minimum: Self;

    const DotsPerInch: Self;

    const CentimetresPerInch: Self;

    #[inline(always)]
    fn new(value: f32) -> Result<Self, CssNumberConversionError> {
        const NegativeZero: f32 = -0.0_f32;
        const PositiveZero: f32 = 0.0_f32;

        if value.is_finite() {
            if value == NegativeZero {
                if Self::can_be_negative() {
                    Ok(Self::_construct(PositiveZero))
                } else {
                    Err(NegativeNumberMayNotBeAllowed)
                }
            } else {
                Ok(Self::_construct(value))
            }
        } else if !Self::can_be_negative() && value.is_sign_negative() {
            Err(NegativeNumberMayNotBeAllowed)
        } else if value.is_infinite() {
            Err(InfinityIsNotAllowed)
        } else if value.is_nan() {
            Err(NotANumberIsNotAllowed)
        } else {
            unreachable!("What other kind of f32 is there?");
        }
    }

    fn as_f32(&self) -> f32;

    fn as_u32(&self) -> u32;

    #[inline(always)]
    fn round(self) -> Self {
        Self::_construct(self.to_f32().round())
    }

    #[inline(always)]
    fn abs(self) -> Self {
        Self::_construct(self.to_f32().abs())
    }

    #[inline(always)]
    fn clamp(value: f32) -> Self {
        if value.is_infinite() {
            if value.is_sign_positive() {
                Self::Maximum
            } else {
                Self::Minimum
            }
        } else if value.is_sign_negative() && !Self::can_be_negative() {
            Self::Zero
        } else {
            Self::_construct(value)
        }
    }

    #[inline(always)]
    fn can_be_negative() -> bool {
        Self::Minimum.is_negative()
    }

    #[doc(hidden)]
    fn _construct(value: f32) -> Self;

    fn parseNumber<'i>(
        value: f32,
        _int_value: Option<i32>,
    ) -> Result<Self, ParseError<'i, CustomParseError<'i>>>;
}
