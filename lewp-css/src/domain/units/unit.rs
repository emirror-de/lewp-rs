// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.

use {
    super::conversions::{
        FontRelativeLengthConversion,
        PercentageConversion,
        ViewportPercentageLengthConversion,
    },
    crate::{
        domain::{
            expressions::{CalcExpression, CalculablePropertyValue},
            numbers::{CssNumber, CssNumberNewType},
        },
        parsers::ParserContext,
        CustomParseError::{self, *},
    },
    cssparser::{ParseError, Parser, ToCss},
    either::{Either, Right},
};

pub trait Unit:
    Sized + ToCss + Default + CssNumberNewType<<Self as Unit>::Number>
{
    type Number: CssNumber;

    const HasDimension: bool;

    fn parse_one_outside_calc_function<'i, 't>(
        context: &ParserContext,
        input: &mut Parser<'i, 't>,
    ) -> Result<
        CalculablePropertyValue<Self>,
        ParseError<'i, CustomParseError<'i>>,
    >;

    fn parse_one_inside_calc_function<'i, 't>(
        context: &ParserContext,
        input: &mut Parser<'i, 't>,
    ) -> Result<
        Either<CalculablePropertyValue<Self>, CalcExpression<Self>>,
        ParseError<'i, CustomParseError<'i>>,
    >;

    #[inline(always)]
    fn to_canonical_dimension(self) -> Self {
        self
    }

    fn to_canonical_dimension_value<
        Conversion: FontRelativeLengthConversion<Self::Number>
            + ViewportPercentageLengthConversion<Self::Number>
            + PercentageConversion<Self::Number>,
    >(
        &self,
        conversion: &Conversion,
    ) -> Self::Number;

    fn from_raw_css_for_var_expression_evaluation(
        value: &str,
        is_not_in_page_rule: bool,
    ) -> Option<Self>;

    #[inline(always)]
    fn number_inside_calc_function<'i>(
        value: f32,
    ) -> Result<
        Either<CalculablePropertyValue<Self>, CalcExpression<Self>>,
        ParseError<'i, CustomParseError<'i>>,
    > {
        let constant =
            Self::Number::new(value).map_err(|cssNumberConversionError| {
                ParseError::from(CouldNotParseCssUnsignedNumber(
                    cssNumberConversionError,
                    value,
                ))
            })?;
        Ok(Right(CalcExpression::Number(constant)))
    }
}
