// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.

use {
    crate::{
        domain::{
            numbers::{CssNumber, CssNumberNewType, CssUnsignedNumber},
            units::PercentageUnit,
        },
        CustomParseError,
    },
    cssparser::{ParseError, Parser, ToCss},
    std::{cmp::Ordering, fmt},
};

/// A percentage from 0% to 100%, indicating the percentage of the animation when this keyframe should run.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct KeyframePercentage(pub PercentageUnit<CssUnsignedNumber>);

impl Ord for KeyframePercentage {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        // We know we have a number from 0 to 1, so unwrap() here is safe.
        self.0.partial_cmp(&other.0).unwrap()
    }
}

impl Eq for KeyframePercentage {}

impl ToCss for KeyframePercentage {
    fn to_css<W: fmt::Write>(&self, dest: &mut W) -> fmt::Result {
        if self.0.is_zero() {
            dest.write_str("to")
        } else {
            self.0.to_css(dest)
        }
    }
}

impl KeyframePercentage {
    pub(crate) fn parse<'i, 't>(
        input: &mut Parser<'i, 't>,
    ) -> Result<KeyframePercentage, ParseError<'i, CustomParseError<'i>>> {
        let percentage = if input
            .r#try(|input| input.expect_ident_matching("from"))
            .is_ok()
        {
            KeyframePercentage(PercentageUnit::ZeroPercent)
        } else if input
            .r#try(|input| input.expect_ident_matching("to"))
            .is_ok()
        {
            KeyframePercentage(PercentageUnit::OneHundredPercent)
        } else {
            let percentage = input.expect_percentage()?;
            if (0. ..=1.).contains(&percentage) {
                KeyframePercentage(PercentageUnit(
                    CssUnsignedNumber::_construct(percentage),
                ))
            } else {
                return Err(ParseError::from(
                    CustomParseError::KeyframePercentageWasNotBetweenZeroAndOneInclusive(
                        percentage,
                    ),
                ));
            }
        };

        Ok(percentage)
    }
}
