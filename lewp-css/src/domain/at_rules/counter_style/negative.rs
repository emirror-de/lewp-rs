use {
    super::Symbol,
    crate::{
        parsers::{Parse, ParserContext},
        CustomParseError,
    },
    cssparser::{ParseError, Parser, ToCss},
    std::fmt,
};

// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.

/// <https://drafts.csswg.org/css-counter-styles/#counter-style-negative>
#[derive(Clone, Debug)]
pub struct Negative(pub Symbol, pub Option<Symbol>);

impl ToCss for Negative {
    fn to_css<W: fmt::Write>(&self, dest: &mut W) -> fmt::Result {
        self.0.to_css(dest)?;
        if let Some(ref symbol) = self.1 {
            dest.write_char(' ')?;
            symbol.to_css(dest)?;
        }
        Ok(())
    }
}

impl Parse for Negative {
    fn parse<'i, 't>(
        context: &ParserContext,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i, CustomParseError<'i>>> {
        Ok(Negative(
            Symbol::parse(context, input)?,
            input.r#try(|input| Symbol::parse(context, input)).ok(),
        ))
    }
}
