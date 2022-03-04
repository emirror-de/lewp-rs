// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.

use {
    super::MediaList,
    crate::domain::{CssRule, CssRules, HasCssRules},
    cssparser::ToCss,
    std::fmt,
};

/// An [`@media`][media] url.
///
/// [media]: <https://drafts.csswg.org/css-conditional/#at-ruledef-media>
#[derive(Debug, Clone)]
pub struct MediaAtRule {
    /// The list of media queries used by this media rule.
    pub media_queries: MediaList,

    /// The nested rules to this media rule.
    pub rules: CssRules,
}

impl HasCssRules for MediaAtRule {
    #[inline(always)]
    fn css_rules(&self) -> &CssRules {
        &self.rules
    }

    #[inline(always)]
    fn css_rules_mut(&mut self) -> &mut CssRules {
        &mut self.rules
    }

    #[inline(always)]
    fn css_rules_slice(&self) -> &[CssRule] {
        &self.rules.0[..]
    }

    #[inline(always)]
    fn css_rules_vec(&self) -> &Vec<CssRule> {
        &self.rules.0
    }

    #[inline(always)]
    fn css_rules_vec_mut(&mut self) -> &mut Vec<CssRule> {
        &mut self.rules.0
    }
}

impl ToCss for MediaAtRule {
    // https://drafts.csswg.org/cssom/#serialize-a-css-rule CSSMediaRule
    fn to_css<W: fmt::Write>(&self, dest: &mut W) -> fmt::Result {
        dest.write_str("@media ")?;
        self.media_queries.to_css(dest)?;
        dest.write_char('{')?;
        self.rules.to_css(dest)?;
        dest.write_char('}')
    }
}
