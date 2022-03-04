// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.

use {
    super::Document,
    crate::{
        domain::SpecifiedUrl,
        parsers::{Parse, ParserContext},
        CustomParseError,
    },
    cssparser::{
        serialize_string,
        BasicParseError,
        BasicParseErrorKind,
        ParseError,
        Parser,
        ToCss,
        Token,
    },
    std::fmt,
};

/// A URL matching function for a `@document` rule's condition.
#[derive(Clone, Debug)]
pub enum UrlMatchingFunction {
    /// Exact URL matching function.
    /// It evaluates to true whenever the URL of the document being styled is exactly the URL given.
    Url(SpecifiedUrl),

    /// URL prefix matching function.
    /// It evaluates to true whenever the URL of the document being styled has the argument to the function as an initial substring (which is true when the two strings are equal).
    /// When the argument is the empty string, it evaluates to true for all documents.
    UrlPrefix(String),

    /// Domain matching function.
    /// It evaluates to true whenever the URL of the document being styled has a host subcomponent and that host subcomponent is exactly the argument to the ‘domain()’ function or a final substring of the host component is a  period (U+002E) immediately followed by the argument to the ‘domain()’ function.
    Domain(String),

    /// Regular expression matching function.
    /// It evaluates to true whenever the regular expression matches the entirety of the URL of the document being styled.
    RegExp(String),
}

macro_rules! parse_quoted_or_unquoted_string {
    ($input:ident, $url_matching_function:expr) => {
        $input.parse_nested_block(|input| {
            let start = input.position();
            input
                .parse_entirely(|input| match input.next() {
                    Ok(&Token::QuotedString(ref value)) => {
                        Ok($url_matching_function(value.as_ref().to_owned()))
                    }
                    Ok(t) => Err(ParseError::from(BasicParseError {
                        kind: BasicParseErrorKind::UnexpectedToken(t.clone()),
                        location: input.state().source_location(),
                    })),
                    Err(e) => Err(e.into()),
                })
                .or_else(|_: ParseError<'i, CustomParseError<'i>>| {
                    while let Ok(_) = input.next() {}

                    Ok($url_matching_function(
                        input.slice_from(start).to_string(),
                    ))
                })
        })
    };
}

impl ToCss for UrlMatchingFunction {
    fn to_css<W: fmt::Write>(&self, dest: &mut W) -> fmt::Result {
        use self::UrlMatchingFunction::*;

        match *self {
            Url(ref url) => url.to_css(dest),

            UrlPrefix(ref url_prefix) => {
                dest.write_str("url-prefix(")?;
                serialize_string(url_prefix, dest)?;
                dest.write_char(')')
            }

            Domain(ref domain) => {
                dest.write_str("domain(")?;
                serialize_string(domain, dest)?;
                dest.write_char(')')
            }

            RegExp(ref regex) => {
                dest.write_str("regexp(")?;
                serialize_string(regex, dest)?;
                dest.write_char(')')
            }
        }
    }
}

impl UrlMatchingFunction {
    /// Parse a URL matching function for a `@document` rule's condition.
    pub(crate) fn parse<'i, 't>(
        context: &ParserContext,
        input: &mut Parser<'i, 't>,
    ) -> Result<UrlMatchingFunction, ParseError<'i, CustomParseError<'i>>> {
        if input
            .r#try(|input| input.expect_function_matching("url-prefix"))
            .is_ok()
        {
            parse_quoted_or_unquoted_string!(
                input,
                UrlMatchingFunction::UrlPrefix
            )
        } else if input
            .r#try(|input| input.expect_function_matching("domain"))
            .is_ok()
        {
            parse_quoted_or_unquoted_string!(input, UrlMatchingFunction::Domain)
        } else if input
            .r#try(|input| input.expect_function_matching("regexp"))
            .is_ok()
        {
            input.parse_nested_block(|input| {
                Ok(UrlMatchingFunction::RegExp(
                    input.expect_string()?.as_ref().to_owned(),
                ))
            })
        } else if let Ok(url) =
            input.r#try(|input| SpecifiedUrl::parse(context, input))
        {
            Ok(UrlMatchingFunction::Url(url))
        } else {
            Err(ParseError::from(
                CustomParseError::DocumentAtRuleUrlMatchingFunctionWasInvalid,
            ))
        }
    }

    /// Evaluate a document condition.
    pub fn evaluate<D: Document>(&self, document: &D) -> bool {
        document.documentMatchesUrl(self)
    }
}
