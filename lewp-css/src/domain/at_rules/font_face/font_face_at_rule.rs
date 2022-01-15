// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.

use {
    super::{
        FamilyName,
        FontDisplay,
        FontFace,
        FontFeatureSettings,
        FontLanguageOverride,
        FontStretch,
        FontStyle,
        FontWeight,
        Source,
    },
    crate::{
        parsers::{FontFaceAtRuleParser, ParserContext},
        CustomParseError,
    },
    cssparser::{
        DeclarationListParser,
        ParseError,
        Parser,
        ToCss,
        UnicodeRange,
    },
    std::fmt,
};

/// A `@font-face` rule.
///
/// <https://drafts.csswg.org/css-fonts/#font-face-rule>
#[derive(Debug, Clone)]
pub struct FontFaceAtRule {
    /// The name of this font face
    pub family: Option<FamilyName>,

    /// The alternative sources for this font face.
    pub sources: Option<Vec<Source>>,

    /// The style of this font face
    pub style: Option<FontStyle>,

    /// The weight of this font face
    pub weight: Option<FontWeight>,

    /// The stretch of this font face
    pub stretch: Option<FontStretch>,

    /// The display of this font face
    pub display: Option<FontDisplay>,

    /// The ranges of code points outside of which this font face should not be used.
    pub unicode_range: Option<Vec<UnicodeRange>>,

    /// The feature settings of this font face.
    pub feature_settings: Option<FontFeatureSettings>,

    /// The language override of this font face.
    pub language_override: Option<FontLanguageOverride>,
}

impl ToCss for FontFaceAtRule {
    fn to_css<W: fmt::Write>(&self, dest: &mut W) -> fmt::Result {
        #[inline(always)]
        fn writePropertyDeclaration<W: fmt::Write, T: ToCss>(
            afterFirst: &mut bool,
            dest: &mut W,
            name: &str,
            value: &Option<T>,
        ) -> fmt::Result {
            if let &Some(ref value) = value {
                if *afterFirst {
                    dest.write_char(';')?;
                } else {
                    *afterFirst = true;
                }

                dest.write_str(name)?;
                dest.write_char(':')?;
                value.to_css(dest)
            } else {
                Ok(())
            }
        }

        #[inline(always)]
        fn writePropertyDeclarationValues<W: fmt::Write, T: ToCss>(
            afterFirst: &mut bool,
            dest: &mut W,
            name: &str,
            value: &Option<Vec<T>>,
        ) -> fmt::Result {
            if let &Some(ref value) = value {
                if value.is_empty() {
                    return Ok(());
                }

                if *afterFirst {
                    dest.write_char(';')?;
                } else {
                    *afterFirst = true;
                }

                dest.write_str(name)?;
                dest.write_char(':')?;

                let mut iterator = value.iter();
                iterator.next().unwrap().to_css(dest)?;
                for value in iterator {
                    dest.write_char(',')?;
                    value.to_css(dest)?;
                }
            }

            Ok(())
        }

        dest.write_str("@font-face{")?;

        let mut afterFirst = false;
        writePropertyDeclaration(
            &mut afterFirst,
            dest,
            "font-family",
            &self.family,
        )?;
        writePropertyDeclarationValues(
            &mut afterFirst,
            dest,
            "src",
            &self.sources,
        )?;
        writePropertyDeclaration(
            &mut afterFirst,
            dest,
            "font-style",
            &self.style,
        )?;
        writePropertyDeclaration(
            &mut afterFirst,
            dest,
            "font-weight",
            &self.weight,
        )?;
        writePropertyDeclaration(
            &mut afterFirst,
            dest,
            "font-stretch",
            &self.stretch,
        )?;
        writePropertyDeclaration(
            &mut afterFirst,
            dest,
            "font-display",
            &self.display,
        )?;
        writePropertyDeclarationValues(
            &mut afterFirst,
            dest,
            "unicode-range",
            &self.unicode_range,
        )?;
        writePropertyDeclaration(
            &mut afterFirst,
            dest,
            "font-feature-settings",
            &self.feature_settings,
        )?;
        writePropertyDeclaration(
            &mut afterFirst,
            dest,
            "font-language-override",
            &self.language_override,
        )?;

        dest.write_char('}')
    }
}

impl FontFaceAtRule {
    #[inline(always)]
    fn empty() -> Self {
        Self {
            family: None,
            sources: None,
            style: None,
            weight: None,
            stretch: None,
            display: None,
            unicode_range: None,
            feature_settings: None,
            language_override: None,
        }
    }

    /// Parse the block inside a `@font-face` rule.
    pub(crate) fn parse_body<'i: 't, 't>(
        context: &ParserContext,
        input: &mut Parser<'i, 't>,
    ) -> Result<FontFaceAtRule, ParseError<'i, CustomParseError<'i>>> {
        let mut rule = Self::empty();

        {
            let parser = FontFaceAtRuleParser {
                context,
                rule: &mut rule,
            };

            let iter = DeclarationListParser::new(input, parser);
            for declaration in iter {
                if declaration.is_err() {
                    return Err(declaration.unwrap_err().0);
                }
            }
        }

        Ok(rule)
    }

    /// Per <https://github.com/w3c/csswg-drafts/issues/1133> an @font-face rule is valid as far as the CSS parser is concerned even if it doesn't have a font-family or src declaration.
    ///
    /// However both are required for the rule to represent an actual font face.
    pub fn font_face(&self) -> Option<FontFace> {
        if self.family.is_some() && self.sources.is_some() {
            Some(FontFace(self))
        } else {
            None
        }
    }
}
