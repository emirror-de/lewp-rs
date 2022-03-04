// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.

use {
    super::ViewportDescriptor,
    crate::{domain::properties::Importance, CustomParseError},
    cssparser::{ParseError, Parser, ToCss},
    std::fmt,
};

#[derive(Clone, Debug, PartialEq)]
#[allow(missing_docs)]
pub struct ViewportDescriptorDeclaration {
    pub descriptor: ViewportDescriptor,
    pub importance: Importance,
}

impl ToCss for ViewportDescriptorDeclaration {
    fn to_css<W: fmt::Write>(&self, dest: &mut W) -> fmt::Result {
        self.to_css_without_trailing_semicolon(dest)?;
        dest.write_char(';')
    }
}

impl ViewportDescriptorDeclaration {
    #[inline(always)]
    pub(crate) fn parse_important<'i, 't>(
        descriptor: ViewportDescriptor,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i, CustomParseError<'i>>> {
        let importance = Importance::parse(input);

        Ok(Self {
            descriptor,
            importance,
        })
    }

    #[inline(always)]
    pub(crate) fn to_css_without_trailing_semicolon<W: fmt::Write>(
        &self,
        dest: &mut W,
    ) -> fmt::Result {
        self.descriptor.to_css(dest)?;
        self.importance.to_css(dest)
    }
}
