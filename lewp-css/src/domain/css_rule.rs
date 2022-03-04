// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.

use {
    super::{
        at_rules::{
            counter_style::CounterStyleAtRule,
            document::DocumentAtRule,
            font_face::FontFaceAtRule,
            font_feature_values::FontFeatureValuesAtRule,
            import::ImportAtRule,
            keyframes::KeyframesAtRule,
            media::MediaAtRule,
            namespace::NamespaceAtRule,
            page::PageAtRule,
            supports::SupportsAtRule,
            viewport::ViewportAtRule,
        },
        CssRuleType,
        StyleRule,
    },
    cssparser::ToCss,
    std::fmt,
};

/// No Charset here, CSSCharsetRule has been removed from CSSOM (<https://drafts.csswg.org/cssom/#changes-from-5-december-2013>) and Edge doesn't support it
#[derive(Debug, Clone)]
pub enum CssRule {
    /// @counter-style
    CounterStyle(CounterStyleAtRule),

    /// @document
    Document(DocumentAtRule),

    /// @font-face
    FontFace(FontFaceAtRule),

    /// @font-feature-values
    FontFeatureValues(FontFeatureValuesAtRule),

    /// @import
    Import(ImportAtRule),

    /// @keyframes
    Keyframes(KeyframesAtRule),

    /// @media
    Media(MediaAtRule),

    /// @namespace
    Namespace(NamespaceAtRule),

    /// @page
    Page(PageAtRule),

    /// Style rules, eg `div { width: 10%; }`
    Style(StyleRule),

    /// @supports
    Supports(SupportsAtRule),

    /// @viewport
    Viewport(ViewportAtRule),
}

impl ToCss for CssRule {
    /// <https://drafts.csswg.org/cssom/#serialize-a-css-rule>
    fn to_css<W: fmt::Write>(&self, dest: &mut W) -> fmt::Result {
        use self::CssRule::*;

        match *self {
            CounterStyle(ref rule) => rule.to_css(dest),

            Document(ref rule) => rule.to_css(dest),

            FontFace(ref rule) => rule.to_css(dest),

            FontFeatureValues(ref rule) => rule.to_css(dest),

            Import(ref rule) => rule.to_css(dest),

            Keyframes(ref rule) => rule.to_css(dest),

            Media(ref rule) => rule.to_css(dest),

            Namespace(ref rule) => rule.to_css(dest),

            Page(ref rule) => rule.to_css(dest),

            Style(ref rule) => rule.to_css(dest),

            Supports(ref rule) => rule.to_css(dest),

            Viewport(ref rule) => rule.to_css(dest),
        }
    }
}

impl CssRule {
    /// Returns the CSSOM rule type of this rule.
    #[inline(always)]
    pub fn rule_type(&self) -> CssRuleType {
        use self::CssRule::*;

        match *self {
            CounterStyle(_) => CssRuleType::CounterStyle,

            Document(_) => CssRuleType::Document,

            FontFace(_) => CssRuleType::FontFace,

            FontFeatureValues(_) => CssRuleType::FontFeatureValues,

            Import(_) => CssRuleType::Import,

            Keyframes(_) => CssRuleType::Keyframes,

            Media(_) => CssRuleType::Media,

            Namespace(_) => CssRuleType::Namespace,

            Page(_) => CssRuleType::Page,

            Style(_) => CssRuleType::Style,

            Supports(_) => CssRuleType::Supports,

            Viewport(_) => CssRuleType::Viewport,
        }
    }
}
