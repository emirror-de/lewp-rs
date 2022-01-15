// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.

use {
    crate::domain::Atom,
    cssparser::ToCss,
    std::fmt::{self, Display, Formatter},
};

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct NamespacePrefix(pub Atom);

impl ToCss for NamespacePrefix {
    // https://drafts.csswg.org/cssom/#serialize-a-css-rule CSSNamespaceRule
    fn to_css<W: fmt::Write>(&self, dest: &mut W) -> fmt::Result {
        self.0.to_css(dest)
    }
}

impl Default for NamespacePrefix {
    #[inline(always)]
    fn default() -> Self {
        NamespacePrefix(Atom::default())
    }
}

impl Display for NamespacePrefix {
    #[inline(always)]
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl From<String> for NamespacePrefix {
    #[inline(always)]
    fn from(value: String) -> Self {
        NamespacePrefix(Atom::from(value))
    }
}

impl<'a> From<&'a str> for NamespacePrefix {
    #[inline(always)]
    fn from(value: &'a str) -> Self {
        NamespacePrefix(Atom::from(value))
    }
}
