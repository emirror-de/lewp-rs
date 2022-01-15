// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.

use {
    super::SupportsCondition,
    crate::domain::{CssRule, CssRules, HasCssRules},
    cssparser::ToCss,
    std::fmt,
};

/// An [`@supports`][supports] rule.
///
/// [supports]: <https://drafts.csswg.org/css-conditional-3/#at-supports>
#[derive(Debug, Clone)]
pub struct SupportsAtRule {
    /// The parsed condition
    pub condition: SupportsCondition,

    /// Child rules
    pub rules: CssRules,
}

impl HasCssRules for SupportsAtRule {
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

impl ToCss for SupportsAtRule {
    fn to_css<W: fmt::Write>(&self, dest: &mut W) -> fmt::Result {
        dest.write_str("@supports ")?;
        self.condition.to_css(dest)?;
        dest.write_char('{')?;
        self.rules.to_css(dest)?;
        dest.write_char('}')
    }
}
