// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.

use {
    crate::{
        parsers::{Parse, ParserContext},
        CustomParseError,
    },
    cssparser::{ParseError, Parser, ToCss, Token},
    std::fmt,
};

/// A @font-feature-values block declaration value that keeps one value.
#[derive(Clone, Debug, PartialEq)]
pub struct SingleValue(pub u32);

impl Parse for SingleValue {
    fn parse<'i, 't>(
        _context: &ParserContext,
        input: &mut Parser<'i, 't>,
    ) -> Result<SingleValue, ParseError<'i, CustomParseError<'i>>> {
        match *input.next()? {
            Token::Number {
                int_value: Some(value),
                ..
            } if value >= 0 => Ok(SingleValue(value as u32)),
            ref unexpectedToken => {
                CustomParseError::unexpectedToken(unexpectedToken)
            }
        }
    }
}

impl ToCss for SingleValue {
    fn to_css<W: fmt::Write>(&self, dest: &mut W) -> fmt::Result {
        self.0.to_css(dest)
    }
}
