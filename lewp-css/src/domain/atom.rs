// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.

use {
    cssparser::{serialize_identifier, CowRcStr, ToCss},
    precomputed_hash::PrecomputedHash,
    std::{
        borrow::{Borrow, Cow},
        collections::hash_map::DefaultHasher,
        fmt::{self, Display, Formatter},
        hash::{Hash, Hasher},
        ops::Deref,
        str::FromStr,
    },
};

/// NOTE: At some future point, Atom may become a wrapper around a string cache value
#[derive(Default, Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Atom(pub String);

impl Deref for Atom {
    type Target = str;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl ToCss for Atom {
    #[inline(always)]
    fn to_css<W: fmt::Write>(&self, dest: &mut W) -> fmt::Result {
        serialize_identifier(&self.0, dest)
    }
}

impl Display for Atom {
    #[inline(always)]
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl From<String> for Atom {
    #[inline(always)]
    fn from(value: String) -> Self {
        Atom(value)
    }
}

impl<'a> From<&'a str> for Atom {
    #[inline(always)]
    fn from(value: &'a str) -> Self {
        Atom(value.to_owned())
    }
}

impl<'i> From<CowRcStr<'i>> for Atom {
    #[inline(always)]
    fn from(value: CowRcStr<'i>) -> Self {
        Atom::from(value.as_ref())
    }
}

impl<'a> From<Cow<'a, str>> for Atom {
    #[inline(always)]
    fn from(value: Cow<'a, str>) -> Self {
        Atom(value.into_owned())
    }
}

impl<'a, 'i> From<&'a CowRcStr<'i>> for Atom {
    #[inline(always)]
    fn from(value: &'a CowRcStr<'i>) -> Self {
        Atom::from(value.as_ref())
    }
}

impl FromStr for Atom {
    type Err = ();

    #[inline(always)]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Atom(s.to_owned()))
    }
}

impl Borrow<str> for Atom {
    #[inline(always)]
    fn borrow(&self) -> &str {
        self.deref()
    }
}

impl PrecomputedHash for Atom {
    #[inline(always)]
    fn precomputed_hash(&self) -> u32 {
        let mut state = DefaultHasher::new();
        self.0.hash(&mut state);
        state.finish() as u32
    }
}

impl Atom {
    #[inline(always)]
    pub fn is_ascii(&self) -> bool {
        self.0.is_ascii()
    }

    #[inline(always)]
    pub fn eq_ignore_ascii_case(&self, name: &str) -> bool {
        self.deref().eq_ignore_ascii_case(name)
    }
}
