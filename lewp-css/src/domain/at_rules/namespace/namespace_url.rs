// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.

use {
    crate::domain::Atom,
    cssparser::ToCss,
    precomputed_hash::PrecomputedHash,
    std::{borrow::Borrow, fmt},
};

#[derive(Default, Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct NamespaceUrl(pub Atom);

impl ToCss for NamespaceUrl {
    // https://drafts.csswg.org/cssom/#serialize-a-css-rule CSSNamespaceRule
    fn to_css<W: fmt::Write>(&self, dest: &mut W) -> fmt::Result {
        self.0.to_css(dest)
    }
}

impl Borrow<str> for NamespaceUrl {
    #[inline(always)]
    fn borrow(&self) -> &str {
        self.0.borrow()
    }
}

impl PrecomputedHash for NamespaceUrl {
    fn precomputed_hash(&self) -> u32 {
        self.0.precomputed_hash()
    }
}
