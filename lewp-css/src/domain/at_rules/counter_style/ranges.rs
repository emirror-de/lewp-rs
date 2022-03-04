use {
    crate::{
        parsers::{Parse, ParserContext},
        CustomParseError,
    },
    cssparser::{
        BasicParseError,
        BasicParseErrorKind,
        ParseError,
        Parser,
        ToCss,
        Token::*,
    },
    std::fmt,
};

// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.

/// <https://drafts.csswg.org/css-counter-styles/#counter-style-range>
///
/// Empty Vec represents 'auto'
#[derive(Clone, Debug)]
pub struct Ranges(pub Vec<::std::ops::Range<Option<i32>>>);

impl Parse for Ranges {
    fn parse<'i, 't>(
        _context: &ParserContext,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i, CustomParseError<'i>>> {
        if input
            .r#try(|input| input.expect_ident_matching("auto"))
            .is_ok()
        {
            Ok(Self::empty())
        } else {
            input
                .parse_comma_separated(|input| {
                    let opt_start = Self::parse_bound(input)?;
                    let opt_end = Self::parse_bound(input)?;
                    if let (Some(start), Some(end)) = (opt_start, opt_end) {
                        if start > end {
                            return Err(ParseError::from(
                                CustomParseError::CounterStyleRangesCanNotHaveStartGreaterThanEnd(
                                    start, end,
                                ),
                            ));
                        }
                    }
                    Ok(opt_start..opt_end)
                })
                .map(Ranges)
        }
    }
}

impl ToCss for Ranges {
    fn to_css<W: fmt::Write>(&self, dest: &mut W) -> fmt::Result {
        let mut iter = self.0.iter();
        if let Some(first) = iter.next() {
            Self::range_to_css(first, dest)?;
            for item in iter {
                dest.write_str(",")?;
                Self::range_to_css(item, dest)?;
            }
            Ok(())
        } else {
            dest.write_str("auto")
        }
    }
}

impl Ranges {
    #[inline(always)]
    pub fn empty() -> Self {
        Ranges(Vec::new())
    }

    #[inline(always)]
    pub fn is_auto(&self) -> bool {
        self.0.is_empty()
    }

    fn parse_bound<'i, 't>(
        input: &mut Parser<'i, 't>,
    ) -> Result<Option<i32>, ParseError<'i, CustomParseError<'i>>> {
        match input.next() {
            Ok(&Number {
                int_value: Some(v), ..
            }) => Ok(Some(v)),

            Ok(&Ident(ref ident)) if ident.eq_ignore_ascii_case("infinite") => {
                Ok(None)
            }

            Ok(token) => Err(ParseError::from(BasicParseError {
                kind: BasicParseErrorKind::UnexpectedToken(token.clone()),
                location: input.state().source_location(),
            })),

            Err(error) => Err(error.into()),
        }
    }

    fn range_to_css<W: fmt::Write>(
        range: &::std::ops::Range<Option<i32>>,
        dest: &mut W,
    ) -> fmt::Result {
        Self::bound_to_css(range.start, dest)?;
        dest.write_char(' ')?;
        Self::bound_to_css(range.end, dest)
    }

    fn bound_to_css<W: fmt::Write>(
        range: Option<i32>,
        dest: &mut W,
    ) -> fmt::Result {
        if let Some(finite) = range {
            finite.to_css(dest)
        } else {
            dest.write_str("infinite")
        }
    }
}
