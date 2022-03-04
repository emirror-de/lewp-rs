// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.

use {crate::domain::Atom, cssparser::ToCss, std::fmt};

/// A @font-feature-values block declaration.
/// It is `<ident>: <integer>+`.
/// This struct can take 3 value types.
/// - `SingleValue` is to keep just one unsigned integer value.
/// - `PairValues` is to keep one or two unsigned integer values.
/// - `VectorValues` is to keep a list of unsigned integer values.
#[derive(Clone, Debug, PartialEq)]
pub struct FontFeatureValuesDeclaration<T: ToCss> {
    /// An `<ident>` for declaration name.
    pub name: Atom,

    /// An `<integer>+` for declaration value.
    pub value: T,
}

impl<T: ToCss> ToCss for FontFeatureValuesDeclaration<T> {
    #[inline(always)]
    fn to_css<W: fmt::Write>(&self, dest: &mut W) -> fmt::Result {
        self.to_css_without_trailing_semicolon(dest)?;
        dest.write_char(';')
    }
}

impl<T: ToCss> FontFeatureValuesDeclaration<T> {
    #[inline(always)]
    pub(crate) fn to_css_without_trailing_semicolon<W: fmt::Write>(
        &self,
        dest: &mut W,
    ) -> fmt::Result {
        self.name.to_css(dest)?;
        dest.write_char(':')?;
        self.value.to_css(dest)
    }
}
