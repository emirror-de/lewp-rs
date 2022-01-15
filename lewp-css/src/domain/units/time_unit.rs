// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.

use {
    super::{
        conversions::{
            FontRelativeLengthConversion,
            ViewportPercentageLengthConversion,
        },
        PercentageUnit,
        Unit,
    },
    crate::{
        domain::{
            expressions::{
                CalcExpression,
                CalculablePropertyValue::{self, Constant, Percentage},
                FunctionParser,
            },
            numbers::{CssNumber, CssNumberNewType},
        },
        parsers::ParserContext,
        serializers::serialize_dimension::serialize_dimension,
        CustomParseError::{self, *},
    },
    cssparser::{
        CowRcStr,
        ParseError,
        Parser,
        ParserInput,
        ToCss,
        Token::{self, *},
    },
    either::{Either, Left},
    std::{fmt, ops::*},
    TimeUnit::*,
};

/// A time: <https://www.w3.org/TR/css3-values/#time-value>
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum TimeUnit<Number: CssNumber> {
    /// A "s" value, seconds
    s(Number),

    /// A "ms" value, milliseconds
    ms(Number),
}

impl<Number: CssNumber> ToCss for TimeUnit<Number> {
    fn to_css<W: fmt::Write>(&self, dest: &mut W) -> fmt::Result {
        match *self {
            s(time) => serialize_dimension(time, "s", dest),
            ms(time) => serialize_dimension(time, "ms", dest),
        }
    }
}

impl<Number: CssNumber> Default for TimeUnit<Number> {
    #[inline(always)]
    fn default() -> Self {
        s(Number::default())
    }
}

impl<Number: CssNumber> Add<Number> for TimeUnit<Number> {
    type Output = TimeUnit<Number>;

    #[inline(always)]
    fn add(self, rhs: Number) -> Self::Output {
        match self {
            s(time) => s(time + rhs),
            ms(time) => ms(time + rhs),
        }
    }
}

impl<Number: CssNumber> AddAssign<Number> for TimeUnit<Number> {
    #[inline(always)]
    fn add_assign(&mut self, rhs: Number) {
        match *self {
            s(ref mut time) => *time = *time + rhs,
            ms(ref mut time) => *time = *time + rhs,
        }
    }
}

impl<Number: CssNumber> Sub<Number> for TimeUnit<Number> {
    type Output = TimeUnit<Number>;

    #[inline(always)]
    fn sub(self, rhs: Number) -> Self::Output {
        match self {
            s(time) => s(time - rhs),
            ms(time) => ms(time - rhs),
        }
    }
}

impl<Number: CssNumber> SubAssign<Number> for TimeUnit<Number> {
    #[inline(always)]
    fn sub_assign(&mut self, rhs: Number) {
        match *self {
            s(ref mut time) => *time = *time - rhs,
            ms(ref mut time) => *time = *time - rhs,
        }
    }
}

impl<Number: CssNumber> Mul<Number> for TimeUnit<Number> {
    type Output = TimeUnit<Number>;

    #[inline(always)]
    fn mul(self, rhs: Number) -> Self::Output {
        match self {
            s(time) => s(time * rhs),
            ms(time) => ms(time * rhs),
        }
    }
}

impl<Number: CssNumber> MulAssign<Number> for TimeUnit<Number> {
    #[inline(always)]
    fn mul_assign(&mut self, rhs: Number) {
        match *self {
            s(ref mut time) => *time = *time * rhs,
            ms(ref mut time) => *time = *time * rhs,
        }
    }
}

impl<Number: CssNumber> Div<Number> for TimeUnit<Number> {
    type Output = TimeUnit<Number>;

    #[inline(always)]
    fn div(self, rhs: Number) -> Self::Output {
        match self {
            s(time) => s(time / rhs),
            ms(time) => ms(time / rhs),
        }
    }
}

impl<Number: CssNumber> DivAssign<Number> for TimeUnit<Number> {
    #[inline(always)]
    fn div_assign(&mut self, rhs: Number) {
        match *self {
            s(ref mut time) => *time = *time / rhs,
            ms(ref mut time) => *time = *time / rhs,
        }
    }
}

impl<Number: CssNumber> Rem<Number> for TimeUnit<Number> {
    type Output = TimeUnit<Number>;

    #[inline(always)]
    fn rem(self, rhs: Number) -> Self::Output {
        match self {
            s(time) => s(time % rhs),
            ms(time) => ms(time % rhs),
        }
    }
}

impl<Number: CssNumber> RemAssign<Number> for TimeUnit<Number> {
    #[inline(always)]
    fn rem_assign(&mut self, rhs: Number) {
        match *self {
            s(ref mut time) => *time = *time % rhs,
            ms(ref mut time) => *time = *time % rhs,
        }
    }
}

impl<Number: CssNumber> Neg for TimeUnit<Number> {
    type Output = TimeUnit<Number>;

    #[inline(always)]
    fn neg(self) -> Self::Output {
        match self {
            s(time) => s(-time),
            ms(time) => ms(-time),
        }
    }
}

impl<Number: CssNumber> CssNumberNewType<Number> for TimeUnit<Number> {
    #[inline(always)]
    fn to_f32(&self) -> f32 {
        self.to_CssNumber().to_f32()
    }

    #[inline(always)]
    fn as_CssNumber(&self) -> &Number {
        match *self {
            s(ref time) => time,
            ms(ref time) => time,
        }
    }
}

impl<NumberX: CssNumber> Unit for TimeUnit<NumberX> {
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
            Number { value, .. } => {
                if value == 0. {
                    return Ok(Constant(Self::default()));
                } else {
                    return CustomParseError::dimensionless(value);
                }
            }

            Dimension {
                value, ref unit, ..
            } => return Self::parseDimension(value, unit).map(Constant),

            Function(ref name) => FunctionParser::parser(name)?,

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
                return Self::parseDimension(value, unit)
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
            s(seconds) => s(seconds),
            ms(milliseconds) => {
                ms(milliseconds / NumberX::_construct(1000_f32))
            }
        }
    }

    #[inline(always)]
    fn to_canonical_dimension_value<
        Conversion: FontRelativeLengthConversion<Self::Number>
            + ViewportPercentageLengthConversion<Self::Number>,
    >(
        &self,
        _conversion: &Conversion,
    ) -> Self::Number {
        match *self {
            s(seconds) => seconds,
            ms(milliseconds) => milliseconds / NumberX::_construct(1000_f32),
        }
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
        ) -> Result<TimeUnit<Number>, ParseError<'i, CustomParseError<'i>>>
        {
            let value = match *input.next()? {
                Token::Number { value, .. } => {
                    if value == 0. {
                        Ok(TimeUnit::default())
                    } else {
                        CustomParseError::dimensionless(value)
                    }
                }

                Token::Dimension {
                    value, ref unit, ..
                } => TimeUnit::parseDimension(value, unit),

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

impl<Number: CssNumber> TimeUnit<Number> {
    #[inline(always)]
    fn parseDimension<'i>(
        value: f32,
        unit: &CowRcStr<'i>,
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

            "s" => Ok(s(cssNumber)),

            "ms" => Ok(ms(cssNumber)),

            _ => return Err(ParseError::from(CouldNotParseDimension(value, unit.clone()))),
        }
    }
}
