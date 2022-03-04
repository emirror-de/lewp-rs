// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.

use {
    crate::{
        domain::{
            expressions::CalculablePropertyValue,
            numbers::CssUnsignedNumber,
            units::{NumberOrPercentageUnit, Unit},
        },
        parsers::ParserContext,
        CustomParseError,
    },
    cssparser::{ParseError, Parser, ToCss},
    std::fmt,
    ViewportZoom::*,
};

/// <https://drafts.csswg.org/css-device-adapt/#descdef-viewport-zoom>
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum ViewportZoom {
    /// The `auto` keyword.
    auto,

    /// invariant or calculated non-negative number or non-negative percentage
    value(CalculablePropertyValue<NumberOrPercentageUnit<CssUnsignedNumber>>),
}

impl ToCss for ViewportZoom {
    fn to_css<W: fmt::Write>(&self, dest: &mut W) -> fmt::Result {
        match *self {
            auto => dest.write_str("auto"),

            value(ref calculable) => calculable.to_css(dest),
        }
    }
}

impl ViewportZoom {
    /// Parse a zoom value per:
    ///
    /// <https://drafts.csswg.org/css-device-adapt/#descdef-viewport-zoom>
    pub(crate) fn parse<'i, 't>(
        context: &ParserContext,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i, CustomParseError<'i>>> {
        use self::ViewportZoom::*;

        if input
            .r#try(|input| input.expect_ident_matching("auto"))
            .is_ok()
        {
            return Ok(auto);
        }

        Ok(value(
            NumberOrPercentageUnit::parse_one_outside_calc_function(
                context, input,
            )?,
        ))
    }
}
