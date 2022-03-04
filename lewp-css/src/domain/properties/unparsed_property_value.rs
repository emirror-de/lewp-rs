// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.

use {
    crate::domain::properties::{CssWideKeyword, SpecifiedValue},
    cssparser::ToCss,
    std::fmt,
};

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum UnparsedPropertyValue {
    CssWideKeyword(CssWideKeyword),
    SpecifiedValue(SpecifiedValue),
}

impl ToCss for UnparsedPropertyValue {
    #[inline(always)]
    fn to_css<W: fmt::Write>(&self, dest: &mut W) -> fmt::Result {
        use self::UnparsedPropertyValue::*;

        match *self {
            CssWideKeyword(cssWideKeyWord) => cssWideKeyWord.to_css(dest),
            SpecifiedValue(ref specifiedValue) => specifiedValue.to_css(dest),
        }
    }
}
