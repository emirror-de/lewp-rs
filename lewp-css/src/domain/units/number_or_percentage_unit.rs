// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.

use {
    super::{
        conversions::{
            FontRelativeLengthConversion,
            PercentageConversion,
            ViewportPercentageLengthConversion,
        },
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
    NumberOrPercentageUnit::*,
};

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum NumberOrPercentageUnit<Number: CssNumber> {
    IsNumber(Number),
    IsPercentage(PercentageUnit<Number>),
}

impl<Number: CssNumber> ToCss for NumberOrPercentageUnit<Number> {
    fn to_css<W: fmt::Write>(&self, dest: &mut W) -> fmt::Result {
        use self::NumberOrPercentageUnit::*;
        match *self {
            IsNumber(ref length) => length.to_css(dest),
            IsPercentage(ref length) => length.to_css(dest),
        }
    }
}

impl<Number: CssNumber> Default for NumberOrPercentageUnit<Number> {
    #[inline(always)]
    fn default() -> Self {
        NumberOrPercentageUnit::IsNumber(Number::default())
    }
}

impl<Number: CssNumber> Add<Number> for NumberOrPercentageUnit<Number> {
    type Output = Self;

    #[inline(always)]
    fn add(self, rhs: Number) -> Self::Output {
        use self::NumberOrPercentageUnit::*;
        match self {
            IsNumber(length) => IsNumber(length + rhs),
            IsPercentage(length) => IsPercentage(length + rhs),
        }
    }
}

impl<Number: CssNumber> AddAssign<Number> for NumberOrPercentageUnit<Number> {
    #[inline(always)]
    fn add_assign(&mut self, rhs: Number) {
        use self::NumberOrPercentageUnit::*;
        match *self {
            IsNumber(ref mut length) => *length = *length + rhs,
            IsPercentage(ref mut length) => *length = *length + rhs,
        }
    }
}

impl<Number: CssNumber> Sub<Number> for NumberOrPercentageUnit<Number> {
    type Output = Self;

    #[inline(always)]
    fn sub(self, rhs: Number) -> Self::Output {
        use self::NumberOrPercentageUnit::*;
        match self {
            IsNumber(length) => IsNumber(length - rhs),
            IsPercentage(length) => IsPercentage(length - rhs),
        }
    }
}

impl<Number: CssNumber> SubAssign<Number> for NumberOrPercentageUnit<Number> {
    #[inline(always)]
    fn sub_assign(&mut self, rhs: Number) {
        use self::NumberOrPercentageUnit::*;
        match *self {
            IsNumber(ref mut length) => *length = *length - rhs,
            IsPercentage(ref mut length) => *length = *length - rhs,
        }
    }
}

impl<Number: CssNumber> Mul<Number> for NumberOrPercentageUnit<Number> {
    type Output = Self;

    #[inline(always)]
    fn mul(self, rhs: Number) -> Self::Output {
        use self::NumberOrPercentageUnit::*;
        match self {
            IsNumber(length) => IsNumber(length * rhs),
            IsPercentage(length) => IsPercentage(length * rhs),
        }
    }
}

impl<Number: CssNumber> MulAssign<Number> for NumberOrPercentageUnit<Number> {
    #[inline(always)]
    fn mul_assign(&mut self, rhs: Number) {
        use self::NumberOrPercentageUnit::*;
        match *self {
            IsNumber(ref mut length) => *length = *length * rhs,
            IsPercentage(ref mut length) => *length = *length * rhs,
        }
    }
}

impl<Number: CssNumber> Div<Number> for NumberOrPercentageUnit<Number> {
    type Output = Self;

    #[inline(always)]
    fn div(self, rhs: Number) -> Self::Output {
        use self::NumberOrPercentageUnit::*;
        match self {
            IsNumber(length) => IsNumber(length / rhs),
            IsPercentage(length) => IsPercentage(length / rhs),
        }
    }
}

impl<Number: CssNumber> DivAssign<Number> for NumberOrPercentageUnit<Number> {
    #[inline(always)]
    fn div_assign(&mut self, rhs: Number) {
        use self::NumberOrPercentageUnit::*;
        match *self {
            IsNumber(ref mut length) => *length = *length / rhs,
            IsPercentage(ref mut length) => *length = *length / rhs,
        }
    }
}

impl<Number: CssNumber> Rem<Number> for NumberOrPercentageUnit<Number> {
    type Output = Self;

    #[inline(always)]
    fn rem(self, rhs: Number) -> Self::Output {
        use self::NumberOrPercentageUnit::*;
        match self {
            IsNumber(length) => IsNumber(length % rhs),
            IsPercentage(length) => IsPercentage(length % rhs),
        }
    }
}

impl<Number: CssNumber> RemAssign<Number> for NumberOrPercentageUnit<Number> {
    #[inline(always)]
    fn rem_assign(&mut self, rhs: Number) {
        use self::NumberOrPercentageUnit::*;
        match *self {
            IsNumber(ref mut length) => *length = *length % rhs,
            IsPercentage(ref mut length) => *length = *length % rhs,
        }
    }
}

impl<Number: CssNumber> Neg for NumberOrPercentageUnit<Number> {
    type Output = Self;

    #[inline(always)]
    fn neg(self) -> Self::Output {
        use self::NumberOrPercentageUnit::*;
        match self {
            IsNumber(length) => IsNumber(-length),
            IsPercentage(length) => IsPercentage(-length),
        }
    }
}

impl<Number: CssNumber> CssNumberNewType<Number>
    for NumberOrPercentageUnit<Number>
{
    #[inline(always)]
    fn to_f32(&self) -> f32 {
        self.to_CssNumber().to_f32()
    }

    #[inline(always)]
    fn as_CssNumber(&self) -> &Number {
        use self::NumberOrPercentageUnit::*;
        match *self {
            IsNumber(ref length) => length.as_CssNumber(),
            IsPercentage(ref length) => length.as_CssNumber(),
        }
    }
}

impl<NumberX: CssNumber> Unit for NumberOrPercentageUnit<NumberX> {
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
            Token::Number {
                value, int_value, ..
            } => {
                return Self::Number::parseNumber(value, int_value)
                    .map(|value| Constant(IsNumber(value)))
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
            Token::Number {
                value, int_value, ..
            } => {
                return Self::Number::parseNumber(value, int_value)
                    .map(|value| Left(Constant(IsNumber(value))))
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
        use self::NumberOrPercentageUnit::*;

        match *self {
            IsNumber(ref length) => {
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
        _is_not_in_page_rule: bool,
    ) -> Option<Self> {
        use self::NumberOrPercentageUnit::*;

        fn from_raw_css_for_var_expression_evaluation_internal<
            'i: 't,
            't,
            Number: CssNumber,
        >(
            input: &mut Parser<'i, 't>,
        ) -> Result<
            NumberOrPercentageUnit<Number>,
            ParseError<'i, CustomParseError<'i>>,
        > {
            let value = match *input.next()? {
                Token::Number {
                    value, int_value, ..
                } => Number::parseNumber(value, int_value).map(IsNumber),

                Token::Percentage { unit_value, .. } => {
                    PercentageUnit::parse_percentage(unit_value)
                        .map(|value| IsPercentage(value))
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

        from_raw_css_for_var_expression_evaluation_internal(&mut input).ok()
    }
}

impl<Number: CssNumber> NumberOrPercentageUnit<Number> {
    /// Get an absolute number using a conversion
    #[inline(always)]
    pub fn to_number<Conversion: PercentageConversion<Number>>(
        &self,
        conversion: &Conversion,
    ) -> Number {
        use self::NumberOrPercentageUnit::*;

        match *self {
            IsNumber(number) => number,
            IsPercentage(percentage) => {
                percentage.to_absolute_value(conversion)
            }
        }
    }
}
