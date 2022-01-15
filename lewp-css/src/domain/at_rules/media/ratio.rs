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
pub struct Ratio {
    width: u32,
    height: u32,
}

impl ToCss for Ratio {
    fn to_css<W: fmt::Write>(&self, dest: &mut W) -> fmt::Result {
        self.width.to_css(dest)?;
        dest.write_str("/")?;
        self.height.to_css(dest)
    }
}

impl Parse for Ratio {
    fn parse<'i, 't>(
        _context: &ParserContext,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i, CustomParseError<'i>>> {
        let width = input.expect_integer()?;
        if width <= 0 {
            return Err(ParseError::from(
                CustomParseError::RatioNumeratorCanNotBeNegativeOrZero(width),
            ));
        }

        input.expect_delim('/')?;

        let height = input.expect_integer()?;
        if height <= 0 {
            return Err(ParseError::from(
                CustomParseError::RatioDivisorCanNotBeNegativeOrZero(width),
            ));
        }

        Ok(Self::new(width as u32, height as u32))
    }
}

impl Ratio {
    /// This method will simplify the ratio, so creating smaller numbers in resultant CSS
    #[inline(always)]
    pub fn new(width: u32, height: u32) -> Self {
        debug_assert!(width != 0, "width can not be zero");
        debug_assert!(height != 0, "height can not be zero");

        // Euclid's algorithm for finding the greatest common divisor
        #[inline(always)]
        fn greatestCommonDivisorByEuclidsAlgorithm(
            nominator: u32,
            denominator: u32,
        ) -> u32 {
            let mut x = nominator;
            let mut y = denominator;
            while y != 0 {
                let t = y;
                y = x % y;
                x = t;
            }
            x
        }
        let greatestCommonDivisor =
            greatestCommonDivisorByEuclidsAlgorithm(width, height);

        Self {
            width: width / greatestCommonDivisor,
            height: height / greatestCommonDivisor,
        }
    }

    #[inline(always)]
    pub fn width(&self) -> u32 {
        self.width
    }

    #[inline(always)]
    pub fn height(&self) -> u32 {
        self.height
    }

    #[inline(always)]
    pub fn to_scalar(&self) -> f64 {
        (self.width as f64) / (self.height as f64)
    }
}
