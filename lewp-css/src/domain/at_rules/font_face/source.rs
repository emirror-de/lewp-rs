// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.

use {
    super::{FamilyName, FontUrlSource},
    crate::{
        domain::SpecifiedUrl,
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

/// A source for a font-face rule.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Source {
    /// A `url()` source.
    Url(FontUrlSource),

    /// A `local()` source.
    Local(FamilyName),
}

impl ToCss for Source {
    fn to_css<W: fmt::Write>(&self, dest: &mut W) -> fmt::Result {
        use self::Source::*;

        match *self {
            Url(ref fontUrlSource) => fontUrlSource.to_css(dest),
            Local(ref familyName) => familyName.to_css(dest),
        }
    }
}

impl Separated for Source {
    type Delimiter = Comma;
}

impl Parse for Source {
    fn parse<'i, 't>(
        context: &ParserContext,
        input: &mut Parser<'i, 't>,
    ) -> Result<Source, ParseError<'i, CustomParseError<'i>>> {
        use self::Source::*;

        if input
            .r#try(|input| input.expect_function_matching("local"))
            .is_ok()
        {
            input
                .parse_nested_block(|input| FamilyName::parse(context, input))
                .map(Local)
        } else {
            let url = SpecifiedUrl::parse(context, input)?;

            // Parsing optional format()
            let format_hints = if input
                .r#try(|input| input.expect_function_matching("format"))
                .is_ok()
            {
                input.parse_nested_block(|input| {
                    input.parse_comma_separated(|input| {
                        Ok(input.expect_string()?.as_ref().to_owned())
                    })
                })?
            } else {
                vec![]
            };

            Ok(Url(FontUrlSource { url, format_hints }))
        }
    }
}
