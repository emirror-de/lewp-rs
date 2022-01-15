// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.

use {
    crate::domain::numbers::CssNumberConversionError,
    std::{
        error::Error,
        fmt::{self, Display, Formatter},
        num::ParseFloatError,
    },
};

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum UnitFromStrError {
    Float(ParseFloatError),
    Conversion(CssNumberConversionError),
    InvalidDimension,
}

impl Display for UnitFromStrError {
    #[inline(always)]
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        use self::UnitFromStrError::*;

        match *self
        {
            Float(ref error) => write!(f, "Could not parse string to CssFloat because parsing it as a float caused '{}'", error),
            Conversion(ref error) => write!(f, "Could not parse string to CssFloat because converting it from a parsed float caused '{}'", error),
            InvalidDimension => write!(f, "Could not parse string to CssFloat because it had an invalid dimension"),
        }
    }
}

impl Error for UnitFromStrError {
    #[inline(always)]
    fn description(&self) -> &str {
        use self::UnitFromStrError::*;

        match *self {
            Float(_) => "float error",
            Conversion(_) => "conversion error",
            InvalidDimension => "invalid dimension",
        }
    }

    #[inline(always)]
    fn cause(&self) -> Option<&dyn Error> {
        use self::UnitFromStrError::*;

        match *self {
            Float(ref error) => Some(error),
            Conversion(ref error) => Some(error),
            InvalidDimension => None,
        }
    }
}

impl From<ParseFloatError> for UnitFromStrError {
    #[inline(always)]
    fn from(error: ParseFloatError) -> Self {
        UnitFromStrError::Float(error)
    }
}

impl From<CssNumberConversionError> for UnitFromStrError {
    #[inline(always)]
    fn from(error: CssNumberConversionError) -> Self {
        UnitFromStrError::Conversion(error)
    }
}
