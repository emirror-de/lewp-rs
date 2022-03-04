// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.

use {
    crate::{
        parsers::{Parse, ParserContext},
        CustomParseError,
    },
    cssparser::{serialize_identifier, ParseError, Parser, ToCss},
    std::fmt,
};

/// A font-weight value for a @font-face rule.
/// The font-weight CSS property specifies the weight or boldness of the font.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum FontWeight {
    _100,
    _200,
    _300,
    _400,
    _500,
    _600,
    _700,
    _800,
    _900,
}

impl ToCss for FontWeight {
    fn to_css<W: fmt::Write>(&self, dest: &mut W) -> fmt::Result {
        use self::FontWeight::*;

        match *self {
            _100 => serialize_identifier("100", dest),
            _200 => serialize_identifier("200", dest),
            _300 => serialize_identifier("300", dest),
            _400 => serialize_identifier("400", dest),
            _500 => serialize_identifier("500", dest),
            _600 => serialize_identifier("600", dest),
            _700 => serialize_identifier("700", dest),
            _800 => serialize_identifier("800", dest),
            _900 => serialize_identifier("900", dest),
        }
    }
}

impl Parse for FontWeight {
    fn parse<'i, 't>(
        _: &ParserContext,
        input: &mut Parser<'i, 't>,
    ) -> Result<FontWeight, ParseError<'i, CustomParseError<'i>>> {
        let result = input.r#try(|input| {
            let ident = input.expect_ident().map_err(|_| ())?;
            match_ignore_ascii_case! {
                ident,
                "normal" => Ok(Self::normal),
                "bold" => Ok(Self::bold),
                _ => Err(())
            }
        });

        result.or_else(|_| {
            Self::from_int(input.expect_integer()?).map_err(|()| {
                ParseError::from(
                    CustomParseError::FontFaceAtRuleFontWeightWasNotAValidIdentifierOrInteger,
                )
            })
        })
    }
}

impl FontWeight {
    /// Convert from an integer to Weight
    /// As of CSS Fonts Module Level 3, only the following values are valid: 100, 200, 300, 400, 500, 600, 700, 800 and 900
    pub fn from_int(weight: i32) -> Result<Self, ()> {
        use self::FontWeight::*;

        match weight {
            100 => Ok(_100),
            200 => Ok(_200),
            300 => Ok(_300),
            400 => Ok(_400),
            500 => Ok(_500),
            600 => Ok(_600),
            700 => Ok(_700),
            800 => Ok(_800),
            900 => Ok(_900),
            _ => Err(()),
        }
    }

    pub fn isExactlyNormal(&self) -> bool {
        *self == FontWeight::_400
    }

    pub fn isExactlyBold(&self) -> bool {
        *self == FontWeight::_700
    }

    pub fn isBold(&self) -> bool {
        *self > FontWeight::_500
    }

    /// Return the bolder weight
    pub fn bolder(self) -> Self {
        use self::FontWeight::*;

        if self < _400 {
            _400
        } else if self < _600 {
            _700
        } else {
            _900
        }
    }

    /// Returns the lighter weight
    pub fn lighter(self) -> Self {
        use self::FontWeight::*;

        if self < _600 {
            _100
        } else if self < _800 {
            _400
        } else {
            _700
        }
    }

    pub const normal: FontWeight = FontWeight::_400;

    pub const bold: FontWeight = FontWeight::_700;
}
