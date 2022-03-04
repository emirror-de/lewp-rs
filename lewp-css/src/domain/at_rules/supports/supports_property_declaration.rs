// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.

use {
    super::consume_any_value::consume_any_value,
    crate::CustomParseError,
    cssparser::{ParseError, Parser, ToCss},
    std::fmt,
};

/// A possibly-invalid property declaration
#[derive(Debug, Clone)]
pub struct SupportsPropertyDeclaration(pub String);

impl ToCss for SupportsPropertyDeclaration {
    fn to_css<W: fmt::Write>(&self, dest: &mut W) -> fmt::Result {
        dest.write_str(&self.0)
    }
}

impl SupportsPropertyDeclaration {
    /// Parse a declaration
    pub(crate) fn parse<'i, 't>(
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i, CustomParseError<'i>>> {
        let pos = input.position();
        input.expect_ident()?;
        input.expect_colon()?;
        consume_any_value(input)?;
        Ok(SupportsPropertyDeclaration(
            input.slice_from(pos).to_owned(),
        ))
    }
}
