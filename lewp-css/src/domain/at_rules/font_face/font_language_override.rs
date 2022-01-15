// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.

use {
    super::OpenTypeLanguageTag,
    crate::CustomParseError,
    cssparser::{serialize_identifier, ParseError, Parser, ToCss},
    std::fmt,
};

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum FontLanguageOverride {
    normal,
    Override(OpenTypeLanguageTag),
}

impl ToCss for FontLanguageOverride {
    fn to_css<W: fmt::Write>(&self, dest: &mut W) -> fmt::Result {
        use self::FontLanguageOverride::*;

        match *self {
            normal => serialize_identifier("normal", dest),
            Override(openTypeLanguageTag) => openTypeLanguageTag.to_css(dest),
        }
    }
}

impl FontLanguageOverride {
    /// Parse a font-family value
    pub(crate) fn parse<'i, 't>(
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i, CustomParseError<'i>>> {
        use self::FontLanguageOverride::*;

        if let Ok(value) = input.r#try(|input| match input.expect_string() {
            Err(_) => Err(()),
            Ok(value) => match OpenTypeLanguageTag::parse(value) {
                Err(_) => Err(()),
                Ok(openTypeLanguageTag) => Ok(Override(openTypeLanguageTag)),
            },
        }) {
            return Ok(value);
        }

        let identifier = input.expect_ident()?.clone();
        match_ignore_ascii_case! {
            &identifier,

            "normal" => Ok(normal),

            _ => Err(ParseError::from(CustomParseError::InvalidFontLanguageOverrideIdentifier(identifier.clone()))),
        }
    }
}
