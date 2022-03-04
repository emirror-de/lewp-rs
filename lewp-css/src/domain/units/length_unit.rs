// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.

use {
    super::{
        conversions::{
            FontRelativeLengthConversion,
            PercentageConversion,
            ViewportPercentageLengthConversion,
        },
        AbsoluteLength::{self, *},
        FontRelativeLength::{self, *},
        LengthUnit::*,
        PercentageUnit,
        Unit,
        ViewportPercentageLength::{self, *},
    },
    crate::{
        domain::{
            expressions::{
                CalcExpression,
                CalculablePropertyValue::{self, *},
                FunctionParser,
            },
            numbers::{CssNumber, CssNumberNewType},
        },
        parsers::ParserContext,
        CustomParseError::{self, *},
    },
    cssparser::{CowRcStr, ParseError, Parser, ParserInput, ToCss, Token},
    either::{Either, Left},
    std::{fmt, ops::*},
};

/// A `<length>` without taking `calc` expressions into account
///
/// <https://drafts.csswg.org/css-values/#lengths>
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum LengthUnit<Number: CssNumber> {
    /// An absolute length
    ///
    /// <https://drafts.csswg.org/css-values/#absolute-length>
    Absolute(AbsoluteLength<Number>),

    /// A font-relative length:
    ///
    /// <https://drafts.csswg.org/css-values/#font-relative-lengths>
    FontRelative(FontRelativeLength<Number>),

    /// A viewport-relative length.
    /// Not valid in @page rules (the parser enforces this)
    ///
    /// <https://drafts.csswg.org/css-values/#viewport-relative-lengths>
    ViewportPercentage(ViewportPercentageLength<Number>),
}

impl<Number: CssNumber> ToCss for LengthUnit<Number> {
    fn to_css<W: fmt::Write>(&self, dest: &mut W) -> fmt::Result {
        match *self {
            Absolute(ref length) => length.to_css(dest),
            FontRelative(ref length) => length.to_css(dest),
            ViewportPercentage(ref length) => length.to_css(dest),
        }
    }
}

impl<Number: CssNumber> Default for LengthUnit<Number> {
    #[inline(always)]
    fn default() -> Self {
        Absolute(AbsoluteLength::default())
    }
}

impl<Number: CssNumber> Add<Number> for LengthUnit<Number> {
    type Output = Self;

    #[inline(always)]
    fn add(self, rhs: Number) -> Self::Output {
        match self {
            Absolute(length) => Absolute(length + rhs),
            FontRelative(length) => FontRelative(length + rhs),
            ViewportPercentage(length) => ViewportPercentage(length + rhs),
        }
    }
}

impl<Number: CssNumber> AddAssign<Number> for LengthUnit<Number> {
    #[inline(always)]
    fn add_assign(&mut self, rhs: Number) {
        match *self {
            Absolute(ref mut length) => *length = *length + rhs,
            FontRelative(ref mut length) => *length = *length + rhs,
            ViewportPercentage(ref mut length) => *length = *length + rhs,
        }
    }
}

impl<Number: CssNumber> Sub<Number> for LengthUnit<Number> {
    type Output = Self;

    #[inline(always)]
    fn sub(self, rhs: Number) -> Self::Output {
        match self {
            Absolute(length) => Absolute(length - rhs),
            FontRelative(length) => FontRelative(length - rhs),
            ViewportPercentage(length) => ViewportPercentage(length - rhs),
        }
    }
}

impl<Number: CssNumber> SubAssign<Number> for LengthUnit<Number> {
    #[inline(always)]
    fn sub_assign(&mut self, rhs: Number) {
        match *self {
            Absolute(ref mut length) => *length = *length - rhs,
            FontRelative(ref mut length) => *length = *length - rhs,
            ViewportPercentage(ref mut length) => *length = *length - rhs,
        }
    }
}

impl<Number: CssNumber> Mul<Number> for LengthUnit<Number> {
    type Output = Self;

    #[inline(always)]
    fn mul(self, rhs: Number) -> Self::Output {
        match self {
            Absolute(length) => Absolute(length * rhs),
            FontRelative(length) => FontRelative(length * rhs),
            ViewportPercentage(length) => ViewportPercentage(length * rhs),
        }
    }
}

impl<Number: CssNumber> MulAssign<Number> for LengthUnit<Number> {
    #[inline(always)]
    fn mul_assign(&mut self, rhs: Number) {
        match *self {
            Absolute(ref mut length) => *length = *length * rhs,
            FontRelative(ref mut length) => *length = *length * rhs,
            ViewportPercentage(ref mut length) => *length = *length * rhs,
        }
    }
}

impl<Number: CssNumber> Div<Number> for LengthUnit<Number> {
    type Output = Self;

    #[inline(always)]
    fn div(self, rhs: Number) -> Self::Output {
        match self {
            Absolute(length) => Absolute(length / rhs),
            FontRelative(length) => FontRelative(length / rhs),
            ViewportPercentage(length) => ViewportPercentage(length / rhs),
        }
    }
}

impl<Number: CssNumber> DivAssign<Number> for LengthUnit<Number> {
    #[inline(always)]
    fn div_assign(&mut self, rhs: Number) {
        match *self {
            Absolute(ref mut length) => *length = *length / rhs,
            FontRelative(ref mut length) => *length = *length / rhs,
            ViewportPercentage(ref mut length) => *length = *length / rhs,
        }
    }
}

impl<Number: CssNumber> Rem<Number> for LengthUnit<Number> {
    type Output = LengthUnit<Number>;

    #[inline(always)]
    fn rem(self, rhs: Number) -> Self::Output {
        match self {
            Absolute(length) => Absolute(length % rhs),
            FontRelative(length) => FontRelative(length % rhs),
            ViewportPercentage(length) => ViewportPercentage(length % rhs),
        }
    }
}

impl<Number: CssNumber> RemAssign<Number> for LengthUnit<Number> {
    #[inline(always)]
    fn rem_assign(&mut self, rhs: Number) {
        match *self {
            Absolute(ref mut length) => *length = *length % rhs,
            FontRelative(ref mut length) => *length = *length % rhs,
            ViewportPercentage(ref mut length) => *length = *length % rhs,
        }
    }
}

impl<Number: CssNumber> Neg for LengthUnit<Number> {
    type Output = LengthUnit<Number>;

    #[inline(always)]
    fn neg(self) -> Self::Output {
        match self {
            Absolute(length) => Absolute(-length),
            FontRelative(length) => FontRelative(-length),
            ViewportPercentage(length) => ViewportPercentage(-length),
        }
    }
}

impl<Number: CssNumber> CssNumberNewType<Number> for LengthUnit<Number> {
    #[inline(always)]
    fn to_f32(&self) -> f32 {
        self.to_CssNumber().to_f32()
    }

    #[inline(always)]
    fn as_CssNumber(&self) -> &Number {
        match *self {
            Absolute(ref length) => length.as_CssNumber(),
            FontRelative(ref length) => length.as_CssNumber(),
            ViewportPercentage(ref length) => length.as_CssNumber(),
        }
    }
}

impl<NumberX: CssNumber> Unit for LengthUnit<NumberX> {
    type Number = NumberX;

    const HasDimension: bool = true;

    #[inline(always)]
    fn parse_one_outside_calc_function<'i, 't>(
        context: &ParserContext,
        input: &mut Parser<'i, 't>,
    ) -> Result<
        CalculablePropertyValue<Self>,
        ParseError<'i, CustomParseError<'i>>,
    > {
        let functionParser = match *input.next()? {
            Token::Number { value, .. } => {
                return Self::parseUnitLessNumber(
                    value,
                    context.parsing_mode_allows_unitless_lengths(),
                )
                .map(Constant)
            }

            Token::Dimension {
                value, ref unit, ..
            } => {
                return Self::parseDimension(
                    value,
                    unit,
                    context.isNotInPageRule(),
                )
                .map(Constant)
            }

            Token::Function(ref name) => FunctionParser::parser(name)?,

            ref unexpectedToken => {
                return CustomParseError::unexpectedToken(unexpectedToken)
            }
        };
        functionParser.parse_one_outside_calc_function(context, input)
    }

    #[inline(always)]
    fn parse_one_inside_calc_function<'i, 't>(
        context: &ParserContext,
        input: &mut Parser<'i, 't>,
    ) -> Result<
        Either<CalculablePropertyValue<Self>, CalcExpression<Self>>,
        ParseError<'i, CustomParseError<'i>>,
    > {
        let functionParser = match *input.next()? {
            Token::Number { value, .. } => {
                return Self::number_inside_calc_function(value)
            }

            Token::Percentage { unit_value, .. } => {
                return PercentageUnit::parse_percentage(unit_value)
                    .map(|value| Left(Percentage(value)))
            }

            Token::Dimension {
                value, ref unit, ..
            } => {
                return Self::parseDimension(
                    value,
                    unit,
                    context.isNotInPageRule(),
                )
                .map(|value| Left(Constant(value)))
            }

            Token::ParenthesisBlock => FunctionParser::parentheses,

            Token::Function(ref name) => FunctionParser::parser(name)?,

            ref unexpectedToken => {
                return CustomParseError::unexpectedToken(unexpectedToken)
            }
        };
        functionParser.parse_one_inside_calc_function(context, input)
    }

    #[inline(always)]
    fn to_canonical_dimension(self) -> Self {
        match self {
            Absolute(ref length) => Absolute(px(length.to_px())),
            unchanged => unchanged,
        }
    }

    #[inline(always)]
    fn to_canonical_dimension_value<
        Conversion: FontRelativeLengthConversion<Self::Number>
            + ViewportPercentageLengthConversion<Self::Number>
            + PercentageConversion<Self::Number>,
    >(
        &self,
        conversion: &Conversion,
    ) -> Self::Number {
        self.to_px(conversion)
    }

    #[inline(always)]
    fn from_raw_css_for_var_expression_evaluation(
        value: &str,
        is_not_in_page_rule: bool,
    ) -> Option<Self> {
        fn from_raw_css_for_var_expression_evaluation_internal<
            'i: 't,
            't,
            Number: CssNumber,
        >(
            is_not_in_page_rule: bool,
            input: &mut Parser<'i, 't>,
        ) -> Result<LengthUnit<Number>, ParseError<'i, CustomParseError<'i>>>
        {
            let value = match *input.next()? {
                Token::Number { value, .. } => {
                    LengthUnit::parseUnitLessNumber(value, false)
                }

                Token::Dimension {
                    value, ref unit, ..
                } => {
                    LengthUnit::parseDimension(value, unit, is_not_in_page_rule)
                }

                ref unexpectedToken => {
                    CustomParseError::unexpectedToken(unexpectedToken)
                }
            };

            input.skip_whitespace();

            input.expect_exhausted()?;

            value
        }

        const LineNumberingIsZeroBased: u32 = 0;

        let mut parserInput = ParserInput::new_with_line_number_offset(
            value,
            LineNumberingIsZeroBased,
        );
        let mut input = Parser::new(&mut parserInput);

        from_raw_css_for_var_expression_evaluation_internal(
            is_not_in_page_rule,
            &mut input,
        )
        .ok()
    }
}

impl<Number: CssNumber> LengthUnit<Number> {
    /// Checks whether the length value is zero.
    #[inline]
    pub fn is_absolute_zero(&self) -> bool {
        match *self {
            Absolute(length) => length.is_zero(),
            _ => false,
        }
    }

    #[inline(always)]
    pub fn is_absolute_length(&self) -> bool {
        match *self {
            Absolute(..) => true,
            FontRelative(..) | ViewportPercentage(..) => false,
        }
    }

    /// Convert this into a pixel value.
    #[inline(always)]
    pub fn to_px<
        Conversion: FontRelativeLengthConversion<Number>
            + ViewportPercentageLengthConversion<Number>,
    >(
        &self,
        conversion: &Conversion,
    ) -> Number {
        match *self {
            Absolute(ref length) => length.to_px(),
            FontRelative(ref length) => length.to_px(conversion),
            ViewportPercentage(ref length) => length.to_px(conversion),
        }
    }

    /// Convert this into AppUnits.
    #[inline]
    pub fn to_app_units<
        Conversion: FontRelativeLengthConversion<Number>
            + ViewportPercentageLengthConversion<Number>,
    >(
        &self,
        conversion: &Conversion,
    ) -> Number {
        match *self {
            Absolute(ref length) => length.to_app_units(),
            FontRelative(ref length) => length.to_app_units(conversion),
            ViewportPercentage(ref length) => length.to_app_units(conversion),
        }
    }

    #[inline(always)]
    pub(crate) fn parseUnitLessNumber<'i>(
        value: f32,
        parsing_mode_allows_unitless_lengths: bool,
    ) -> Result<Self, ParseError<'i, CustomParseError<'i>>> {
        if value == 0. {
            Ok(Self::default())
        } else if parsing_mode_allows_unitless_lengths {
            let cssNumber =
                Number::new(value).map_err(|cssNumberConversionError| {
                    ParseError::from(CouldNotParseCssSignedNumber(
                        cssNumberConversionError,
                        value,
                    ))
                })?;
            Ok(Absolute(px(cssNumber)))
        } else {
            CustomParseError::dimensionless(value)
        }
    }

    #[inline(always)]
    pub(crate) fn parseDimension<'i>(
        value: f32,
        unit: &CowRcStr<'i>,
        is_not_in_page_rule: bool,
    ) -> Result<Self, ParseError<'i, CustomParseError<'i>>> {
        let cssNumber =
            Number::new(value).map_err(|cssNumberConversionError| {
                ParseError::from(CouldNotParseCssSignedNumber(
                    cssNumberConversionError,
                    value,
                ))
            })?;

        match_ignore_ascii_case! {
            &*unit,

            "px" => Ok(Absolute(px(cssNumber))),

            "in" => Ok(Absolute(in_(cssNumber))),

            "cm" => Ok(Absolute(cm(cssNumber))),

            "mm" => Ok(Absolute(mm(cssNumber))),

            "q" => Ok(Absolute(q(cssNumber))),

            "pt" => Ok(Absolute(pt(cssNumber))),

            "pc" => Ok(Absolute(pc(cssNumber))),

            "em" => if is_not_in_page_rule
            {
                Ok(FontRelative(em(cssNumber)))
            }
            else
            {
                Err(ParseError::from(CustomParseError::FontRelativeLengthsAreNotAllowedInAPageAtRule))
            },

            "ex" => if is_not_in_page_rule
            {
                Ok(FontRelative(ex(cssNumber)))
            }
            else
            {
                Err(ParseError::from(CustomParseError::FontRelativeLengthsAreNotAllowedInAPageAtRule))
            },

            "ch" => Ok(FontRelative(ch(cssNumber))),

            "rem" => Ok(FontRelative(rem(cssNumber))),

            "vw" => if is_not_in_page_rule
            {
                Ok(ViewportPercentage(vw(cssNumber)))
            }
            else
            {
                Err(ParseError::from(CustomParseError::ViewportLengthsAreNotAllowedInAPageAtRule))
            },

            "vh" => if is_not_in_page_rule
            {
                Ok(ViewportPercentage(vh(cssNumber)))
            }
            else
            {
                Err(ParseError::from(CustomParseError::ViewportLengthsAreNotAllowedInAPageAtRule))
            },

            "vmin" => if is_not_in_page_rule
            {
                Ok(ViewportPercentage(vmin(cssNumber)))
            }
            else
            {
                Err(ParseError::from(CustomParseError::ViewportLengthsAreNotAllowedInAPageAtRule))
            },

            "vmax" => if is_not_in_page_rule
            {
                Ok(ViewportPercentage(vmax(cssNumber)))
            }
            else
            {
                Err(ParseError::from(CustomParseError::ViewportLengthsAreNotAllowedInAPageAtRule))
            },

            _ => Err(ParseError::from(CouldNotParseDimension(value, unit.clone()))),
        }
    }
}
