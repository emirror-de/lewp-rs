// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.

use super::UrlMatchingFunction;

/// A trait that is used when evaluating CSS rules that are decided using document attributes
pub trait Document {
    /// Used when evaluating @document rules
    fn documentMatchesUrl(
        &self,
        urlMatchingFunction: &UrlMatchingFunction,
    ) -> bool;
}
