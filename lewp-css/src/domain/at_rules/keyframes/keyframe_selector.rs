// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.

use {
    super::KeyframePercentage,
    crate::CustomParseError,
    cssparser::{ParseError, Parser, ToCss},
    std::fmt,
};

/// A keyframes selector is a list of percentages or from/to symbols, which are converted at parse time to percentages.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct KeyframeSelector(pub Vec<KeyframePercentage>);

impl ToCss for KeyframeSelector {
    fn to_css<W: fmt::Write>(&self, dest: &mut W) -> fmt::Result {
        let mut iter = self.0.iter();
        iter.next().unwrap().to_css(dest)?;
        for percentage in iter {
            dest.write_char(',')?;
            percentage.to_css(dest)?;
        }
        Ok(())
    }
}

impl KeyframeSelector {
    /// Return the list of percentages this selector contains.
    #[inline]
    pub fn percentages(&self) -> &[KeyframePercentage] {
        &self.0
    }

    /// Parse a keyframe selector from CSS input.
    pub(crate) fn parse<'i, 't>(
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i, CustomParseError<'i>>> {
        input
            .parse_comma_separated(KeyframePercentage::parse)
            .map(KeyframeSelector)
    }
}
