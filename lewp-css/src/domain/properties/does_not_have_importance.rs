// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.

use {
    super::{HasImportance, Importance},
    crate::CustomParseError,
    cssparser::{ParseError, ToCss},
    std::fmt,
};

/// A type representing that !important can not be present in a property declaration
#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct DoesNotHaveImportance;

impl ToCss for DoesNotHaveImportance {
    #[inline(always)]
    fn to_css<W: fmt::Write>(&self, _dest: &mut W) -> fmt::Result {
        Ok(())
    }
}

impl HasImportance for DoesNotHaveImportance {
    #[inline(always)]
    fn validateParsedImportance<'i>(
        importance: Importance,
    ) -> Result<Self, ParseError<'i, CustomParseError<'i>>> {
        if importance.isImportant() {
            Err(ParseError::from(
                CustomParseError::ImportantIsNotAllowedInKeyframePropertyDeclarationValues,
            ))
        } else {
            Ok(DoesNotHaveImportance)
        }
    }

    /// Return whether this is an important declaration.
    #[inline(always)]
    fn isImportant(&self) -> bool {
        false
    }
}
