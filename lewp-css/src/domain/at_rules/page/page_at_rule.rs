// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.

use {
    super::PageSelectorPseudoClass,
    crate::domain::{
        properties::{Importance, PropertyDeclaration, PropertyDeclarations},
        HasPropertyDeclarations,
    },
    cssparser::ToCss,
    std::fmt,
};

/// A [`@page`](crate::domain::CssRule::Page) rule.
/// page: <https://drafts.csswg.org/css2/page.html#page-box>
/// page-selectors: <https://drafts.csswg.org/css2/page.html#page-selectors>
#[derive(Default, Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct PageAtRule {
    pub page_selector_pseudo_class: Option<PageSelectorPseudoClass>,

    /// The declaration block this page rule contains.
    pub property_declarations: PropertyDeclarations<Importance>,
}

impl ToCss for PageAtRule {
    fn to_css<W: fmt::Write>(&self, dest: &mut W) -> fmt::Result {
        dest.write_str("@page")?;
        if let Some(ref page_selector_pseudo_class) =
            self.page_selector_pseudo_class
        {
            dest.write_char(' ')?;
            page_selector_pseudo_class.to_css(dest)?;
        }
        dest.write_char('{')?;
        self.property_declarations.to_css(dest)?;
        dest.write_char('}')
    }
}

impl HasPropertyDeclarations<Importance> for PageAtRule {
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
