// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.

use {
    crate::parsers::separators::Separator,
    cssparser::{ParseError, Parser},
};

/// Type used as the associated type in the `Separated` trait on a type to indicate that a serialized list of elements of this type is separated by commas.
pub(crate) struct Comma;

impl Separator for Comma {
    fn separator() -> &'static str {
        ","
    }

    fn parse<'i, 't, F, T, E>(
        input: &mut Parser<'i, 't>,
        parse_one: F,
    ) -> Result<Vec<T>, ParseError<'i, E>>
    where
        F: for<'tt> FnMut(&mut Parser<'i, 'tt>) -> Result<T, ParseError<'i, E>>,
    {
        input.parse_comma_separated(parse_one)
    }
}
