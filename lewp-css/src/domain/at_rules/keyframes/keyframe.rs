// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.

use {
    super::KeyframeSelector,
    crate::domain::{
        properties::{
            DoesNotHaveImportance,
            PropertyDeclaration,
            PropertyDeclarations,
        },
        HasPropertyDeclarations,
    },
    cssparser::ToCss,
    std::fmt,
};

/// A keyframe.
#[derive(Debug, Clone)]
pub struct Keyframe {
    /// The selector this keyframe was specified from.
    pub selector: KeyframeSelector,

    /// The declaration block that was declared inside this keyframe.
    pub property_declarations: PropertyDeclarations<DoesNotHaveImportance>,
}

impl ToCss for Keyframe {
    fn to_css<W: fmt::Write>(&self, dest: &mut W) -> fmt::Result {
        self.selector.to_css(dest)?;
        dest.write_char('{')?;
        self.property_declarations.to_css(dest)?;
        dest.write_char('}')?;
        Ok(())
    }
}

impl HasPropertyDeclarations<DoesNotHaveImportance> for Keyframe {
    #[inline(always)]
    fn property_declarations(
        &self,
    ) -> &PropertyDeclarations<DoesNotHaveImportance> {
        &self.property_declarations
    }

    #[inline(always)]
    fn property_declarations_mut(
        &mut self,
    ) -> &mut PropertyDeclarations<DoesNotHaveImportance> {
        &mut self.property_declarations
    }

    #[inline(always)]
    fn property_declarations_slice(
        &self,
    ) -> &[PropertyDeclaration<DoesNotHaveImportance>] {
        &self.property_declarations.0[..]
    }

    #[inline(always)]
    fn property_declarations_vec(
        &self,
    ) -> &Vec<PropertyDeclaration<DoesNotHaveImportance>> {
        &self.property_declarations.0
    }

    #[inline(always)]
    fn property_declarations_vec_mut(
        &mut self,
    ) -> &mut Vec<PropertyDeclaration<DoesNotHaveImportance>> {
        &mut self.property_declarations.0
    }
}
