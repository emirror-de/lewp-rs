use {
    super::AdditiveTuple,
    crate::{
        parsers::{Parse, ParserContext},
        CustomParseError,
    },
    cssparser::{ParseError, Parser, ToCss},
};

// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.

/// <https://drafts.csswg.org/css-counter-styles/#descdef-counter-style-additive-symbols>
#[derive(Clone, Debug)]
pub struct AdditiveSymbols(pub Vec<AdditiveTuple>);

impl ToCss for AdditiveSymbols {
    fn to_css<W: std::fmt::Write>(&self, dest: &mut W) -> std::fmt::Result {
        let mut iter = self.0.iter();
        let first = iter.next().unwrap();
        first.to_css(dest)?;
        for item in iter {
            dest.write_char(',')?;
            item.to_css(dest)?;
        }
        Ok(())
    }
}

impl Parse for AdditiveSymbols {
    fn parse<'i, 't>(
        context: &ParserContext,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i, CustomParseError<'i>>> {
        let tuples = Vec::<AdditiveTuple>::parse(context, input)?;
        // FIXME maybe? https://github.com/w3c/csswg-drafts/issues/1220
        if tuples
            .windows(2)
            .any(|window| window[0].weight <= window[1].weight)
        {
            return Err(ParseError::from(CustomParseError::CounterStyleAdditiveSymbolsCanNotHaveASecondWeightEqualToOrGreaterThanTheFirst));
        }
        Ok(AdditiveSymbols(tuples))
    }
}
