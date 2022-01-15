// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.

use {
    super::Importance,
    crate::CustomParseError,
    cssparser::{ParseError, ToCss},
    std::{fmt::Debug, hash::Hash},
};

/// A trait representing that !important can or can not be present in a property declaration
pub trait HasImportance:
    ToCss + Sized + Debug + Copy + Clone + Ord + PartialOrd + Eq + PartialEq + Hash
{
    /// Validate and convert importance, if permitted
    fn validateParsedImportance<'i>(
        importance: Importance,
    ) -> Result<Self, ParseError<'i, CustomParseError<'i>>>;

    /// Return whether this is an important declaration.
    fn isImportant(&self) -> bool;
}
