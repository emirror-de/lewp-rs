// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.

use {
    super::{AttrFunction, CalcFunction, Expression, VarFunction},
    crate::domain::units::{conversions::*, PercentageUnit, Unit},
    cssparser::ToCss,
    std::fmt,
    CalculablePropertyValue::*,
};

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum CalculablePropertyValue<U: Unit> {
    Constant(U),

    Percentage(PercentageUnit<U::Number>),

    Calc(CalcFunction<U>),

    Attr(AttrFunction),

    Var(VarFunction),
}

impl<U: Unit> Default for CalculablePropertyValue<U> {
    #[inline(always)]
    fn default() -> Self {
        CalculablePropertyValue::Constant(U::default())
    }
}

impl<U: Unit> ToCss for CalculablePropertyValue<U> {
    fn to_css<W: fmt::Write>(&self, dest: &mut W) -> fmt::Result {
        match *self {
            Constant(ref constant) => constant.to_css(dest),

            Percentage(ref percentage) => percentage.to_css(dest),

            Calc(ref function) => function.to_css(dest),

            Attr(ref function) => function.to_css(dest),

            Var(ref function) => function.to_css(dest),
        }
    }
}

impl<U: Unit> Expression<U> for CalculablePropertyValue<U> {
    /// Evaluate the CalculablePropertyValue by returning the numeric value of the canonical dimension
    /// Division by zero is handled by returning the maximum possible f32 value
    /// Subtractions for UnsignedCssNumber that are negative are handled by returning 0.0
    #[inline(always)]
    fn evaluate<
        Conversion: FontRelativeLengthConversion<U::Number>
            + ViewportPercentageLengthConversion<U::Number>
            + PercentageConversion<U::Number>
            + AttributeConversion<U>
            + CssVariableConversion,
    >(
        &self,
        conversion: &Conversion,
    ) -> Option<U::Number> {
        match *self {
            Constant(ref constant) => Some(constant.to_CssNumber()),

            Percentage(ref percentage) => {
                Some(percentage.to_absolute_value(conversion))
            }

            Calc(ref function) => function.evaluate(conversion),

            Attr(ref function) => function.evaluate(conversion),

            Var(ref function) => function.evaluate(conversion),
        }
    }
}
