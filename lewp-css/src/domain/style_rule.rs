// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.

use {
    super::{
        properties::{Importance, PropertyDeclaration, PropertyDeclarations},
        selectors::DeduplicatedSelectors,
        HasPropertyDeclarations,
    },
    cssparser::ToCss,
    std::fmt,
};

/// A style rule, with selectors and declarations.
#[derive(Debug, Clone)]
pub struct StyleRule {
    /// The list of selectors in this rule.
    pub selectors: DeduplicatedSelectors,

    /// The declaration block with the properties it contains.
    pub property_declarations: PropertyDeclarations<Importance>,
}

impl ToCss for StyleRule {
    /// <https://drafts.csswg.org/cssom/#serialize-a-css-rule>
    fn to_css<W: fmt::Write>(&self, dest: &mut W) -> fmt::Result {
        self.selectors.to_css(dest)?;

        dest.write_char('{')?;

        self.property_declarations.to_css(dest)?;

        dest.write_char('}')
    }
}

impl HasPropertyDeclarations<Importance> for StyleRule {
    #[inline(always)]
    fn property_declarations(&self) -> &PropertyDeclarations<Importance> {
        &self.property_declarations
    }

    #[inline(always)]
    fn property_declarations_mut(
        &mut self,
    ) -> &mut PropertyDeclarations<Importance> {
        &mut self.property_declarations
    }

    #[inline(always)]
    fn property_declarations_slice(
        &self,
    ) -> &[PropertyDeclaration<Importance>] {
        &self.property_declarations.0[..]
    }

    #[inline(always)]
    fn property_declarations_vec(
        &self,
    ) -> &Vec<PropertyDeclaration<Importance>> {
        &self.property_declarations.0
    }

    #[inline(always)]
    fn property_declarations_vec_mut(
        &mut self,
    ) -> &mut Vec<PropertyDeclaration<Importance>> {
        &mut self.property_declarations.0
    }
}
