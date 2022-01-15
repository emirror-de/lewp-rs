// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.

use {
    super::{
        keyframe_selector_parser_prelude::KeyframeSelectorParserPrelude,
        ParserContext,
    },
    crate::{
        domain::{
            at_rules::keyframes::{Keyframe, KeyframeSelector},
            properties::PropertyDeclarations,
            CssRuleType,
        },
        CustomParseError,
    },
    cssparser::{
        AtRuleParser,
        ParseError,
        Parser,
        ParserState,
        QualifiedRuleParser,
        RuleListParser,
    },
};

/// Parses a keyframes list, like:
/// 0%, 50% {
///     width: 50%;
/// }
///
/// 40%, 60%, 100% {
///     width: 100%;
/// }
pub(crate) struct KeyframeListParser<'a> {
    context: &'a ParserContext,
}

// Default methods reject all @ rules.
impl<'a, 'i> AtRuleParser<'i> for KeyframeListParser<'a> {
    type Prelude = ();

    type AtRule = Keyframe;

    type Error = CustomParseError<'i>;
}

impl<'a, 'i> QualifiedRuleParser<'i> for KeyframeListParser<'a> {
    type Prelude = KeyframeSelectorParserPrelude;

    type QualifiedRule = Keyframe;

    type Error = CustomParseError<'i>;

    fn parse_prelude<'t>(
        &mut self,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self::Prelude, ParseError<'i, CustomParseError<'i>>> {
        match KeyframeSelector::parse(input) {
            Ok(selector) => Ok(KeyframeSelectorParserPrelude { selector }),

            Err(error) => Err(error),
        }
    }

    fn parse_block<'t>(
        &mut self,
        prelude: Self::Prelude,
        _start: &ParserState,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self::QualifiedRule, ParseError<'i, CustomParseError<'i>>> {
        let context = ParserContext::new_with_rule_type(
            self.context,
            CssRuleType::Keyframe,
        );

        Ok(Keyframe {
            selector: prelude.selector,
            property_declarations:
                PropertyDeclarations::parse_property_declaration_list(
                    &context, input,
                )?,
        })
    }
}

impl<'a> KeyframeListParser<'a> {
    /// Parses a keyframe list from CSS input.
    pub(crate) fn parse_keyframe_list<'i: 't, 't>(
        context: &ParserContext,
        input: &mut Parser<'i, 't>,
    ) -> Result<Vec<Keyframe>, ParseError<'i, CustomParseError<'i>>> {
        let iter = RuleListParser::new_for_nested_rule(
            input,
            KeyframeListParser { context },
        );

        let mut keyframes = Vec::new();

        for keyframe in iter {
            match keyframe {
                Ok(keyframe) => keyframes.push(keyframe),

                Err(preciseParseError) => return Err(preciseParseError.0),
            }
        }

        Ok(keyframes)
    }
}
