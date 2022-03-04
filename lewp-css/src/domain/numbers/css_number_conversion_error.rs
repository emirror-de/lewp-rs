// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.

use std::{
    error::Error,
    fmt::{self, Display, Formatter},
};

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum CssNumberConversionError {
    InfinityIsNotAllowed,
    NotANumberIsNotAllowed,
    NegativeNumberMayNotBeAllowed,
    FloatingPointNumberMayNotBeAllowed,
}

impl Display for CssNumberConversionError {
    #[inline(always)]
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.write_str(&self.to_string())
    }
}

impl Error for CssNumberConversionError {
    #[inline(always)]
    fn description(&self) -> &str {
        use self::CssNumberConversionError::*;

        match *self {
            InfinityIsNotAllowed => "infinity is not a valid CSS number",
            NotANumberIsNotAllowed => {
                "not a number (NaN) is not a valid CSS number"
            }
            NegativeNumberMayNotBeAllowed => {
                "negative numbers are not valid for a CSS number"
            }
            FloatingPointNumberMayNotBeAllowed => {
                "floating point numbers are not valid for a CSS integer"
            }
        }
    }

    #[inline(always)]
    fn cause(&self) -> Option<&dyn Error> {
        None
    }
}
