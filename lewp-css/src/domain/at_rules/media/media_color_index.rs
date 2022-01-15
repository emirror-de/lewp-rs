// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.

use {
    crate::{
        domain::{
            expressions::CalculablePropertyValue,
            numbers::CssUnsignedInteger,
            units::Unit,
        },
        parsers::{Parse, ParserContext},
        CustomParseError,
    },
    cssparser::{ParseError, Parser, ToCss},
    std::fmt,
};

#[derive(Default, Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct MediaColorIndex(CalculablePropertyValue<CssUnsignedInteger>);

impl ToCss for MediaColorIndex {
    fn to_css<W: fmt::Write>(&self, dest: &mut W) -> fmt::Result {
        self.0.to_css(dest)
    }
}

impl Parse for MediaColorIndex {
    fn parse<'i, 't>(
        context: &ParserContext,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i, CustomParseError<'i>>> {
        Ok(MediaColorIndex(
            CssUnsignedInteger::parse_one_outside_calc_function(
                context, input,
            )?,
        ))
    }
}
