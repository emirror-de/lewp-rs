// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.

use {
    crate::{
        domain::{
            expressions::CalculablePropertyValue,
            numbers::{CssNumber, CssSignedNumber},
            units::{ResolutionUnit, Unit},
        },
        parsers::ParserContext,
        CustomParseError,
    },
    cssparser::{ParseError, Parser, ToCss, Token},
    std::fmt,
};

/// A resolution.
#[derive(Clone, Debug, PartialEq)]
pub enum MediaResolution {
    infinite,

    finite(CalculablePropertyValue<ResolutionUnit<CssSignedNumber>>),
}

impl ToCss for MediaResolution {
    fn to_css<W: fmt::Write>(&self, dest: &mut W) -> fmt::Result {
        use self::MediaResolution::*;

        match *self {
            infinite => dest.write_str("infinite"),
            finite(ref value) => value.to_css(dest),
        }
    }
}

impl MediaResolution {
    // WebKit only supports integer values of 1 and 2, but we more liberally support floating point values of any value, positive or negative or zero
    pub(crate) fn parseWebKit<'i, 't>(
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i, CustomParseError<'i>>> {
        use self::MediaResolution::*;

        let value = match *input.next()? {
            Token::Number { value, .. } => CssSignedNumber::new(value).map_err(
                |cssNumberConversionError| {
                    ParseError::from(
                        CustomParseError::CouldNotParseCssSignedNumber(
                            cssNumberConversionError,
                            value,
                        ),
                    )
                },
            ),

            ref unexpectedToken => {
                CustomParseError::unexpectedToken(unexpectedToken)
            }
        }?;

        Ok(finite(CalculablePropertyValue::Constant(
            ResolutionUnit::dppx(value),
        )))
    }

    pub(crate) fn parse<'i, 't>(
        context: &ParserContext,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i, CustomParseError<'i>>> {
        use self::MediaResolution::*;

        if input.r#try(|i| i.expect_ident_matching("auto")).is_ok() {
            return Ok(infinite);
        }

        Ok(finite(ResolutionUnit::parse_one_outside_calc_function(
            context, input,
        )?))
    }
}
