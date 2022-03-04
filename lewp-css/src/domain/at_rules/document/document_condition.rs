// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.

use {
    super::{Document, UrlMatchingFunction},
    crate::{parsers::ParserContext, CustomParseError},
    cssparser::{ParseError, Parser, ToCss},
    std::fmt,
};

/// A `@document` rule's condition.
///
/// <https://www.w3.org/TR/2012/WD-css3-conditional-20120911/#at-document>
///
/// The `@document` rule's condition is written as a comma-separated list of URL matching functions, and the condition evaluates to true whenever any one of those functions evaluates to true.
#[derive(Clone, Debug)]
pub struct DocumentCondition(pub Vec<UrlMatchingFunction>);

impl ToCss for DocumentCondition {
    fn to_css<W: fmt::Write>(&self, dest: &mut W) -> fmt::Result {
        let mut iter = self.0.iter();
        let first = iter.next().expect("Empty DocumentCondition, should contain at least one URL matching function");
        first.to_css(dest)?;
        for url_matching_function in iter {
            dest.write_str(", ")?;
            url_matching_function.to_css(dest)?;
        }
        Ok(())
    }
}

impl DocumentCondition {
    /// Parse a document condition.
    pub(crate) fn parse<'i, 't>(
        context: &ParserContext,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i, CustomParseError<'i>>> {
        input
            .parse_comma_separated(|input| {
                UrlMatchingFunction::parse(context, input)
            })
            .map(DocumentCondition)
    }

    /// Evaluate a document condition.
    pub fn evaluate<D: Document>(&self, document: &D) -> bool {
        self.0.iter().any(|url_matching_function| {
            url_matching_function.evaluate(document)
        })
    }
}
