// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.

use {
    super::ViewportDescriptorDeclaration,
    crate::{
        domain::{
            at_rules::VendorPrefixedAtRule,
            HasVendorPrefix,
            VendorPrefix,
        },
        parsers::{
            viewport_at_rule_parser::ViewportAtRuleParser,
            ParserContext,
        },
        CustomParseError,
    },
    cssparser::{DeclarationListParser, ParseError, Parser, ToCss},
    std::fmt,
};

/// A `@viewport` rule.
#[derive(Clone, Debug, PartialEq)]
pub struct ViewportAtRule {
    pub vendor_prefix: Option<VendorPrefix>,

    /// The declarations contained in this @viewport rule.
    pub declarations: Vec<ViewportDescriptorDeclaration>,
}

impl ViewportAtRule {
    /// Parse a single @viewport rule.
    pub(crate) fn parse_body<'i, 't>(
        vendor_prefix: Option<VendorPrefix>,
        context: &ParserContext,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i, CustomParseError<'i>>> {
        let parser = ViewportAtRuleParser { context };

        let mut declarations = Vec::new();
        let parser = DeclarationListParser::new(input, parser);
        for result in parser {
            match result {
                Ok(viewportDescriptorDeclaration) => {
                    declarations.push(viewportDescriptorDeclaration);
                }

                Err(preciseParseError) => return Err(preciseParseError.0),
            }
        }
        Ok(Self {
            vendor_prefix,
            declarations,
        })
    }
}

impl ToCss for ViewportAtRule {
    fn to_css<W: fmt::Write>(&self, dest: &mut W) -> fmt::Result {
        dest.write_char('@')?;
        if let Some(ref vendorPrefix) = self.vendor_prefix {
            vendorPrefix.to_css(dest)?;
        }
        dest.write_str("viewport{")?;

        let length = self.declarations.len();
        if length != 0 {
            for index in 0..(length - 1) {
                (unsafe { self.declarations.get_unchecked(index) })
                    .to_css(dest)?;
            }

            (unsafe { self.declarations.get_unchecked(length - 1) })
                .to_css_without_trailing_semicolon(dest)?
        }

        dest.write_char('}')
    }
}

impl HasVendorPrefix for ViewportAtRule {
    #[inline(always)]
    fn isNotVendorPrefixed(&self) -> bool {
        self.vendor_prefix.is_none()
    }
}

impl VendorPrefixedAtRule for ViewportAtRule {}
