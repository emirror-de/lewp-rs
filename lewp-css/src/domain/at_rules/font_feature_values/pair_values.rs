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

/// A @font-feature-values block declaration value that keeps one or two values.
#[derive(Clone, Debug, PartialEq)]
pub struct PairValues(pub u32, pub Option<u32>);

impl Parse for PairValues {
    fn parse<'i, 't>(
        _context: &ParserContext,
        input: &mut Parser<'i, 't>,
    ) -> Result<PairValues, ParseError<'i, CustomParseError<'i>>> {
        let first = match *input.next()? {
            Token::Number {
                int_value: Some(firstValue),
                ..
            } if firstValue >= 0 => firstValue as u32,
            ref unexpectedToken => {
                return CustomParseError::unexpectedToken(unexpectedToken)
            }
        };

        match input.next() {
            Ok(&Token::Number {
                int_value: Some(secondValue),
                ..
            }) if secondValue >= 0 => {
                Ok(PairValues(first, Some(secondValue as u32)))
            }

            // It can't be anything other than number.
            Ok(unexpectedToken) => {
                CustomParseError::unexpectedToken(unexpectedToken)
            }

            // It can be just one value.
            Err(_) => Ok(PairValues(first, None)),
        }
    }
}

impl ToCss for PairValues {
    fn to_css<W: fmt::Write>(&self, dest: &mut W) -> fmt::Result {
        self.0.to_css(dest)?;

        if let Some(second) = self.1 {
            dest.write_char(' ')?;
            second.to_css(dest)?;
        }
        Ok(())
    }
}
