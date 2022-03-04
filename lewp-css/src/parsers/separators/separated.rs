// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.

use super::Separator;

/// Marker trait on T to automatically implement ToCss for Vec<T> when values of T are separated by some delimiter `delim`.
pub(crate) trait Separated {
    /// Associated type indicating which separator is used.
    type Delimiter: Separator;
}
