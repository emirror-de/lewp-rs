// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.

bitflags! {
    /// The mode to use when parsing values.
    pub struct ParsingMode: u8
    {
        /// In CSS, units must have units, except for zero values, where the unit can be omitted.
        /// <https://www.w3.org/TR/css3-values/#lengths>
        const Default = 0x00;

        /// In SVG, a coordinate or length value without a unit identifier (e.g., "25") is assumed to be in user units (px).
        /// <https://www.w3.org/TR/SVG/coords.html#Units>
        const AllowUnitLessLength = 0x01;

        /// In SVG, out-of-range values are not treated as an error in parsing.
        /// <https://www.w3.org/TR/SVG/implnote.html#RangeClamping>
        const AllowAllNumericValues = 0x02;
    }
}

impl ParsingMode {
    /// Whether the parsing mode allows unit-less units for non-zero values to be interpreted as px.
    pub(crate) fn allows_unitless_lengths(&self) -> bool {
        self.intersects(Self::AllowUnitLessLength)
    }

    /// Whether the parsing mode allows all numeric values.
    #[allow(dead_code)]
    pub(crate) fn allows_all_numeric_values(&self) -> bool {
        self.intersects(Self::AllowAllNumericValues)
    }
}
