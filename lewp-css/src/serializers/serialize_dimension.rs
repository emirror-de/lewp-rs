// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.

use {crate::domain::numbers::CssNumber, std::fmt};

/// Serialize a value with given unit into dest.
pub(crate) fn serialize_dimension<W: fmt::Write, Number: CssNumber>(
    value: Number,
    unit: &str,
    dest: &mut W,
) -> fmt::Result {
    value.to_css(dest)?;
    dest.write_str(unit)
}
