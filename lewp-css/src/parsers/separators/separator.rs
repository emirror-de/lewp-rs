// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.

use cssparser::{ParseError, Parser};

/// A trait satisfied by the types corresponding to separators.
pub(crate) trait Separator {
    /// The separator string that the satisfying separator type corresponds to.
    fn separator() -> &'static str;

    /// Parses a sequence of values separated by this separator.
    ///
    /// The given closure is called repeatedly for each item in the sequence.
    ///
    /// Successful results are accumulated in a vector.
    ///
    /// This method returns `Err(_)` the first time a closure does or if the separators aren't correct.
    fn parse<'i, 't, F, T, E>(
        parser: &mut Parser<'i, 't>,
        parse_one: F,
    ) -> Result<Vec<T>, ParseError<'i, E>>
    where
        F: for<'tt> FnMut(&mut Parser<'i, 'tt>) -> Result<T, ParseError<'i, E>>;
}
