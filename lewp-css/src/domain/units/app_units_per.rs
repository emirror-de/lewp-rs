// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.

pub trait AppUnitsPer {
    /// Number of app units per pixel
    const AppUnitsPerPX: Self;

    /// Number of app units per inch
    const AppUnitsPerIN: Self;

    /// Number of app units per centimeter
    const AppUnitsPerCM: Self;

    /// Number of app units per millimeter
    const AppUnitsPerMM: Self;

    /// Number of app units per quarter
    const AppUnitsPerQ: Self;

    /// Number of app units per point
    const AppUnitsPerPT: Self;

    /// Number of app units per pica
    const AppUnitsPerPC: Self;
}

impl AppUnitsPer for f32 {
    /// Number of app units per pixel
    const AppUnitsPerPX: f32 = 60.;

    /// Number of app units per inch
    const AppUnitsPerIN: f32 = Self::AppUnitsPerPX * 96.;

    /// Number of app units per centimeter
    const AppUnitsPerCM: f32 = Self::AppUnitsPerIN / 2.54;

    /// Number of app units per millimeter
    const AppUnitsPerMM: f32 = Self::AppUnitsPerIN / 25.4;

    /// Number of app units per quarter
    const AppUnitsPerQ: f32 = Self::AppUnitsPerMM / 4.;

    /// Number of app units per point
    const AppUnitsPerPT: f32 = Self::AppUnitsPerIN / 72.;

    /// Number of app units per pica
    const AppUnitsPerPC: f32 = Self::AppUnitsPerPT * 12.;
}
