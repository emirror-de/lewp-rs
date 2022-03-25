// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.

use {
    super::*,
    crate::domain::{
        expressions::CalculablePropertyValue,
        numbers::CssSignedNumber,
        units::LengthUnit,
    },
};

/// A trait that is used when evaluating CSS rules that are decided using device attributes
/// See also ::domain::units::conversions::* for similar traits used for evaluating calculable CSS properties and converting percentages to units
pub trait Device {
    /// Used when evaluating @media rules
    /// Does this device match this media type? (All is implicitly matched and is not requested of the device)
    fn mediaTypeMatches(&self, mediaType: MediaType) -> bool;

    /// <https://www.w3.org/TR/mediaqueries-4/#width>
    fn viewportWidthMatches(
        &self,
        width: &Range<CalculablePropertyValue<LengthUnit<CssSignedNumber>>>,
    ) -> bool;

    /// <https://www.w3.org/TR/mediaqueries-4/#height>
    fn viewportHeightMatches(
        &self,
        height: &Range<CalculablePropertyValue<LengthUnit<CssSignedNumber>>>,
    ) -> bool;

    /// <https://www.w3.org/TR/mediaqueries-4/#aspect-ratio>
    fn viewportAspectRatioMatches(&self, ratio: &Range<Ratio>) -> bool;

    /// <https://www.w3.org/TR/mediaqueries-4/#orientation>
    fn orientationMatches(&self, orientation: MediaOrientation) -> bool;

    /// <https://www.w3.org/TR/mediaqueries-4/#resolution>
    fn viewportResolutionMatches(
        &self,
        resolution: &Range<MediaResolution>,
    ) -> bool;

    /// <https://www.w3.org/TR/mediaqueries-4/#scan>
    fn scanMatches(&self, scan: &MediaScan) -> bool;

    /// <https://www.w3.org/TR/mediaqueries-4/#grid>
    fn gridMatches(&self, grid: &MediaGrid) -> bool;

    /// <https://www.w3.org/TR/mediaqueries-4/#update>
    fn updateMatches(&self, update: &MediaUpdate) -> bool;

    /// <https://www.w3.org/TR/mediaqueries-4/#mf-overflow-block>
    fn overflowBlockMatches(&self, overflowBlock: &MediaOverflowBlock) -> bool;

    /// <https://www.w3.org/TR/mediaqueries-4/#mf-overflow-inline>
    fn overflowInlineMatches(
        &self,
        overflowInline: &MediaOverflowInline,
    ) -> bool;

    /// <https://www.w3.org/TR/mediaqueries-4/#color>
    fn colorBitDepthMatches(
        &self,
        colorBitDepth: &Range<ColorBitDepth>,
    ) -> bool;

    /// <https://www.w3.org/TR/mediaqueries-4/#color-index>
    fn colorIndexMatches(&self, colorIndex: &Range<MediaColorIndex>) -> bool;

    /// <https://www.w3.org/TR/mediaqueries-4/#monochrome>
    fn monochromeBitDepthMatches(
        &self,
        monochromeBitDepth: &Range<MonochromeBitDepth>,
    ) -> bool;

    /// <https://www.w3.org/TR/mediaqueries-4/#color-gamut>
    fn colorGamutMatches(&self, colorGamut: &MediaColorGamut) -> bool;

    /// <https://www.w3.org/TR/mediaqueries-4/#pointer>
    fn pointerMatches(&self, pointer: &MediaPointer) -> bool;

    /// <https://www.w3.org/TR/mediaqueries-4/#hover>
    fn hoverMatches(&self, hover: &MediaHover) -> bool;

    /// <https://www.w3.org/TR/mediaqueries-4/#any-input>
    fn anyPointerMatches(&self, pointer: &MediaPointer) -> bool;

    /// <https://www.w3.org/TR/mediaqueries-4/#any-input>
    fn anyHoverMatches(&self, hover: &MediaHover) -> bool;

    /// <https://compat.spec.whatwg.org/#css-media-queries-webkit-transform-3d>
    fn transform3DMatches(&self, transform3D: &MediaTransform3D) -> bool;

    /// No Spec found
    fn prefers_color_scheme(&self, color_scheme: &ColorScheme) -> bool;

    /// No Spec found
    fn prefers_reduced_motion(&self, reduced_motion: &ReducedMotion) -> bool;
}
