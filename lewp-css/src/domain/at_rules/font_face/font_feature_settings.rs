// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.

use {
    super::FontFeatureSetting,
    crate::{
        parsers::{Parse, ParserContext},
        CustomParseError,
    },
    cssparser::{
        serialize_identifier,
        serialize_string,
        ParseError,
        Parser,
        ToCss,
    },
    std::{collections::BTreeMap, fmt},
};

#[derive(Debug, Clone)]
pub struct FontFeatureSettings(pub BTreeMap<String, u32>);

impl ToCss for FontFeatureSettings {
    fn to_css<W: fmt::Write>(&self, dest: &mut W) -> fmt::Result {
        if self.0.is_empty() {
            serialize_identifier("normal", dest)
        } else {
            for (openTypeFeatureTag, integer) in self.0.iter() {
                serialize_string(openTypeFeatureTag, dest)?;
                let integer = *integer;
                if integer != 1 {
                    dest.write_str(" ")?;
                    integer.to_css(dest)?;
                }
            }
            Ok(())
        }
    }
}

impl Parse for FontFeatureSettings {
    fn parse<'i, 't>(
        _: &ParserContext,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i, CustomParseError<'i>>> {
        if input
            .r#try(|input| input.expect_ident_matching("normal"))
            .is_ok()
        {
            Ok(FontFeatureSettings(BTreeMap::new()))
        } else {
            let mut settings = BTreeMap::new();
            for setting in input.parse_comma_separated(|input| {
                FontFeatureSetting::parse(input)
            })? {
                settings.insert(setting.0, setting.1);
            }
            Ok(FontFeatureSettings(settings))
        }
    }
}

impl FontFeatureSettings {
    #[inline(always)]
    pub fn setting(&self, openTypeFeatureTag: &str) -> Option<u32> {
        self.0.get(openTypeFeatureTag).copied()
    }

    #[inline(always)]
    pub fn isOn(&self, openTypeFeatureTag: &str) -> Option<bool> {
        self.0.get(openTypeFeatureTag).map(|integer| *integer == 1)
    }

    #[inline(always)]
    pub fn isOff(&self, openTypeFeatureTag: &str) -> Option<bool> {
        self.0.get(openTypeFeatureTag).map(|integer| *integer == 0)
    }

    #[inline(always)]
    pub fn isNormal(&self) -> bool {
        self.0.is_empty()
    }

    #[inline(always)]
    pub fn normal() -> Self {
        FontFeatureSettings(Default::default())
    }
}
