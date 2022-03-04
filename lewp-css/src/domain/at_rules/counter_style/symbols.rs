// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.

use {
    super::Symbol,
    crate::{
        parsers::{Parse, ParserContext},
        CustomParseError,
    },
    cssparser::{ParseError, Parser, ToCss},
    std::fmt::{self, Write},
};

/// <https://drafts.csswg.org/css-counter-styles/#descdef-counter-style-symbols>
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Symbols(pub Vec<Symbol>);

impl ToCss for Symbols {
    fn to_css<W: Write>(&self, dest: &mut W) -> fmt::Result {
        let mut iter = self.0.iter();
        let first = iter.next().unwrap();
        first.to_css(dest)?;
        for item in iter {
            dest.write_char(' ')?;
            item.to_css(dest)?;
        }
        Ok(())
    }
}

impl Parse for Symbols {
    fn parse<'i, 't>(
        context: &ParserContext,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i, CustomParseError<'i>>> {
        let mut symbols = Vec::new();
        loop {
            if let Ok(s) = input.r#try(|input| Symbol::parse(context, input)) {
                symbols.push(s)
            } else if symbols.is_empty() {
                return Err(ParseError::from(
                    CustomParseError::CounterStyleSymbolsCanNotBeEmpty,
                ));
            } else {
                return Ok(Symbols(symbols));
            }
        }
    }
}
