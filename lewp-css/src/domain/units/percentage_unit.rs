// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.

use {
    super::{
        conversions::{
            FontRelativeLengthConversion,
            PercentageConversion,
            ViewportPercentageLengthConversion,
        },
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
        serializers::serialize_percentage::serialize_percentage,
        CustomParseError::{self, *},
    },
    cssparser::{ParseError, Parser, ParserInput, ToCss, Token},
    either::{Either, Left},
    std::{fmt, ops::*},
};

/// A percentage
#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct PercentageUnit<Number: CssNumber>(pub Number);

impl<Number: CssNumber> ToCss for PercentageUnit<Number> {
    fn to_css<W: fmt::Write>(&self, dest: &mut W) -> fmt::Result {
        serialize_percentage(self.0, dest)
    }
}

impl<Number: CssNumber> Default for PercentageUnit<Number> {
    #[inline(always)]
    fn default() -> Self {
        PercentageUnit(Number::default())
    }
}

impl<Number: CssNumber> Add<Number> for PercentageUnit<Number> {
    type Output = Self;

    #[inline(always)]
    fn add(self, rhs: Number) -> Self::Output {
        PercentageUnit(self.0 + rhs)
    }
}

impl<Number: CssNumber> AddAssign<Number> for PercentageUnit<Number> {
    #[inline(always)]
    fn add_assign(&mut self, rhs: Number) {
        *self = PercentageUnit(self.0 + rhs);
    }
}

impl<Number: CssNumber> Sub<Number> for PercentageUnit<Number> {
    type Output = Self;

    #[inline(always)]
    fn sub(self, rhs: Number) -> Self::Output {
        PercentageUnit(self.0 - rhs)
    }
}

impl<Number: CssNumber> SubAssign<Number> for PercentageUnit<Number> {
    #[inline(always)]
    fn sub_assign(&mut self, rhs: Number) {
        *self = PercentageUnit(self.0 - rhs);
    }
}

impl<Number: CssNumber> Mul<Number> for PercentageUnit<Number> {
    type Output = Self;

    #[inline(always)]
    fn mul(self, rhs: Number) -> Self::Output {
        PercentageUnit(self.0 * rhs)
    }
}

impl<Number: CssNumber> MulAssign<Number> for PercentageUnit<Number> {
    #[inline(always)]
    fn mul_assign(&mut self, rhs: Number) {
        *self = PercentageUnit(self.0 * rhs);
    }
}

impl<Number: CssNumber> Div<Number> for PercentageUnit<Number> {
    type Output = PercentageUnit<Number>;

    #[inline(always)]
    fn div(self, rhs: Number) -> Self::Output {
        PercentageUnit(self.0 / rhs)
    }
}

impl<Number: CssNumber> DivAssign<Number> for PercentageUnit<Number> {
    #[inline(always)]
    fn div_assign(&mut self, rhs: Number) {
        *self = PercentageUnit(self.0 / rhs);
    }
}

impl<Number: CssNumber> Rem<Number> for PercentageUnit<Number> {
    type Output = PercentageUnit<Number>;

    #[inline(always)]
    fn rem(self, rhs: Number) -> Self::Output {
        PercentageUnit(self.0 % rhs)
    }
}

impl<Number: CssNumber> RemAssign<Number> for PercentageUnit<Number> {
    #[inline(always)]
    fn rem_assign(&mut self, rhs: Number) {
        *self = PercentageUnit(self.0 % rhs);
    }
}

impl<Number: CssNumber> Neg for PercentageUnit<Number> {
    type Output = PercentageUnit<Number>;

    #[inline(always)]
    fn neg(self) -> Self::Output {
        PercentageUnit(-self.0)
    }
}

impl<Number: CssNumber> CssNumberNewType<Number> for PercentageUnit<Number> {
    #[inline(always)]
    fn to_f32(&self) -> f32 {
        self.to_CssNumber().to_f32()
    }

    #[inline(always)]
    fn as_CssNumber(&self) -> &Number {
        self.0.as_CssNumber()
    }
}

impl<NumberX: CssNumber> Unit for PercentageUnit<NumberX> {
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
                if value == 0. {
                    return Ok(Constant(Self::default()));
                } else {
                    return CustomParseError::dimensionless(value);
                }
            }

            Token::Percentage { unit_value, .. } => {
                return Self::parse_percentage_outside_calc_function(unit_value)
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
    fn to_canonical_dimension_value<
        Conversion: FontRelativeLengthConversion<Self::Number>
            + ViewportPercentageLengthConversion<Self::Number>
            + PercentageConversion<Self::Number>,
    >(
        &self,
        conversion: &Conversion,
    ) -> Self::Number {
        self.to_absolute_value(conversion)
    }

    #[inline(always)]
    fn from_raw_css_for_var_expression_evaluation(
        value: &str,
        _is_not_in_page_rule: bool,
    ) -> Option<Self> {
        fn from_raw_css_for_var_expression_evaluation_internal<
            'i: 't,
            't,
            Number: CssNumber,
        >(
            input: &mut Parser<'i, 't>,
        ) -> Result<PercentageUnit<Number>, ParseError<'i, CustomParseError<'i>>>
        {
            let value = match *input.next()? {
                Token::Number { value, .. } => {
                    if value == 0. {
                        Ok(PercentageUnit::default())
                    } else {
                        CustomParseError::dimensionless(value)
                    }
                }

                Token::Percentage { unit_value, .. } => {
                    PercentageUnit::parse_percentage(unit_value)
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

impl<Number: CssNumber> PercentageUnit<Number> {
    pub const ZeroPercent: PercentageUnit<Number> =
        PercentageUnit(Number::Zero);

    pub const OneHundredPercent: PercentageUnit<Number> =
        PercentageUnit(Number::One);
}

impl<Number: CssNumber> PercentageUnit<Number> {
    #[inline(always)]
    pub fn to_absolute_value<Conversion: PercentageConversion<Number>>(
        &self,
        conversion: &Conversion,
    ) -> Number {
        self.to_CssNumber() * conversion.one_hundred_percent_in_absolute_units()
    }

    #[inline(always)]
    pub(crate) fn parse_percentage_outside_calc_function<'i>(
        unit_value: f32,
    ) -> Result<
        CalculablePropertyValue<Self>,
        ParseError<'i, CustomParseError<'i>>,
    > {
        Self::parse_percentage(unit_value).map(Constant)
    }

    #[inline(always)]
    pub(crate) fn parse_percentage<'i>(
        unit_value: f32,
    ) -> Result<Self, ParseError<'i, CustomParseError<'i>>> {
        let percentage =
            Number::new(unit_value).map_err(|cssNumberConversionError| {
                ParseError::from(CouldNotParseCssUnsignedNumber(
                    cssNumberConversionError,
                    unit_value,
                ))
            })?;
        Ok(PercentageUnit(percentage))
    }
}
