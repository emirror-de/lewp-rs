// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.

use {
    super::{
        qualified_rule_parser_prelude::QualifiedRuleParserPrelude,
        AtRuleBlockPrelude::{self, *},
        KeyframeListParser,
        OurSelectorParser,
        ParserContext,
    },
    crate::{
        domain::{
            at_rules::{
                counter_style::CounterStyleAtRule,
                document::{DocumentAtRule, DocumentCondition},
                font_face::{FamilyName, FontFaceAtRule},
                font_feature_values::FontFeatureValuesAtRule,
                keyframes::{KeyframesAtRule, KeyframesName},
                media::{MediaAtRule, MediaList},
                namespace::Namespaces,
                page::{PageAtRule, PageSelectorPseudoClass},
                supports::{SupportsAtRule, SupportsCondition},
                viewport::ViewportAtRule,
            },
            properties::PropertyDeclarations,
            CounterStyleIdent,
            CssRule,
            CssRuleType,
            CssRules,
            StyleRule,
            VendorPrefix::*,
        },
        CustomParseError,
    },
    cssparser::{
        AtRuleParser,
        CowRcStr,
        ParseError,
        Parser,
        ParserState,
        QualifiedRuleParser,
        RuleListParser,
    },
    std::{collections::HashMap, rc::Rc},
};

#[derive(Clone)]
pub(crate) struct NestedRuleParser<'a> {
    pub context: &'a ParserContext,
    pub namespaces: Rc<Namespaces>,
}

impl<'a, 'i> AtRuleParser<'i> for NestedRuleParser<'a> {
    type Prelude = AtRuleBlockPrelude;

    type AtRule = CssRule;

    type Error = CustomParseError<'i>;

    fn parse_prelude<'t>(
        &mut self,
        name: CowRcStr<'i>,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self::Prelude, ParseError<'i, Self::Error>> {
        match_ignore_ascii_case! {
            &name,

            "counter-style" => Ok(Self::Prelude::CounterStyle(CounterStyleIdent::parseForCounterStyleAtRule(input)?)),

            "document" => Ok(Self::Prelude::Document(None, DocumentCondition::parse(self.context, input)?)),

            "-moz-document" => Ok(Self::Prelude::Document(Some(moz), DocumentCondition::parse(self.context, input)?)),

            "font-face" => Ok(Self::Prelude::FontFace),

            "font-feature-values" => Ok(Self::Prelude::FontFeatureValues(FamilyName::parse_family_name_list(self.context, input)?)),

            "keyframes" => Ok(Self::Prelude::Keyframes(None, KeyframesName::parse(input)?)),

            "-webkit-keyframes" => Ok(Self::Prelude::Keyframes(Some(webkit), KeyframesName::parse(input)?)),

            "-moz-keyframes" => Ok(Self::Prelude::Keyframes(Some(moz), KeyframesName::parse(input)?)),

            "media" => Ok(Self::Prelude::Media(MediaList::parse_media_query_list(self.context, input, false)?)),

            "page" => Ok(Self::Prelude::Page(PageSelectorPseudoClass::parse(input)?)),

            "supports" => Ok(Self::Prelude::Supports(SupportsCondition::parse(input)?)),

            "viewport" => Ok(Self::Prelude::Viewport(None)),

            "-ms-viewport" => Ok(Self::Prelude::Viewport(Some(ms))),

            "-o-viewport" => Ok(Self::Prelude::Viewport(Some(o))),

            _ => Err(ParseError::from(CustomParseError::UnsupportedAtRule(name.clone())))
        }
    }

    fn parse_block<'t>(
        &mut self,
        prelude: AtRuleBlockPrelude,
        _start: &ParserState,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self::AtRule, ParseError<'i, Self::Error>> {
        let cssRule = match prelude {
            CounterStyle(name) => {
                CssRule::CounterStyle(CounterStyleAtRule::parse_body(
                    name,
                    &CssRuleType::CounterStyle.context(self),
                    input,
                )?)
            }

            Document(vendor_prefix, condition) => {
                CssRule::Document(DocumentAtRule {
                    vendor_prefix,
                    condition,
                    rules: self
                        .parse_nested_rules(input, CssRuleType::Document)?,
                })
            }

            FontFace => CssRule::FontFace(FontFaceAtRule::parse_body(
                &CssRuleType::FontFace.context(self),
                input,
            )?),

            FontFeatureValues(family_names) => {
                CssRule::FontFeatureValues(FontFeatureValuesAtRule::parse_body(
                    &CssRuleType::FontFeatureValues.context(self),
                    input,
                    family_names,
                )?)
            }

            Import(rule) => CssRule::Import(rule),

            Keyframes(vendor_prefix, name) => {
                CssRule::Keyframes(KeyframesAtRule {
                    vendor_prefix,
                    name,
                    keyframes: KeyframeListParser::parse_keyframe_list(
                        &CssRuleType::Keyframes.context(self),
                        input,
                    )?,
                })
            }

            Media(media_queries) => CssRule::Media(MediaAtRule {
                media_queries,
                rules: self.parse_nested_rules(input, CssRuleType::Media)?,
            }),

            Namespace(rule) => CssRule::Namespace(rule),

            Page(page_selector_pseudo_class) => CssRule::Page(PageAtRule {
                page_selector_pseudo_class,
                property_declarations:
                    PropertyDeclarations::parse_property_declaration_list(
                        &CssRuleType::Page.context(self),
                        input,
                    )?,
            }),

            Supports(condition) => CssRule::Supports(SupportsAtRule {
                condition,
                rules: self.parse_nested_rules(input, CssRuleType::Supports)?,
            }),

            Viewport(vendor_prefix) => {
                CssRule::Viewport(ViewportAtRule::parse_body(
                    vendor_prefix,
                    &CssRuleType::Viewport.context(self),
                    input,
                )?)
            }
        };

        Ok(cssRule)
    }
}

impl<'a, 'i> QualifiedRuleParser<'i> for NestedRuleParser<'a> {
    type Prelude = QualifiedRuleParserPrelude;

    type QualifiedRule = CssRule;

    type Error = CustomParseError<'i>;

    fn parse_prelude<'t>(
        &mut self,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self::Prelude, ParseError<'i, Self::Error>> {
        let applyVendorPrefixToPseudoClasses = HashMap::default();
        let applyVendorPrefixToPseudoElements = HashMap::default();
        let ourSelectorParser = OurSelectorParser {
            namespaces: self.namespaces.clone(),
            applyVendorPrefixToPseudoClasses: &applyVendorPrefixToPseudoClasses,
            applyVendorPrefixToPseudoElements:
                &applyVendorPrefixToPseudoElements,
        };

        let selectors = ourSelectorParser.parse(input)?;

        Ok(QualifiedRuleParserPrelude { selectors })
    }

    fn parse_block<'t>(
        &mut self,
        prelude: Self::Prelude,
        _start: &ParserState,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self::QualifiedRule, ParseError<'i, Self::Error>> {
        let context =
            ParserContext::new_with_rule_type(self.context, CssRuleType::Style);

        let styleRule = StyleRule {
            selectors: prelude.selectors,
            property_declarations:
                PropertyDeclarations::parse_property_declaration_list(
                    &context, input,
                )?,
        };

        Ok(CssRule::Style(styleRule))
    }
}

impl<'a> NestedRuleParser<'a> {
    #[inline(always)]
    pub(crate) fn context_new_with_rule_type(
        &self,
        cssRuleType: CssRuleType,
    ) -> ParserContext {
        ParserContext::new_with_rule_type(self.context, cssRuleType)
    }

    fn parse_nested_rules<'i: 't, 't>(
        &mut self,
        input: &mut Parser<'i, 't>,
        rule_type: CssRuleType,
    ) -> Result<CssRules, ParseError<'i, CustomParseError<'i>>> {
        let context = rule_type.context(self);

        let nested_parser = NestedRuleParser {
            context: &context,
            namespaces: self.namespaces.clone(),
        };

        let iter =
            RuleListParser::new_for_nested_rule(input, nested_parser);
        let mut rules = Vec::new();
        for result in iter {
            match result {
                Ok(rule) => rules.push(rule),
                Err(preciseParseError) => return Err(preciseParseError.0),
            }
        }
        Ok(CssRules(rules))
    }
}
