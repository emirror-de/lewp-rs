// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.

use {
    crate::{
        parsers::{Parse, ParserContext},
        CustomParseError,
    },
    cssparser::{BasicParseError, ParseError, Parser, ToCss, Token},
    std::fmt,
};

/// A @font-feature-values block declaration value that keeps a list of values.
#[derive(Clone, Debug, PartialEq)]
pub struct VectorValues(pub Vec<u32>);

impl Parse for VectorValues {
    fn parse<'i, 't>(
        _context: &ParserContext,
        input: &mut Parser<'i, 't>,
    ) -> Result<VectorValues, ParseError<'i, CustomParseError<'i>>> {
        let mut vec = vec![];
        loop {
            match input.next() {
                Ok(&Token::Number {
                    int_value: Some(a), ..
                }) if a >= 0 => {
                    vec.push(a as u32);
                }

                // It can't be anything other than number.
                Ok(unexpectedToken) => {
                    return CustomParseError::unexpectedToken(unexpectedToken)
                }

                Err(_) => break,
            }
        }

        if vec.is_empty() {
            return Err(ParseError::from(BasicParseError {
                kind: cssparser::BasicParseErrorKind::EndOfInput,
                location: input.state().source_location(),
            }));
        }

        Ok(VectorValues(vec))
    }
}

impl ToCss for VectorValues {
    fn to_css<W: fmt::Write>(&self, dest: &mut W) -> fmt::Result {
        let mut iter = self.0.iter();
        let first = iter.next();
        if let Some(first) = first {
            first.to_css(dest)?;
            for value in iter {
                dest.write_char(' ')?;
                value.to_css(dest)?;
            }
        }
        Ok(())
    }
}
