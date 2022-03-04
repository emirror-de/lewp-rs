// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.

use {
    super::{
        FamilyName,
        FamilyNameSyntax::*,
        FontFamily::*,
        GenericFontFamilyName::{self, *},
    },
    crate::{domain::Atom, CustomParseError},
    cssparser::{serialize_identifier, ParseError, Parser, ToCss},
    std::fmt,
};

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum FontFamily {
    FamilyName(FamilyName),
    Generic(GenericFontFamilyName),
}

impl ToCss for FontFamily {
    fn to_css<W: fmt::Write>(&self, dest: &mut W) -> fmt::Result {
        use self::FontFamily::*;

        match *self {
            FamilyName(ref name) => name.to_css(dest),

            Generic(ref name) => name.to_css(dest),
        }
    }
}

impl FontFamily {
    /// Parse a font-family value
    pub(crate) fn parse<'i, 't>(
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i, CustomParseError<'i>>> {
        if let Ok(value) = input.r#try(|i| i.expect_string_cloned()) {
            return Ok(FontFamily::FamilyName(FamilyName {
                name: Atom::from(&*value),
                syntax: Quoted,
            }));
        }

        let first_ident = input.expect_ident()?.clone();

        // From https://drafts.csswg.org/css-fonts/#propdef-font-family:-
        // Font family names that happen to be the same as a keyword value (`inherit`, `serif`, `sans-serif`, `monospace`, `fantasy`, and `cursive`) must be quoted to prevent confusion with the keywords with the same names.
        // The keywords ‘initial’ and ‘default’ are reserved for future use and must also be quoted when used as font names.
        // UAs must not consider these keywords as matching the <family-name> type."
        let is_css_wide_keyword = {
            match_ignore_ascii_case! {
                &first_ident,

                "serif" => return Ok(Generic(serif)),
                "sans-serif" => return Ok(Generic(sans_serif)),
                "cursive" => return Ok(Generic(cursive)),
                "fantasy" => return Ok(Generic(fantasy)),
                "monospace" => return Ok(Generic(monospace)),

                "inherit" | "initial" | "unset" | "default" => true,

                _ => false,
            }
        };

        let mut value = first_ident.as_ref().to_owned();
        let mut serialization = String::new();
        serialize_identifier(&first_ident, &mut serialization).unwrap();

        // These keywords are not allowed by themselves.
        // The only way this value can be valid is with another keyword.
        if is_css_wide_keyword {
            let ident = input.expect_ident()?;
            value.push(' ');
            value.push_str(ident);
            serialization.push(' ');
            serialize_identifier(ident, &mut serialization).unwrap();
        }

        while let Ok(ident) = input.r#try(|i| i.expect_ident_cloned()) {
            value.push(' ');
            value.push_str(&ident);
            serialization.push(' ');
            serialize_identifier(&ident, &mut serialization).unwrap();
        }

        Ok(FontFamily::FamilyName(FamilyName {
            name: Atom::from(value),
            syntax: Identifiers(serialization),
        }))
    }
}
