// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.

use crate::parsers::{NestedRuleParser, ParserContext};

#[allow(missing_docs)]
#[repr(u16)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum CssRuleType {
    /// Obsolete and now reserved
    /// <https://drafts.csswg.org/cssom/#the-cssrule-interface>
    Unknown = 0,

    // <https://drafts.csswg.org/cssom/#the-cssrule-interface>
    Style = 1,
    Charset = 2,
    Import = 3,
    Media = 4,
    FontFace = 5,
    Page = 6,

    // <https://drafts.csswg.org/css-animations-1/#interface-cssrule-idl>
    Keyframes = 7,
    Keyframe = 8,

    // <https://drafts.csswg.org/cssom/#the-cssrule-interface>
    Margin = 9,
    Namespace = 10,

    // <https://drafts.csswg.org/css-counter-styles-3/#extentions-to-cssrule-interface>
    CounterStyle = 11,

    // <https://drafts.csswg.org/css-conditional-3/#extentions-to-cssrule-interface>
    Supports = 12,

    // <https://www.w3.org/TR/2012/WD-css3-conditional-20120911/#extentions-to-cssrule-interface>
    Document = 13,

    // <https://drafts.csswg.org/css-fonts-3/#om-fontfeaturevalues>
    FontFeatureValues = 14,

    // <https://drafts.csswg.org/css-device-adapt/#css-rule-interface>
    Viewport = 15,
}

impl CssRuleType {
    #[inline(always)]
    pub(crate) fn context(
        self,
        nestedRuleParser: &NestedRuleParser,
    ) -> ParserContext {
        nestedRuleParser.context_new_with_rule_type(self)
    }
}
