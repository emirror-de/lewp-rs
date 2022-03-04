// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.

use {
    super::{Expression, VarExpression},
    crate::domain::units::{conversions::*, Unit},
    cssparser::ToCss,
    std::{fmt, rc::Rc},
};

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct VarFunction(pub Rc<VarExpression>);

impl ToCss for VarFunction {
    fn to_css<W: fmt::Write>(&self, dest: &mut W) -> fmt::Result {
        dest.write_str("var(")?;
        self.0.to_css(dest)?;
        dest.write_char(')')
    }
}

impl<U: Unit> Expression<U> for VarFunction {
    /// Evaluate the VarFunction by returning the numeric value of the canonical dimension
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
        self.0.evaluate(conversion)
    }
}
