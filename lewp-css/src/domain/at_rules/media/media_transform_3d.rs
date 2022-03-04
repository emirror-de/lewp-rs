// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.

use {
    crate::{
        parsers::{Parse, ParserContext},
        CustomParseError,
    },
    cssparser::{ParseError, Parser, ToCss},
    std::fmt,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MediaTransform3D {
    pub support: bool,
}

impl ToCss for MediaTransform3D {
    fn to_css<W: fmt::Write>(&self, dest: &mut W) -> fmt::Result {
        let value = if self.support { 1 } else { 0 };
        value.to_css(dest)
    }
}

impl Parse for MediaTransform3D {
    fn parse<'i, 't>(
        _context: &ParserContext,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i, CustomParseError<'i>>> {
        let support = match input.expect_integer()? {
            0 => false,
            1 => true,
            invalid => {
                return Err(ParseError::from(
                    CustomParseError::MediaTransform3DMustBeEitherZeroOrOne(
                        invalid,
                    ),
                ))
            }
        };

        Ok(Self { support })
    }
}
