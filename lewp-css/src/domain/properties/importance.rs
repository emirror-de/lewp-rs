// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.

use {
    super::HasImportance,
    crate::CustomParseError,
    cssparser::{parse_important, ParseError, Parser, ToCss},
    std::fmt,
    Importance::*,
};

/// A declaration [importance][importance].
///
/// [importance]: <https://drafts.csswg.org/css-cascade/#importance>
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum Importance {
    /// Indicates a declaration without `!important`.
    Normal,

    /// Indicates a declaration with `!important`.
    Important,
}

impl Default for Importance {
    #[inline(always)]
    fn default() -> Self {
        Importance::Normal
    }
}

impl ToCss for Importance {
    fn to_css<W: fmt::Write>(&self, dest: &mut W) -> fmt::Result {
        match *self {
            Normal => Ok(()),
            Important => dest.write_str("!important"),
        }
    }
}

impl HasImportance for Importance {
    #[inline(always)]
    fn validateParsedImportance<'i>(
        importance: Importance,
    ) -> Result<Self, ParseError<'i, CustomParseError<'i>>> {
        Ok(importance)
    }

    /// Return whether this is an important declaration.
    #[inline(always)]
    fn isImportant(&self) -> bool {
        match *self {
            Normal => false,
            Important => true,
        }
    }
}

impl Importance {
    #[inline(always)]
    pub fn from_bool(isImportant: bool) -> Self {
        if isImportant {
            Important
        } else {
            Normal
        }
    }

    #[inline(always)]
    pub(crate) fn parse<'i, 't>(input: &mut Parser<'i, 't>) -> Self {
        Self::from_bool(input.r#try(parse_important).is_ok())
    }
}
