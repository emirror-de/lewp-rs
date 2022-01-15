// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.

use {
    super::{CssNumber, CssNumberNewType},
    crate::{
        domain::{
            expressions::{
                CalcExpression,
                CalculablePropertyValue::{self, *},
                FunctionParser,
            },
            units::{
                conversions::*,
                AppUnitsPer,
                PercentageUnit,
                Unit,
                UnitFromStrError,
            },
        },
        parsers::ParserContext,
        CustomParseError,
    },
    cssparser::{ParseError, Parser, ParserInput, ToCss, Token},
    either::{Either, Left},
    std::{
        cmp::Ordering,
        fmt::{self, Display, Formatter, LowerExp, UpperExp},
        hash::{Hash, Hasher},
        ops::*,
        str::FromStr,
    },
};

/// A CSS float value similar to f32 but with a more restricted range
#[derive(Debug, Copy, Clone)]
pub struct CssSignedNumber(f32);

impl PartialEq for CssSignedNumber {
    #[inline(always)]
    fn eq(&self, other: &Self) -> bool {
        self.to_f32().eq(&other.0)
    }
}

impl Eq for CssSignedNumber {}

impl PartialOrd for CssSignedNumber {
    #[inline(always)]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.to_f32().partial_cmp(&other.0)
    }
}

impl Ord for CssSignedNumber {
    #[inline(always)]
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap_or(Ordering::Equal)
    }
}

impl Hash for CssSignedNumber {
    #[inline(always)]
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.to_bits().hash(state)
    }
}

impl ToCss for CssSignedNumber {
    #[inline(always)]
    fn to_css<W: fmt::Write>(&self, dest: &mut W) -> fmt::Result {
        self.to_f32().to_css(dest)
    }
}

impl Display for CssSignedNumber {
    #[inline(always)]
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        <f32 as Display>::fmt(&self.to_f32(), fmt)
    }
}

impl LowerExp for CssSignedNumber {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        <f32 as LowerExp>::fmt(&self.to_f32(), f)
    }
}

impl UpperExp for CssSignedNumber {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        <f32 as UpperExp>::fmt(&self.to_f32(), f)
    }
}

impl Default for CssSignedNumber {
    #[inline(always)]
    fn default() -> Self {
        Self::Zero
    }
}

impl Add<CssSignedNumber> for CssSignedNumber {
    type Output = Self;

    #[inline(always)]
    fn add(self, rhs: CssSignedNumber) -> Self::Output {
        <Self as CssNumber>::clamp(self.to_f32() + rhs.0)
    }
}

impl AddAssign<CssSignedNumber> for CssSignedNumber {
    #[inline(always)]
    fn add_assign(&mut self, rhs: CssSignedNumber) {
        *self = self.add(rhs)
    }
}

impl Sub<CssSignedNumber> for CssSignedNumber {
    type Output = Self;

    #[inline(always)]
    fn sub(self, rhs: CssSignedNumber) -> Self::Output {
        <Self as CssNumber>::clamp(self.to_f32() - rhs.0)
    }
}

impl SubAssign<CssSignedNumber> for CssSignedNumber {
    #[inline(always)]
    fn sub_assign(&mut self, rhs: CssSignedNumber) {
        *self = self.sub(rhs)
    }
}

impl Mul<CssSignedNumber> for CssSignedNumber {
    type Output = Self;

    #[inline(always)]
    fn mul(self, rhs: CssSignedNumber) -> Self::Output {
        <Self as CssNumber>::clamp(self.to_f32() * rhs.0)
    }
}

impl MulAssign<CssSignedNumber> for CssSignedNumber {
    #[inline(always)]
    fn mul_assign(&mut self, rhs: CssSignedNumber) {
        *self = self.mul(rhs)
    }
}

impl Div<CssSignedNumber> for CssSignedNumber {
    type Output = Self;

    #[inline(always)]
    fn div(self, rhs: CssSignedNumber) -> Self::Output {
        if rhs.0.is_nan() {
            let value = if (self.to_f32() / rhs.0).is_sign_positive() {
                ::std::f32::MAX
            } else {
                ::std::f32::MIN
            };
            CssSignedNumber(value)
        } else {
            <Self as CssNumber>::clamp(self.to_f32() / rhs.0)
        }
    }
}

impl DivAssign<CssSignedNumber> for CssSignedNumber {
    #[inline(always)]
    fn div_assign(&mut self, rhs: CssSignedNumber) {
        *self = self.div(rhs)
    }
}

impl Rem<CssSignedNumber> for CssSignedNumber {
    type Output = Self;

    #[inline(always)]
    fn rem(self, rhs: CssSignedNumber) -> Self::Output {
        if rhs.0.is_nan() {
            let value = if (self.to_f32() % rhs.0).is_sign_positive() {
                ::std::f32::MAX
            } else {
                ::std::f32::MIN
            };
            CssSignedNumber(value)
        } else {
            <Self as CssNumber>::clamp(self.to_f32() % rhs.0)
        }
    }
}

impl RemAssign<CssSignedNumber> for CssSignedNumber {
    #[inline(always)]
    fn rem_assign(&mut self, rhs: CssSignedNumber) {
        *self = self.rem(rhs)
    }
}

impl Neg for CssSignedNumber {
    type Output = Self;

    #[inline(always)]
    fn neg(self) -> Self::Output {
        if self.is_zero() {
            self
        } else {
            CssSignedNumber(-self.to_f32())
        }
    }
}

impl CssNumberNewType<Self> for CssSignedNumber {
    #[inline(always)]
    fn to_f32(&self) -> f32 {
        self.0
    }

    #[inline(always)]
    fn as_CssNumber(&self) -> &CssSignedNumber {
        self
    }
}

impl Deref for CssSignedNumber {
    type Target = f32;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<u16> for CssSignedNumber {
    #[inline(always)]
    fn from(small: u16) -> CssSignedNumber {
        CssSignedNumber(small as f32)
    }
}

impl From<i16> for CssSignedNumber {
    #[inline(always)]
    fn from(small: i16) -> CssSignedNumber {
        CssSignedNumber(small as f32)
    }
}

impl From<u8> for CssSignedNumber {
    #[inline(always)]
    fn from(small: u8) -> CssSignedNumber {
        CssSignedNumber(small as f32)
    }
}

impl From<i8> for CssSignedNumber {
    #[inline(always)]
    fn from(small: i8) -> CssSignedNumber {
        CssSignedNumber(small as f32)
    }
}

impl FromStr for CssSignedNumber {
    type Err = UnitFromStrError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value = f32::from_str(s)?;
        Ok(CssSignedNumber::new(value)?)
    }
}

impl CssNumber for CssSignedNumber {
    const Zero: Self = CssSignedNumber(0.0);

    const One: Self = CssSignedNumber(1.0);

    const Maximum: Self = CssSignedNumber(::std::f32::MAX);

    const Minimum: Self = CssSignedNumber(::std::f32::MIN);

    const DotsPerInch: Self = CssSignedNumber(96.0);

    const CentimetresPerInch: Self = CssSignedNumber(2.54);

    #[inline(always)]
    fn as_f32(&self) -> f32 {
        self.0
    }

    #[inline(always)]
    fn as_u32(&self) -> u32 {
        self.0 as u32
    }

    #[doc(hidden)]
    #[inline(always)]
    fn _construct(value: f32) -> Self {
        CssSignedNumber(value)
    }

    #[inline(always)]
    fn parseNumber<'i>(
        value: f32,
        _int_value: Option<i32>,
    ) -> Result<Self, ParseError<'i, CustomParseError<'i>>> {
        CssSignedNumber::new(value).map_err(|cssNumberConversionError| {
            ParseError::from(CustomParseError::CouldNotParseCssSignedNumber(
                cssNumberConversionError,
                value,
            ))
        })
    }
}

impl AppUnitsPer for CssSignedNumber {
    /// Number of app units per pixel
    const AppUnitsPerPX: Self = CssSignedNumber(f32::AppUnitsPerPX);

    /// Number of app units per inch
    const AppUnitsPerIN: Self = CssSignedNumber(f32::AppUnitsPerIN);

    /// Number of app units per centimeter
    const AppUnitsPerCM: Self = CssSignedNumber(f32::AppUnitsPerCM);

    /// Number of app units per millimeter
    const AppUnitsPerMM: Self = CssSignedNumber(f32::AppUnitsPerMM);

    /// Number of app units per quarter
    const AppUnitsPerQ: Self = CssSignedNumber(f32::AppUnitsPerQ);

    /// Number of app units per point
    const AppUnitsPerPT: Self = CssSignedNumber(f32::AppUnitsPerPT);

    /// Number of app units per pica
    const AppUnitsPerPC: Self = CssSignedNumber(f32::AppUnitsPerPC);
}

impl Unit for CssSignedNumber {
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
                value, int_value, ..
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
        ) -> Result<CssSignedNumber, ParseError<'i, CustomParseError<'i>>>
        {
            let value = match *input.next()? {
                Token::Number {
                    value, int_value, ..
                } => CssSignedNumber::parseNumber(value, int_value),

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
