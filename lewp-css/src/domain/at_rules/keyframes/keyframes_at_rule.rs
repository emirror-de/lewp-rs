// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.

use {
    super::{Keyframe, KeyframeSelector, KeyframesName},
    crate::domain::{
        at_rules::VendorPrefixedAtRule,
        HasVendorPrefix,
        VendorPrefix,
    },
    cssparser::ToCss,
    std::fmt,
};

/// A [`@keyframes`](crate::domain::CssRule::Keyframes) rule.
/// Keyframes: <https://drafts.csswg.org/css-animations/#keyframes>
#[derive(Debug, Clone)]
pub struct KeyframesAtRule {
    /// Vendor prefix type the @keyframes has.
    pub vendor_prefix: Option<VendorPrefix>,

    /// The name of the current animation.
    pub name: KeyframesName,

    /// The keyframes specified for this CSS rule.
    pub keyframes: Vec<Keyframe>,
}

impl ToCss for KeyframesAtRule {
    fn to_css<W: fmt::Write>(&self, dest: &mut W) -> fmt::Result {
        dest.write_str("@")?;
        if let Some(ref vendor_prefix) = self.vendor_prefix {
            vendor_prefix.to_css(dest)?;
        }
        dest.write_str("keyframes ")?;
        self.name.to_css(dest)?;
        dest.write_char('{')?;
        for keyframe in self.keyframes.iter() {
            keyframe.to_css(dest)?;
        }
        dest.write_char('}')
    }
}

impl HasVendorPrefix for KeyframesAtRule {
    #[inline(always)]
    fn isNotVendorPrefixed(&self) -> bool {
        self.vendor_prefix.is_none()
    }
}

impl VendorPrefixedAtRule for KeyframesAtRule {}

impl KeyframesAtRule {
    /// Returns the index of the last keyframe that matches the given selector.
    /// If the selector is not valid, or no keyframe is found, returns None.
    ///
    /// Related spec: <https://drafts.csswg.org/css-animations-1/#interface-csskeyframesrule-findrule>
    pub fn find_rule(&self, selector: KeyframeSelector) -> Option<usize> {
        for (index, keyframe) in self.keyframes.iter().enumerate().rev() {
            if keyframe.selector == selector {
                return Some(index);
            }
        }
        None
    }
}
