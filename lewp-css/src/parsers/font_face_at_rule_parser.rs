// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.

use {
    super::{Parse, ParserContext},
    crate::{
        domain::at_rules::font_face::{FontFaceAtRule, FontLanguageOverride},
        CustomParseError,
    },
    cssparser::{
        AtRuleParser,
        CowRcStr,
        DeclarationParser,
        ParseError,
        Parser,
    },
};

pub(crate) struct FontFaceAtRuleParser<'a> {
    pub(crate) context: &'a ParserContext,
    pub(crate) rule: &'a mut FontFaceAtRule,
}

/// Default methods reject all at rules.
impl<'a, 'i> AtRuleParser<'i> for FontFaceAtRuleParser<'a> {
    type Prelude = ();

    type AtRule = ();

    type Error = CustomParseError<'i>;
}

impl<'a, 'i> DeclarationParser<'i> for FontFaceAtRuleParser<'a> {
    type Declaration = ();

    type Error = CustomParseError<'i>;

    fn parse_value<'t>(
        &mut self,
        name: CowRcStr<'i>,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self::Declaration, ParseError<'i, CustomParseError<'i>>> {
        // DeclarationParser also calls parse_entirely so we’d normally not need to, but in these cases we do because we set the value as a side effect rather than returning it.

        match_ignore_ascii_case! {
            &name,

            "font-family" => self.rule.family = Some(input.parse_entirely(|i| Parse::parse(self.context, i))?),

            "src" => self.rule.sources = Some(input.parse_entirely(|i| Parse::parse(self.context, i))?),

            "font-style" => self.rule.style = Some(input.parse_entirely(|i| Parse::parse(self.context, i))?),

            "font-weight" => self.rule.weight = Some(input.parse_entirely(|i| Parse::parse(self.context, i))?),

            "font-stretch" => self.rule.stretch = Some(input.parse_entirely(|i| Parse::parse(self.context, i))?),

            "font-display" => self.rule.display = Some(input.parse_entirely(|i| Parse::parse(self.context, i))?),

            "unicode-range" => self.rule.unicode_range = Some(input.parse_entirely(|i| Parse::parse(self.context, i))?),

            "font-feature-settings" => self.rule.feature_settings = Some(input.parse_entirely(|i| Parse::parse(self.context, i))?),

            "font-language-override" => self.rule.language_override = Some(input.parse_entirely(FontLanguageOverride::parse)?),

            _ => return Err(ParseError::from(CustomParseError::UnsupportedFontFaceProperty(name.clone())))
        }

        Ok(())
    }
}
