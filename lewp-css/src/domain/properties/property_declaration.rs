// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.

use {
    super::{HasImportance, UnparsedPropertyValue},
    crate::domain::{Atom, HasVendorPrefix, VendorPrefix},
    cssparser::ToCss,
    std::fmt,
};

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct PropertyDeclaration<I: HasImportance> {
    pub vendor_prefix: Option<VendorPrefix>,
    pub name: Atom,
    pub value: UnparsedPropertyValue,
    pub importance: I,
}

impl<I: HasImportance> ToCss for PropertyDeclaration<I> {
    fn to_css<W: fmt::Write>(&self, dest: &mut W) -> fmt::Result {
        self.to_css_without_trailing_semicolon(dest)?;
        dest.write_char(';')
    }
}

impl<I: HasImportance> HasVendorPrefix for PropertyDeclaration<I> {
    #[inline(always)]
    fn isNotVendorPrefixed(&self) -> bool {
        self.vendor_prefix.is_none()
    }
}

impl<I: HasImportance> PropertyDeclaration<I> {
    /// <https://drafts.csswg.org/css-variables/#typedef-custom-property-name>
    #[inline(always)]
    pub fn hasACustomPropertyName(&self) -> bool {
        self.name.starts_with("--")
    }

    #[inline(always)]
    pub fn hasAsciiNameIgnoringCase(&self, name: &str) -> bool {
        self.name.eq_ignore_ascii_case(name)
    }

    #[inline(always)]
    pub(crate) fn to_css_without_trailing_semicolon<W: fmt::Write>(
        &self,
        dest: &mut W,
    ) -> fmt::Result {
        if let Some(ref vendorPrefix) = self.vendor_prefix {
            vendorPrefix.to_css(dest)?;
        }
        self.name.to_css(dest)?;
        dest.write_char(':')?;
        self.value.to_css(dest)?;
        self.importance.to_css(dest)
    }
}
