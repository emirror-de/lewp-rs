// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.

use super::CssNumber;

pub trait CssNumberNewType<Number: CssNumber>: Sized {
    fn to_f32(&self) -> f32;

    fn as_CssNumber(&self) -> &Number;

    #[inline(always)]
    fn to_CssNumber(&self) -> Number {
        *self.as_CssNumber()
    }

    #[inline(always)]
    fn is_zero(&self) -> bool {
        const PositiveZero: f32 = 0.0_f32;

        self.to_f32() == PositiveZero
    }

    #[inline(always)]
    fn is_positive(&self) -> bool {
        !self.is_zero() && self.to_f32().is_sign_positive()
    }

    #[inline(always)]
    fn is_negative(&self) -> bool {
        self.to_f32().is_sign_negative()
    }

    #[inline(always)]
    fn is_zero_or_positive(&self) -> bool {
        self.to_f32().is_sign_positive()
    }

    #[inline(always)]
    fn is_zero_or_negative(&self) -> bool {
        self.is_zero() || self.is_negative()
    }
}
