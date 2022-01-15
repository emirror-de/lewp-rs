// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.

use {
    super::{
        at_rules::VendorPrefixedAtRule,
        CssRule::{self},
        HasCssRules,
        RulesMutateError::{self},
    },
    cssparser::ToCss,
    std::fmt,
};

/// A list of CSS rules.
#[derive(Default, Debug, Clone)]
pub struct CssRules(pub Vec<CssRule>);

impl ToCss for CssRules {
    fn to_css<W: fmt::Write>(&self, dest: &mut W) -> fmt::Result {
        for cssRule in self.0.iter() {
            cssRule.to_css(dest)?;
        }

        Ok(())
    }
}

impl HasCssRules for CssRules {
    #[inline(always)]
    fn css_rules(&self) -> &CssRules {
        self
    }

    #[inline(always)]
    fn css_rules_mut(&mut self) -> &mut CssRules {
        self
    }

    #[inline(always)]
    fn css_rules_slice(&self) -> &[CssRule] {
        &self.0[..]
    }

    #[inline(always)]
    fn css_rules_vec(&self) -> &Vec<CssRule> {
        &self.0
    }

    #[inline(always)]
    fn css_rules_vec_mut(&mut self) -> &mut Vec<CssRule> {
        &mut self.0
    }
}

impl CssRules {
    /// Allows vendor prefixing of at-rules
    #[inline(always)]
    pub fn vendor_prefix_at_rules<
        AtRule: VendorPrefixedAtRule,
        CssRuleMatcher: Fn(&CssRule) -> Option<&AtRule>,
        VendorPrefixer: Fn(usize, &AtRule) -> Vec<CssRule>,
    >(
        &mut self,
        remove_unprefixed_at_rule: bool,
        css_rule_matcher: CssRuleMatcher,
        vendor_prefixer: VendorPrefixer,
    ) {
        let mut index = 0;
        while index < self.0.len() {
            let newCssRulesToInsert = match css_rule_matcher(unsafe {
                self.0.get_unchecked(index)
            }) {
                None => None,
                Some(atRule) => {
                    if atRule.isNotVendorPrefixed() {
                        Some(vendor_prefixer(index, atRule))
                    } else {
                        None
                    }
                }
            };

            index += if let Some(mut newCssRulesToInsert) = newCssRulesToInsert
            {
                let indexIncrement = newCssRulesToInsert.len();

                // TODO: Inefficient
                for newCssRuleToInsert in newCssRulesToInsert.drain(..) {
                    self.css_rules_vec_mut().insert(index, newCssRuleToInsert);
                }
                if remove_unprefixed_at_rule {
                    self.css_rules_vec_mut().remove(index + indexIncrement);
                    indexIncrement
                } else {
                    indexIncrement + 1
                }
            } else {
                1
            };
        }
    }

    /// Whether this CSS rules is empty.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Returns whether all the rules in this list are namespace or import rules.
    fn only_namespace_or_import(&self) -> bool {
        use self::CssRule::*;

        self.0.iter().all(|r| match *r {
            Namespace(..) | Import(..) => true,
            _ => false,
        })
    }

    /// <https://drafts.csswg.org/cssom/#remove-a-css-rule>
    pub fn remove_rule(
        &mut self,
        index: usize,
    ) -> Result<(), RulesMutateError> {
        use self::{CssRule::Namespace, RulesMutateError::*};

        // Step 1, 2
        if index >= self.0.len() {
            return Err(IndexSize);
        }

        {
            // Step 3
            let rule = &self.0[index];

            // Step 4
            if let Namespace(..) = *rule {
                if !self.only_namespace_or_import() {
                    return Err(InvalidState);
                }
            }
        }

        // Step 5, 6
        self.0.remove(index);

        Ok(())
    }
}
