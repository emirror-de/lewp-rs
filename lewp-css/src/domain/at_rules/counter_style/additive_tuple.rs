use {
    super::Symbol,
    crate::{
        parsers::{
            separators::{Comma, Separated},
            Parse,
            ParserContext,
        },
        CustomParseError,
    },
    cssparser::{ParseError, Parser, ToCss},
    std::fmt,
};

// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.

/// <integer> && <symbol>
#[derive(Clone, Debug)]
pub struct AdditiveTuple {
    /// <integer>
    pub weight: u32,

    /// <symbol>
    pub symbol: Symbol,
}

impl ToCss for AdditiveTuple {
    fn to_css<W: fmt::Write>(&self, dest: &mut W) -> fmt::Result {
        self.weight.to_css(dest)?;
        dest.write_char(' ')?;
        self.symbol.to_css(dest)
    }
}

impl Separated for AdditiveTuple {
    type Delimiter = Comma;
}

impl Parse for AdditiveTuple {
    fn parse<'i, 't>(
        context: &ParserContext,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i, CustomParseError<'i>>> {
        let symbol = input.r#try(|input| Symbol::parse(context, input));
        let weight = input.expect_integer()?;
        if weight < 0 {
            return Err(ParseError::from(
                CustomParseError::CounterStyleAdditiveTupleWeightCanNotBeNegative(weight),
            ));
        }
        let symbol = symbol.or_else(|_| Symbol::parse(context, input))?;
        Ok(Self {
            weight: weight as u32,
            symbol,
        })
    }
}
