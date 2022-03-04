// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.

use {
    crate::{
        domain::CounterStyleIdent,
        parsers::{Parse, ParserContext},
        CustomParseError,
    },
    cssparser::{ParseError, Parser, ToCss},
    std::fmt,
};

/// <https://drafts.csswg.org/css-counter-styles/#counter-style-system>
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum System {
    /// 'cyclic'
    Cyclic,

    /// 'numeric'
    Numeric,

    /// 'alphabetic'
    Alphabetic,

    /// 'symbolic'
    Symbolic,

    /// 'additive'
    Additive,

    /// 'fixed <integer>?'
    Fixed {
        /// '<integer>?'
        first_symbol_value: Option<i32>,
    },

    /// 'extends <counter-style-name>'
    Extends(CounterStyleIdent),
}

impl Parse for System {
    fn parse<'i, 't>(
        _context: &ParserContext,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i, CustomParseError<'i>>> {
        use self::System::*;

        let identifier = input.expect_ident_cloned()?;

        match_ignore_ascii_case! {
            &*identifier,

            "cyclic" => Ok(Cyclic),

            "numeric" => Ok(Numeric),

            "alphabetic" => Ok(Alphabetic),

            "symbolic" => Ok(Symbolic),

            "additive" => Ok(Additive),

            "fixed" =>
            {
                let first_symbol_value = input.r#try(|i| i.expect_integer()).ok();
                Ok(Fixed { first_symbol_value })
            },

            "extends" =>  Ok(Extends(CounterStyleIdent::parse(input)?)),

            _ => Err(ParseError::from(CustomParseError::CounterStyleSystemIsNotKnown(identifier.clone()))),
        }
    }
}

impl ToCss for System {
    fn to_css<W: fmt::Write>(&self, dest: &mut W) -> fmt::Result {
        use self::System::*;

        match *self {
            Cyclic => dest.write_str("cyclic"),

            Numeric => dest.write_str("numeric"),

            Alphabetic => dest.write_str("alphabetic"),

            Symbolic => dest.write_str("symbolic"),

            Additive => dest.write_str("additive"),

            Fixed { first_symbol_value } => {
                if let Some(value) = first_symbol_value {
                    dest.write_str("fixed ")?;
                    value.to_css(dest)
                } else {
                    dest.write_str("fixed")
                }
            }

            Extends(ref other) => {
                dest.write_str("extends ")?;
                other.to_css(dest)
            }
        }
    }
}
