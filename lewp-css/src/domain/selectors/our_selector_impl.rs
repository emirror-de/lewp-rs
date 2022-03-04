// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.

use {
    crate::{
        domain::{
            at_rules::namespace::{NamespacePrefix, NamespaceUrl, Namespaces},
            selectors::{
                NonTreeStructuralPseudoClass,
                OurSelector,
                PseudoElement,
                VendorPrefixablePseudoClassName,
                VendorPrefixablePseudoElementName,
            },
            Atom,
            VendorPrefix,
        },
        parsers::OurSelectorParser,
        CustomParseError,
    },
    cssparser::{ParseError, Parser, ParserInput, ToCss},
    selectors::SelectorImpl,
    std::collections::HashMap,
};

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct OurSelectorImpl;

impl SelectorImpl for OurSelectorImpl {
    type ExtraMatchingData = String;

    type AttrValue = Atom;

    type Identifier = Atom;

    type LocalName = Atom;

    type NamespacePrefix = NamespacePrefix;

    type NamespaceUrl = NamespaceUrl;

    type BorrowedNamespaceUrl = str;

    type BorrowedLocalName = str;

    type NonTSPseudoClass = NonTreeStructuralPseudoClass;

    type PseudoElement = PseudoElement;
}

impl OurSelectorImpl {
    /// Parses a selector
    pub fn parse_selector(
        selector_css: &str,
    ) -> Result<OurSelector, ParseError<CustomParseError>> {
        const LineNumberingIsZeroBased: u32 = 0;

        let mut parserInput = ParserInput::new_with_line_number_offset(
            selector_css,
            LineNumberingIsZeroBased,
        );
        let mut input = Parser::new(&mut parserInput);

        let applyVendorPrefixToPseudoClasses = HashMap::default();
        let applyVendorPrefixToPseudoElements = HashMap::default();
        let ourSelectorParser = OurSelectorParser {
            namespaces: Namespaces::empty(),
            applyVendorPrefixToPseudoClasses: &applyVendorPrefixToPseudoClasses,
            applyVendorPrefixToPseudoElements:
                &applyVendorPrefixToPseudoElements,
        };

        match ourSelectorParser.parse(&mut input) {
            Err(error) => Err(error),
            Ok(mut selectors) => {
                let newSelector = selectors.0.drain(..).next().unwrap();
                Ok(newSelector)
            }
        }
    }

    /// Applies a vendor prefix to a CSS selector
    #[inline(always)]
    pub fn reparse_with_vendor_prefix<'a>(
        selector: &OurSelector,
        applyVendorPrefixToPseudoClasses: &'a HashMap<
            VendorPrefixablePseudoClassName,
            VendorPrefix,
        >,
        applyVendorPrefixToPseudoElements: &'a HashMap<
            VendorPrefixablePseudoElementName,
            VendorPrefix,
        >,
    ) -> Option<OurSelector> {
        const LineNumberingIsZeroBased: u32 = 0;

        let originalCss = selector.to_css_string();

        let mut parserInput = ParserInput::new_with_line_number_offset(
            &originalCss,
            LineNumberingIsZeroBased,
        );
        let mut input = Parser::new(&mut parserInput);

        let ourSelectorParser = OurSelectorParser {
            namespaces: Namespaces::empty(),
            applyVendorPrefixToPseudoClasses,
            applyVendorPrefixToPseudoElements,
        };

        let mut selectors = ourSelectorParser.parse(&mut input).unwrap();
        let newSelector = selectors.0.drain(..).next().unwrap();

        if newSelector.to_css_string() != originalCss {
            Some(newSelector)
        } else {
            None
        }
    }
}
