// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.

use {
    super::{
        conversions::{
            FontRelativeLengthConversion,
            PercentageConversion,
            ViewportPercentageLengthConversion,
        },
        LengthUnit,
        PercentageUnit,
        Unit,
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
        CustomParseError,
    },
    cssparser::{ParseError, Parser, ParserInput, ToCss, Token},
    either::{Either, Left},
    std::{fmt, ops::*},
    LengthOrPercentageUnit::*,
};

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum LengthOrPercentageUnit<Number: CssNumber> {
    IsLength(LengthUnit<Number>),
    IsPercentage(PercentageUnit<Number>),
}

impl<Number: CssNumber> ToCss for LengthOrPercentageUnit<Number> {
    fn to_css<W: fmt::Write>(&self, dest: &mut W) -> fmt::Result {
        use self::LengthOrPercentageUnit::*;
        match *self {
            IsLength(ref length) => length.to_css(dest),
            IsPercentage(ref length) => length.to_css(dest),
        }
    }
}

impl<Number: CssNumber> Default for LengthOrPercentageUnit<Number> {
    #[inline(always)]
    fn default() -> Self {
        LengthOrPercentageUnit::IsLength(LengthUnit::default())
    }
}

impl<Number: CssNumber> Add<Number> for LengthOrPercentageUnit<Number> {
    type Output = Self;

    #[inline(always)]
    fn add(self, rhs: Number) -> Self::Output {
        use self::LengthOrPercentageUnit::*;
        match self {
            IsLength(length) => IsLength(length + rhs),
            IsPercentage(length) => IsPercentage(length + rhs),
        }
    }
}

impl<Number: CssNumber> AddAssign<Number> for LengthOrPercentageUnit<Number> {
    #[inline(always)]
    fn add_assign(&mut self, rhs: Number) {
        use self::LengthOrPercentageUnit::*;
        match *self {
            IsLength(ref mut length) => *length = *length + rhs,
            IsPercentage(ref mut length) => *length = *length + rhs,
        }
    }
}

impl<Number: CssNumber> Sub<Number> for LengthOrPercentageUnit<Number> {
    type Output = Self;

    #[inline(always)]
    fn sub(self, rhs: Number) -> Self::Output {
        use self::LengthOrPercentageUnit::*;
        match self {
            IsLength(length) => IsLength(length - rhs),
            IsPercentage(length) => IsPercentage(length - rhs),
        }
    }
}

impl<Number: CssNumber> SubAssign<Number> for LengthOrPercentageUnit<Number> {
    #[inline(always)]
    fn sub_assign(&mut self, rhs: Number) {
        use self::LengthOrPercentageUnit::*;
        match *self {
            IsLength(ref mut length) => *length = *length - rhs,
            IsPercentage(ref mut length) => *length = *length - rhs,
        }
    }
}

impl<Number: CssNumber> Mul<Number> for LengthOrPercentageUnit<Number> {
    type Output = Self;

    #[inline(always)]
    fn mul(self, rhs: Number) -> Self::Output {
        use self::LengthOrPercentageUnit::*;
        match self {
            IsLength(length) => IsLength(length * rhs),
            IsPercentage(length) => IsPercentage(length * rhs),
        }
    }
}

impl<Number: CssNumber> MulAssign<Number> for LengthOrPercentageUnit<Number> {
    #[inline(always)]
    fn mul_assign(&mut self, rhs: Number) {
        use self::LengthOrPercentageUnit::*;
        match *self {
            IsLength(ref mut length) => *length = *length * rhs,
            IsPercentage(ref mut length) => *length = *length * rhs,
        }
    }
}

impl<Number: CssNumber> Div<Number> for LengthOrPercentageUnit<Number> {
    type Output = Self;

    #[inline(always)]
    fn div(self, rhs: Number) -> Self::Output {
        use self::LengthOrPercentageUnit::*;
        match self {
            IsLength(length) => IsLength(length / rhs),
            IsPercentage(length) => IsPercentage(length / rhs),
        }
    }
}

impl<Number: CssNumber> DivAssign<Number> for LengthOrPercentageUnit<Number> {
    #[inline(always)]
    fn div_assign(&mut self, rhs: Number) {
        use self::LengthOrPercentageUnit::*;
        match *self {
            IsLength(ref mut length) => *length = *length / rhs,
            IsPercentage(ref mut length) => *length = *length / rhs,
        }
    }
}

impl<Number: CssNumber> Rem<Number> for LengthOrPercentageUnit<Number> {
    type Output = Self;

    #[inline(always)]
    fn rem(self, rhs: Number) -> Self::Output {
        use self::LengthOrPercentageUnit::*;
        match self {
            IsLength(length) => IsLength(length % rhs),
            IsPercentage(length) => IsPercentage(length % rhs),
        }
    }
}

impl<Number: CssNumber> RemAssign<Number> for LengthOrPercentageUnit<Number> {
    #[inline(always)]
    fn rem_assign(&mut self, rhs: Number) {
        use self::LengthOrPercentageUnit::*;
        match *self {
            IsLength(ref mut length) => *length = *length % rhs,
            IsPercentage(ref mut length) => *length = *length % rhs,
        }
    }
}

impl<Number: CssNumber> Neg for LengthOrPercentageUnit<Number> {
    type Output = Self;

    #[inline(always)]
    fn neg(self) -> Self::Output {
        use self::LengthOrPercentageUnit::*;
        match self {
            IsLength(length) => IsLength(-length),
            IsPercentage(length) => IsPercentage(-length),
        }
    }
}

impl<Number: CssNumber> CssNumberNewType<Number>
    for LengthOrPercentageUnit<Number>
{
    #[inline(always)]
    fn to_f32(&self) -> f32 {
        self.to_CssNumber().to_f32()
    }

    #[inline(always)]
    fn as_CssNumber(&self) -> &Number {
        use self::LengthOrPercentageUnit::*;
        match *self {
            IsLength(ref length) => length.as_CssNumber(),
            IsPercentage(ref length) => length.as_CssNumber(),
        }
    }
}

impl<NumberX: CssNumber> Unit for LengthOrPercentageUnit<NumberX> {
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
                return LengthUnit::parseUnitLessNumber(
                    value,
                    context.parsing_mode_allows_unitless_lengths(),
                )
                .map(|value| Constant(IsLength(value)))
            }

            Token::Dimension {
                value, ref unit, ..
            } => {
                return LengthUnit::parseDimension(
                    value,
                    unit,
                    context.isNotInPageRule(),
                )
                .map(|value| Constant(IsLength(value)))
            }

            Token::Percentage { unit_value, .. } => {
                return PercentageUnit::parse_percentage(unit_value)
                    .map(|value| Constant(IsPercentage(value)))
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

            Token::Dimension {
                value, ref unit, ..
            } => {
                return LengthUnit::parseDimension(
                    value,
                    unit,
                    context.isNotInPageRule(),
                )
                .map(|value| Left(Constant(IsLength(value))))
            }

            Token::Percentage { unit_value, .. } => {
                return PercentageUnit::parse_percentage(unit_value)
                    .map(|value| Left(Constant(IsPercentage(value))))
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
    fn to_canonical_dimension_value<
        Conversion: FontRelativeLengthConversion<Self::Number>
            + ViewportPercentageLengthConversion<Self::Number>
            + PercentageConversion<Self::Number>,
    >(
        &self,
        conversion: &Conversion,
    ) -> Self::Number {
        use self::LengthOrPercentageUnit::*;

        match *self {
            IsLength(ref length) => {
                length.to_canonical_dimension_value(conversion)
            }
            IsPercentage(ref percentage) => {
                percentage.to_canonical_dimension_value(conversion)
            }
        }
    }

    #[inline(always)]
    fn from_raw_css_for_var_expression_evaluation(
        value: &str,
        is_not_in_page_rule: bool,
    ) -> Option<Self> {
        use self::LengthOrPercentageUnit::*;

        fn from_raw_css_for_var_expression_evaluation_internal<
            'i: 't,
            't,
            Number: CssNumber,
        >(
            is_not_in_page_rule: bool,
            input: &mut Parser<'i, 't>,
        ) -> Result<
            LengthOrPercentageUnit<Number>,
            ParseError<'i, CustomParseError<'i>>,
        > {
            let value = match *input.next()? {
                Token::Number { value, .. } => {
                    LengthUnit::parseUnitLessNumber(value, false).map(IsLength)
                }

                Token::Percentage { unit_value, .. } => {
                    PercentageUnit::parse_percentage(unit_value)
                        .map(|value| IsPercentage(value))
                }

                Token::Dimension {
                    value, ref unit, ..
                } => {
                    LengthUnit::parseDimension(value, unit, is_not_in_page_rule)
                        .map(IsLength)
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

impl<Number: CssNumber> LengthOrPercentageUnit<Number> {
    /// Get an absolute length using a conversion
    #[inline(always)]
    pub fn to_px<
        Conversion: FontRelativeLengthConversion<Number>
            + ViewportPercentageLengthConversion<Number>
            + PercentageConversion<Number>,
    >(
        &self,
        conversion: &Conversion,
    ) -> Number {
        use self::LengthOrPercentageUnit::*;

        match *self {
            IsLength(ref length) => length.to_px(conversion),
            IsPercentage(ref percentage) => {
                percentage.to_absolute_value(conversion)
            }
        }
    }

    /// Convert this into AppUnits.
    #[inline]
    pub fn to_app_units<
        Conversion: FontRelativeLengthConversion<Number>
            + ViewportPercentageLengthConversion<Number>
            + PercentageConversion<Number>,
    >(
        &self,
        conversion: &Conversion,
    ) -> Number {
        self.to_px(conversion) * Number::AppUnitsPerPX
    }
}
