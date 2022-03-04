// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.

use {
    crate::{
        domain::{
            expressions::CalculablePropertyValue,
            numbers::CssUnsignedNumber,
            units::{LengthOrPercentageUnit, Unit},
        },
        parsers::ParserContext,
        CustomParseError,
    },
    cssparser::{ParseError, Parser, ToCss},
    std::fmt,
    ViewportLength::*,
};

/// ViewportLength is a length | percentage | auto
/// See <http://dev.w3.org/csswg/css-device-adapt/#min-max-width-desc>
/// extend-to-zoom is explicitly not supported as it does not occur in CSS, only when converting from HTML's meta name="viewport" tag (see <http://dev.w3.org/csswg/css-device-adapt/#extend-to-zoom>)
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[allow(missing_docs)]
pub enum ViewportLength {
    /// Automatic length
    auto,

    /// invariant or calculated non-negative length or non-negative percentage
    value(CalculablePropertyValue<LengthOrPercentageUnit<CssUnsignedNumber>>),
}

impl ToCss for ViewportLength {
    fn to_css<W: fmt::Write>(&self, dest: &mut W) -> fmt::Result {
        match *self {
            auto => dest.write_str("auto"),

            value(ref numeric_value) => numeric_value.to_css(dest),
        }
    }
}

impl ViewportLength {
    pub(crate) fn parse<'i, 't>(
        context: &ParserContext,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i, CustomParseError<'i>>> {
        use self::ViewportLength::*;

        if input.r#try(|i| i.expect_ident_matching("auto")).is_ok() {
            return Ok(auto);
        }

        Ok(value(
            LengthOrPercentageUnit::parse_one_outside_calc_function(
                context, input,
            )?,
        ))
    }
}
