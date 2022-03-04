// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.

use {
    super::{Document, DocumentCondition},
    crate::domain::{
        at_rules::VendorPrefixedAtRule,
        CssRule,
        CssRules,
        HasCssRules,
        HasVendorPrefix,
        VendorPrefix,
    },
    cssparser::ToCss,
    std::fmt,
};

/// A @document rule
#[derive(Debug, Clone)]
pub struct DocumentAtRule {
    pub vendor_prefix: Option<VendorPrefix>,

    /// The parsed condition
    pub condition: DocumentCondition,

    /// Child rules
    pub rules: CssRules,
}

impl HasCssRules for DocumentAtRule {
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

impl ToCss for DocumentAtRule {
    fn to_css<W: fmt::Write>(&self, dest: &mut W) -> fmt::Result {
        dest.write_str("@")?;
        if let Some(ref vendor_prefix) = self.vendor_prefix {
            vendor_prefix.to_css(dest)?;
        }
        dest.write_str("document ")?;
        self.condition.to_css(dest)?;
        dest.write_char('{')?;
        self.rules.to_css(dest)?;
        dest.write_char('}')
    }
}

impl HasVendorPrefix for DocumentAtRule {
    #[inline(always)]
    fn isNotVendorPrefixed(&self) -> bool {
        self.vendor_prefix.is_none()
    }
}

impl VendorPrefixedAtRule for DocumentAtRule {}

impl DocumentAtRule {
    /// Evaluate a document condition.
    pub fn evaluate<D: Document>(&self, document: &D) -> bool {
        self.condition.evaluate(document)
    }
}
