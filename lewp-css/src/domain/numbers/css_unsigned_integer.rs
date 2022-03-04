// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.

use {
    super::{CssNumber, CssNumberConversionError},
    crate::{
        domain::{
            expressions::{
                CalcExpression,
                CalculablePropertyValue::{self, *},
                FunctionParser,
            },
            numbers::CssNumberNewType,
            units::{conversions::*, AppUnitsPer, PercentageUnit, Unit},
        },
        parsers::ParserContext,
        CustomParseError,
    },
    cssparser::{ParseError, Parser, ParserInput, ToCss, Token},
    either::{Either, Left},
    std::{
        fmt,
        fmt::{Display, Formatter},
        num::ParseIntError,
        ops::{Deref, *},
        str::FromStr,
    },
};

/// A CSS integer value similar to u32
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct CssUnsignedInteger(u32);

impl ToCss for CssUnsignedInteger {
    #[inline(always)]
    fn to_css<W: fmt::Write>(&self, dest: &mut W) -> fmt::Result {
        self.0.to_css(dest)
    }
}

impl Display for CssUnsignedInteger {
    #[inline(always)]
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        <u32 as Display>::fmt(&self.0, fmt)
    }
}

impl Default for CssUnsignedInteger {
    #[inline(always)]
    fn default() -> Self {
        Self::Zero
    }
}

impl Add<CssUnsignedInteger> for CssUnsignedInteger {
    type Output = Self;

    #[inline(always)]
    fn add(self, rhs: CssUnsignedInteger) -> Self::Output {
        CssUnsignedInteger(self.0 + rhs.0)
    }
}

impl AddAssign<CssUnsignedInteger> for CssUnsignedInteger {
    #[inline(always)]
    fn add_assign(&mut self, rhs: CssUnsignedInteger) {
        *self = self.add(rhs)
    }
}

impl Sub<CssUnsignedInteger> for CssUnsignedInteger {
    type Output = Self;

    #[inline(always)]
    fn sub(self, rhs: CssUnsignedInteger) -> Self::Output {
        CssUnsignedInteger(self.0 - rhs.0)
    }
}

impl SubAssign<CssUnsignedInteger> for CssUnsignedInteger {
    #[inline(always)]
    fn sub_assign(&mut self, rhs: CssUnsignedInteger) {
        *self = self.sub(rhs)
    }
}

impl Mul<CssUnsignedInteger> for CssUnsignedInteger {
    type Output = Self;

    #[inline(always)]
    fn mul(self, rhs: CssUnsignedInteger) -> Self::Output {
        CssUnsignedInteger(self.0 * rhs.0)
    }
}

impl MulAssign<CssUnsignedInteger> for CssUnsignedInteger {
    #[inline(always)]
    fn mul_assign(&mut self, rhs: CssUnsignedInteger) {
        *self = self.mul(rhs)
    }
}

impl Div<CssUnsignedInteger> for CssUnsignedInteger {
    type Output = Self;

    #[inline(always)]
    fn div(self, rhs: CssUnsignedInteger) -> Self::Output {
        if rhs.0 == 0 {
            CssUnsignedInteger(::std::u32::MAX)
        } else {
            CssUnsignedInteger(self.0 / rhs.0)
        }
    }
}

impl DivAssign<CssUnsignedInteger> for CssUnsignedInteger {
    #[inline(always)]
    fn div_assign(&mut self, rhs: CssUnsignedInteger) {
        *self = self.div(rhs)
    }
}

impl Rem<CssUnsignedInteger> for CssUnsignedInteger {
    type Output = Self;

    #[inline(always)]
    fn rem(self, rhs: CssUnsignedInteger) -> Self::Output {
        CssUnsignedInteger(self.0 % rhs.0)
    }
}

impl RemAssign<CssUnsignedInteger> for CssUnsignedInteger {
    #[inline(always)]
    fn rem_assign(&mut self, rhs: CssUnsignedInteger) {
        *self = self.rem(rhs)
    }
}

impl Neg for CssUnsignedInteger {
    type Output = Self;

    #[inline(always)]
    fn neg(self) -> Self::Output {
        if self.is_zero() {
            self
        } else {
            CssUnsignedInteger(0)
        }
    }
}

impl CssNumberNewType<Self> for CssUnsignedInteger {
    #[inline(always)]
    fn to_f32(&self) -> f32 {
        self.0 as f32
    }

    #[inline(always)]
    fn as_CssNumber(&self) -> &CssUnsignedInteger {
        self
    }

    #[inline(always)]
    fn is_zero(&self) -> bool {
        self.0 == 0
    }

    #[inline(always)]
    fn is_positive(&self) -> bool {
        self.0 != 0
    }

    #[inline(always)]
    fn is_negative(&self) -> bool {
        false
    }

    #[inline(always)]
    fn is_zero_or_positive(&self) -> bool {
        true
    }

    #[inline(always)]
    fn is_zero_or_negative(&self) -> bool {
        self.is_zero()
    }
}

impl Deref for CssUnsignedInteger {
    type Target = u32;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<u32> for CssUnsignedInteger {
    #[inline(always)]
    fn from(small: u32) -> CssUnsignedInteger {
        CssUnsignedInteger(small)
    }
}

impl From<u16> for CssUnsignedInteger {
    #[inline(always)]
    fn from(small: u16) -> CssUnsignedInteger {
        CssUnsignedInteger(small as u32)
    }
}

impl From<u8> for CssUnsignedInteger {
    #[inline(always)]
    fn from(small: u8) -> CssUnsignedInteger {
        CssUnsignedInteger(small as u32)
    }
}

impl FromStr for CssUnsignedInteger {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value = u32::from_str(s)?;
        Ok(CssUnsignedInteger(value))
    }
}

impl CssNumber for CssUnsignedInteger {
    const Zero: Self = CssUnsignedInteger(0);

    const One: Self = CssUnsignedInteger(1);

    const Maximum: Self = CssUnsignedInteger(::std::u32::MAX);

    const Minimum: Self = CssUnsignedInteger(::std::u32::MIN);

    const DotsPerInch: Self = CssUnsignedInteger(96);

    const CentimetresPerInch: Self = CssUnsignedInteger(2);

    #[inline(always)]
    fn as_f32(&self) -> f32 {
        self.0 as f32
    }

    #[inline(always)]
    fn as_u32(&self) -> u32 {
        self.0
    }

    #[inline(always)]
    fn round(self) -> Self {
        self
    }

    #[inline(always)]
    fn abs(self) -> Self {
        self
    }

    #[doc(hidden)]
    #[inline(always)]
    fn _construct(value: f32) -> Self {
        CssUnsignedInteger(value as u32)
    }

    #[inline(always)]
    fn new(value: f32) -> Result<Self, CssNumberConversionError> {
        if value.is_sign_negative() {
            Err(CssNumberConversionError::NegativeNumberMayNotBeAllowed)
        } else if value == 0.0 {
            Ok(CssUnsignedInteger(0))
        } else {
            let cast = value as u32;
            if f32::from_bits(cast) == value {
                Ok(CssUnsignedInteger(0))
            } else {
                Err(CssNumberConversionError::FloatingPointNumberMayNotBeAllowed)
            }
        }
    }

    #[inline(always)]
    fn parseNumber<'i>(
        value: f32,
        int_value: Option<i32>,
    ) -> Result<Self, ParseError<'i, CustomParseError<'i>>> {
        if let Some(constant) = int_value {
            if constant >= 0 {
                Ok(CssUnsignedInteger(constant as u32))
            } else {
                Err(ParseError::from(
                    CustomParseError::UnsignedIntegersCanNotBeNegative(
                        constant,
                    ),
                ))
            }
        } else {
            Err(ParseError::from(
                CustomParseError::UnsignedIntegersCanNotBeFloats(value),
            ))
        }
    }
}

impl AppUnitsPer for CssUnsignedInteger {
    /// Number of app units per pixel
    const AppUnitsPerPX: Self = CssUnsignedInteger(f32::AppUnitsPerPX as u32);

    /// Number of app units per inch
    const AppUnitsPerIN: Self = CssUnsignedInteger(f32::AppUnitsPerIN as u32);

    /// Number of app units per centimeter
    const AppUnitsPerCM: Self = CssUnsignedInteger(f32::AppUnitsPerCM as u32);

    /// Number of app units per millimeter
    const AppUnitsPerMM: Self = CssUnsignedInteger(f32::AppUnitsPerMM as u32);

    /// Number of app units per quarter
    const AppUnitsPerQ: Self = CssUnsignedInteger(f32::AppUnitsPerQ as u32);

    /// Number of app units per point
    const AppUnitsPerPT: Self = CssUnsignedInteger(f32::AppUnitsPerPT as u32);

    /// Number of app units per pica
    const AppUnitsPerPC: Self = CssUnsignedInteger(f32::AppUnitsPerPC as u32);
}

impl Unit for CssUnsignedInteger {
    type Number = Self;

    const HasDimension: bool = false;

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
                int_value, value, ..
            } => return Self::parseNumber(value, int_value).map(Constant),

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
                return Self::parseNumber(value, int_value)
                    .map(|value| Left(Constant(value)))
            }

            Token::Percentage { unit_value, .. } => {
                return PercentageUnit::parse_percentage(unit_value)
                    .map(|value| Left(Percentage(value)))
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
        _conversion: &Conversion,
    ) -> Self::Number {
        self.to_CssNumber()
    }

    #[inline(always)]
    fn from_raw_css_for_var_expression_evaluation(
        value: &str,
        _is_not_in_page_rule: bool,
    ) -> Option<Self> {
        fn from_raw_css_for_var_expression_evaluation_internal<'i: 't, 't>(
            input: &mut Parser<'i, 't>,
        ) -> Result<CssUnsignedInteger, ParseError<'i, CustomParseError<'i>>>
        {
            let value = match *input.next()? {
                Token::Number {
                    value, int_value, ..
                } => CssUnsignedInteger::parseNumber(value, int_value),

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
