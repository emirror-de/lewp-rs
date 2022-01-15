// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.

use {
    crate::{
        domain::CounterStyleIdent,
        parsers::{Parse, ParserContext},
        CustomParseError,
    },
    cssparser::{ParseError, Parser, ToCss},
    std::fmt,
};

/// <https://drafts.csswg.org/css-counter-styles/#counter-style-speak-as>
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SpeakAs {
    /// auto
    Auto,

    /// bullets
    Bullets,

    /// numbers
    Numbers,

    /// words
    Words,

    SpellOut,

    /// <counter-style-name>
    Other(CounterStyleIdent),
}

impl ToCss for SpeakAs {
    fn to_css<W: fmt::Write>(&self, dest: &mut W) -> fmt::Result {
        use self::SpeakAs::*;

        match *self {
            Auto => dest.write_str("auto"),

            Bullets => dest.write_str("bullets"),

            Numbers => dest.write_str("numbers"),

            Words => dest.write_str("words"),

            SpellOut => dest.write_str("spell-out"),

            Other(ref ident) => ident.to_css(dest),
        }
    }
}

impl Parse for SpeakAs {
    fn parse<'i, 't>(
        _context: &ParserContext,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i, CustomParseError<'i>>> {
        use self::SpeakAs::*;

        let result = input.r#try(|input| {
            let ident = input.expect_ident().map_err(|_| ())?;
            match_ignore_ascii_case! {
                &*ident,

                "auto" => Ok(Auto),

                "bullets" => Ok(Bullets),

                "numbers" => Ok(Numbers),

                "words" => Ok(Words),

                "spell-out" => Ok(SpellOut),

                _ => Err(()),
            }
        });

        result.or_else(|_| Ok(Other(CounterStyleIdent::parse(input)?)))
    }
}
