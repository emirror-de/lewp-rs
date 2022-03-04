// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.

use {
    super::{
        FontFeatureValuesBlockType,
        FontFeatureValuesDeclarationsParser,
        Parse,
        ParserContext,
    },
    crate::{
        domain::at_rules::font_feature_values::{
            FontFeatureValuesAtRule,
            FontFeatureValuesDeclaration,
        },
        CustomParseError,
    },
    cssparser::{
        AtRuleParser,
        BasicParseErrorKind,
        CowRcStr,
        ParseError,
        ParseErrorKind,
        Parser,
        ParserState,
        QualifiedRuleParser,
        ToCss,
    },
};

/// Parser for `FontFeatureValuesRule`. Parses all blocks
/// <feature-type> {
///   <feature-value-declaration-list>
/// }
/// <feature-type> = @stylistic | @historical-forms | @styleset |
/// @character-variant | @swash | @ornaments | @annotation
pub(crate) struct FontFeatureValuesAtRuleParser<'a> {
    pub(crate) context: &'a ParserContext,
    pub(crate) rule: &'a mut FontFeatureValuesAtRule,
}

/// Default methods reject all qualified rules.
impl<'a, 'i> QualifiedRuleParser<'i> for FontFeatureValuesAtRuleParser<'a> {
    type Prelude = ();

    type QualifiedRule = ();

    type Error = CustomParseError<'i>;
}

impl<'a, 'i> AtRuleParser<'i> for FontFeatureValuesAtRuleParser<'a> {
    type Prelude = FontFeatureValuesBlockType;

    type AtRule = ();

    type Error = CustomParseError<'i>;

    fn parse_prelude<'t>(
        &mut self,
        name: CowRcStr<'i>,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self::Prelude, ParseError<'i, Self::Error>> {
        match_ignore_ascii_case! {
            &*name,

            "swash" => Ok(FontFeatureValuesBlockType::swash),

            "stylistic" => Ok(FontFeatureValuesBlockType::stylistic),

            "ornaments" => Ok(FontFeatureValuesBlockType::ornaments),

            "annotation" => Ok(FontFeatureValuesBlockType::annotation),

            "character-variant" => Ok(FontFeatureValuesBlockType::character_variant),

            "styleset" => Ok(FontFeatureValuesBlockType::styleset),

            _ => Err(ParseError {
                kind: ParseErrorKind::Basic(BasicParseErrorKind::AtRuleBodyInvalid),
                location: input.state().source_location(),
            }),
        }
    }

    fn parse_block<'t>(
        &mut self,
        prelude: Self::Prelude,
        _start: &ParserState,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self::AtRule, ParseError<'i, CustomParseError<'i>>> {
        use self::FontFeatureValuesBlockType::*;

        match prelude {
            swash => {
                Self::parseBlock(self.context, input, &mut self.rule.swash)
            }
            stylistic => {
                Self::parseBlock(self.context, input, &mut self.rule.stylistic)
            }
            ornaments => {
                Self::parseBlock(self.context, input, &mut self.rule.ornaments)
            }
            annotation => {
                Self::parseBlock(self.context, input, &mut self.rule.annotation)
            }
            character_variant => Self::parseBlock(
                self.context,
                input,
                &mut self.rule.character_variant,
            ),
            styleset => {
                Self::parseBlock(self.context, input, &mut self.rule.styleset)
            }
        }
    }
}

impl<'a> FontFeatureValuesAtRuleParser<'_> {
    #[inline(always)]
    fn parseBlock<'i, 't, T: 'a + ToCss + Parse>(
        context: &ParserContext,
        input: &mut Parser<'i, 't>,
        declarations: &'a mut Vec<FontFeatureValuesDeclaration<T>>,
    ) -> Result<(), ParseError<'i, CustomParseError<'i>>> {
        FontFeatureValuesDeclarationsParser::parseBlock(
            input,
            context,
            declarations,
        )
    }
}
