// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.

use {
    crate::{parsers::{Parse, ParserContext}, CustomParseError, domain::units::{Unit,LengthUnit}},
    super::{MediaExpressionKind::{self, *}, Ratio, MediaResolution, MediaGrid, ColorBitDepth, MediaOrientation, MediaScan, MediaUpdate, MediaOverflowBlock, MediaOverflowInline, MediaColorGamut, MediaPointer, MediaHover, MediaTransform3D, Device, MediaColorIndex, MonochromeBitDepth, ColorScheme, ReducedMotion},
    super::Range::*,
    cssparser::{ParseError, Parser, ToCss},
    std::fmt,
};

/// A single expression as per <http://dev.w3.org/csswg/mediaqueries-3/#media1>
#[derive(Clone, Debug, PartialEq)]
pub struct MediaExpression(pub MediaExpressionKind);

impl ToCss for MediaExpression
{
	fn to_css<W: fmt::Write>(&self, dest: &mut W) -> fmt::Result
	{
		#[inline(always)]
		fn write<W: fmt::Write, T: ToCss>(dest: &mut W, name: &str, value: &T) -> fmt::Result
		{
			dest.write_char('(')?;
			dest.write_str(name)?;
			dest.write_char(':')?;
			value.to_css(dest)?;
			dest.write_char(')')
		}
		
		match self.0
		{
			Width(AtLeast(ref value)) => write(dest, "min-width", value),
			
			Width(AtMost(ref value)) => write(dest, "max-width", value),
			
			Width(Exact(ref value)) => write(dest, "width", value),
			
			Height(AtLeast(ref value)) => write(dest, "min-height", value),
			
			Height(AtMost(ref value)) => write(dest, "max-height", value),
			
			Height(Exact(ref value)) => write(dest, "height", value),
			
			AspectRatio(AtLeast(ref value)) => write(dest, "min-aspect-ratio", value),
			
			AspectRatio(AtMost(ref value)) => write(dest, "max-aspect-ratio", value),
			
			AspectRatio(Exact(ref value)) => write(dest, "aspect-ratio", value),
			
			Orientation(ref value) => write(dest, "orientation", value),
			
			Resolution(AtLeast(ref value)) => write(dest, "min-resolution", value),
			
			Resolution(AtMost(ref value)) => write(dest, "max-resolution", value),
			
			Resolution(Exact(ref value)) => write(dest, "resolution", value),
			
			Scan(ref value) => write(dest, "scan", value),
			
			Grid(ref value) => write(dest, "grid", value),
			
			Update(ref value) => write(dest, "update", value),
			
			OverflowBlock(ref value) => write(dest, "overflow-block", value),
			
			OverflowInline(ref value) => write(dest, "overflow-inline", value),
			
			Color(AtLeast(ref value)) => write(dest, "min-color", value),
			
			Color(AtMost(ref value)) => write(dest, "max-color", value),
			
			Color(Exact(ref value)) => write(dest, "color", value),
			
			ColorIndex(AtLeast(ref value)) => write(dest, "min-color-index", value),
			
			ColorIndex(AtMost(ref value)) => write(dest, "max-color-index", value),
			
			ColorIndex(Exact(ref value)) => write(dest, "color-index", value),
			
			Monochrome(AtLeast(ref value)) => write(dest, "min-monochrome", value),
			
			Monochrome(AtMost(ref value)) => write(dest, "max-monochrome", value),
			
			Monochrome(Exact(ref value)) => write(dest, "monochrome", value),
			
			ColorGamut(ref value) => write(dest, "color-gamut", value),
			
			Pointer(ref value) => write(dest, "pointer", value),
			
			Hover(ref value) => write(dest, "hover", value),
			
			AnyPointer(ref value) => write(dest, "any-pointer", value),
			
			AnyHover(ref value) => write(dest, "any-hover", value),
			
			Transform3D(ref value) => write(dest, "-webkit-transform-3d", value),
			
			PrefersColorScheme(ref value) => write(dest, "prefers-color-scheme", value),
			
			PrefersReducedMotion(ref value) => write(dest, "prefers-reduced-motion", value),
		}
	}
}

impl MediaExpression
{
	pub(crate) fn parse<'i, 't>(context: &ParserContext, input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i, CustomParseError<'i>>>
	{
		input.expect_parenthesis_block()?;
		input.parse_nested_block(|input|
		{
			let name = input.expect_ident_cloned()?;
			input.expect_colon()?;
			
			Ok
			(
				MediaExpression
				(
					match_ignore_ascii_case!
					{
						&name,
						
						"min-width" => Width(AtLeast(LengthUnit::parse_one_outside_calc_function(context, input)?)),
						
						"max-width" => Width(AtMost(LengthUnit::parse_one_outside_calc_function(context, input)?)),
						
						"width" => Width(Exact(LengthUnit::parse_one_outside_calc_function(context, input)?)),
						
						"min-height" => Height(AtLeast(LengthUnit::parse_one_outside_calc_function(context, input)?)),
						
						"max-height" => Height(AtMost(LengthUnit::parse_one_outside_calc_function(context, input)?)),
						
						"height" => Height(Exact(LengthUnit::parse_one_outside_calc_function(context, input)?)),
						
						"min-aspect-ratio" => AspectRatio(AtLeast(Ratio::parse(context, input)?)),
						
						"max-aspect-ratio" => AspectRatio(AtMost(Ratio::parse(context, input)?)),
						
						"aspect-ratio" => AspectRatio(Exact(Ratio::parse(context, input)?)),
						
						"orientation" => Orientation(MediaOrientation::parse(input)?),
						
						"min-resolution" => Resolution(AtLeast(MediaResolution::parse(context, input)?)),
						
						"max-resolution" => Resolution(AtLeast(MediaResolution::parse(context, input)?)),
						
						"resolution" => Resolution(Exact(MediaResolution::parse(context, input)?)),
						
						"-webkit-min-device-pixel-ratio" => Resolution(AtLeast(MediaResolution::parseWebKit(input)?)),
						
						"-webkit-max-device-pixel-ratio" => Resolution(AtMost(MediaResolution::parseWebKit(input)?)),
						
						"-webkit-device-pixel-ratio" => Resolution(Exact(MediaResolution::parseWebKit(input)?)),
						
						"scan" => Scan(MediaScan::parse(input)?),
						
						"grid" => Grid(MediaGrid::parse(context, input)?),
						
						"update" => Update(MediaUpdate::parse(input)?),
						
						"overflow-block" => OverflowBlock(MediaOverflowBlock::parse(input)?),
						
						"overflow-inline" => OverflowInline(MediaOverflowInline::parse(input)?),
						
						"min-color" => Color(AtLeast(ColorBitDepth::parse(context, input)?)),
						
						"max-color" => Color(AtMost(ColorBitDepth::parse(context, input)?)),
						
						"color" => Color(Exact(ColorBitDepth::parse(context, input)?)),
						
						"min-color-index" => ColorIndex(AtLeast(MediaColorIndex::parse(context, input)?)),
						
						"max-color-index" => ColorIndex(AtMost(MediaColorIndex::parse(context, input)?)),
						
						"color-index" => ColorIndex(Exact(MediaColorIndex::parse(context, input)?)),
						
						"min-monochrome" => Monochrome(AtLeast(MonochromeBitDepth::parse(context, input)?)),
						
						"max-monochrome" => Monochrome(AtMost(MonochromeBitDepth::parse(context, input)?)),
						
						"monochrome" => Monochrome(Exact(MonochromeBitDepth::parse(context, input)?)),
						
						"color-gamut" => ColorGamut(MediaColorGamut::parse(input)?),
						
						"pointer" => Pointer(MediaPointer::parse(input)?),
						
						"hover" => Hover(MediaHover::parse(input)?),
						
						"any-pointer" => AnyPointer(MediaPointer::parse(input)?),
						
						"any-hover" => AnyHover(MediaHover::parse(input)?),
						
						"-webkit-transform-3d" => Transform3D(MediaTransform3D::parse(context, input)?),
						
						"prefers-color-scheme" => PrefersColorScheme(ColorScheme::parse(input)?),
						
						"prefers-reduced-motion" => PrefersReducedMotion(ReducedMotion::parse(input)?),
						
						"min-device-width" | "max-device-width" | "device-width" | "min-device-height" | "max-device-height" | "device-height" | "min-device-aspect-ratio" | "max-device-aspect-ratio" | "device-aspect-ratio" => return Err(ParseError::from(CustomParseError::DeprecatedMediaQueryExpression(name.clone()))),
						
						_ => return Err(ParseError::from(CustomParseError::UnsupportedMediaQueryExpression(name.clone())))
					}
				)
			)
		})
	}
	
	/// Evaluate this expression and return whether it matches the current device.
	pub fn matches<D: Device>(&self, device: &D) -> bool
	{
		use self::MediaExpressionKind::*;
		
		match self.0
		{
			Width(ref range) => device.viewportWidthMatches(range),
			
			Height(ref range) => device.viewportHeightMatches(range),
			
			AspectRatio(ref range) => device.viewportAspectRatioMatches(range),
			
			Orientation(orientation) => device.orientationMatches(orientation),
			
			Resolution(ref range) => device.viewportResolutionMatches(range),
			
			Scan(ref scan) => device.scanMatches(scan),
			
			Grid(ref grid) => device.gridMatches(grid),
			
			Update(ref update) => device.updateMatches(update),
			
			OverflowBlock(ref overflowBlock) => device.overflowBlockMatches(overflowBlock),
			
			OverflowInline(ref overflowInline) => device.overflowInlineMatches(overflowInline),
			
			Color(ref range) => device.colorBitDepthMatches(range),
			
			ColorIndex(ref range) => device.colorIndexMatches(range),
			
			Monochrome(ref range) => device.monochromeBitDepthMatches(range),
			
			ColorGamut(ref colorGamut) => device.colorGamutMatches(colorGamut),
			
			Pointer(ref pointer) => device.pointerMatches(pointer),
			
			Hover(ref hover) => device.hoverMatches(hover),
			
			AnyPointer(ref pointer) => device.anyPointerMatches(pointer),
			
			AnyHover(ref hover) => device.anyHoverMatches(hover),
			
			Transform3D(ref transform3D) => device.transform3DMatches(transform3D),
			
			PrefersColorScheme(ref prefers_color_scheme) => device.prefers_color_scheme(prefers_color_scheme),
			
			PrefersReducedMotion(ref prefers_reduced_motion) => device.prefers_reduced_motion(prefers_reduced_motion),
		}
	}
}
