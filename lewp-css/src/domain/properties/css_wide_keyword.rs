// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.

use {
    cssparser::{Parser, ToCss},
    std::fmt,
};

/// An enum to represent a CSS-wide keyword.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum CssWideKeyword {
    /// The `initial` keyword.
    initial,

    /// The `inherit` keyword.
    inherit,

    /// The `unset` keyword.
    unset,
}

impl ToCss for CssWideKeyword {
    fn to_css<W: fmt::Write>(&self, dest: &mut W) -> fmt::Result {
        use self::CssWideKeyword::*;

        let value = match *self {
            initial => "initial",
            inherit => "inherit",
            unset => "unset",
        };

        dest.write_str(value)
    }
}

impl CssWideKeyword {
    #[inline(always)]
    pub fn to_str(&self) -> &'static str {
        use self::CssWideKeyword::*;

        match *self {
            initial => "initial",
            inherit => "inherit",
            unset => "unset",
        }
    }

    #[inline(always)]
    fn from_ident<'i>(ident: &str) -> Option<Self> {
        use self::CssWideKeyword::*;

        match_ignore_ascii_case! {
            ident,

            "initial" => Some(initial),

            "inherit" => Some(inherit),

            "unset" => Some(unset),

            _ => None
        }
    }

    #[inline(always)]
    pub(crate) fn parse(input: &mut Parser) -> Result<Self, ()> {
        let ident = input.expect_ident().map_err(|_| ())?.clone();
        input.expect_exhausted().map_err(|_| ())?;
        Self::from_ident(&ident).ok_or(())
    }
}
