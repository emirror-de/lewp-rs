use {
    crate::{
        parsers::{Parse, ParserContext},
        CustomParseError,
    },
    cssparser::{ParseError, Parser, ToCss},
    std::fmt,
};

use super::Symbol;

// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.

/// <https://drafts.csswg.org/css-counter-styles/#counter-style-pad>
#[derive(Clone, Debug)]
pub struct Pad(pub u32, pub Symbol);

impl ToCss for Pad {
    fn to_css<W: fmt::Write>(&self, dest: &mut W) -> fmt::Result {
        self.0.to_css(dest)?;
        dest.write_char(' ')?;
        self.1.to_css(dest)
    }
}

impl Parse for Pad {
    fn parse<'i, 't>(
        context: &ParserContext,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i, CustomParseError<'i>>> {
        let pad_with = input.r#try(|input| Symbol::parse(context, input));
        let min_length = input.expect_integer()?;
        if min_length < 0 {
            return Err(ParseError::from(
                CustomParseError::CounterStylePadMinLengthCanNotBeNegative(
                    min_length,
                ),
            ));
        }
        let pad_with = pad_with.or_else(|_| Symbol::parse(context, input))?;
        Ok(Pad(min_length as u32, pad_with))
    }
}
