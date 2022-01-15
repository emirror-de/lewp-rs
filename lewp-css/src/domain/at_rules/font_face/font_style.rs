// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.

use {
    crate::{
        define_css_keyword_enum,
        parsers::{Parse, ParserContext},
        CustomParseError,
    },
    cssparser::{ParseError, Parser},
};

define_css_keyword_enum! {
    FontStyle:
    "normal" => normal,
    "italic" => italic,
    "oblique" => oblique,
}

impl Parse for FontStyle {
    fn parse<'i, 't>(
        _: &ParserContext,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i, CustomParseError<'i>>> {
        FontStyle::parse(input)
    }
}
