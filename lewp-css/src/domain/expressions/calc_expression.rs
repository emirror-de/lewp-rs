// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.

use {
    super::{CalculablePropertyValue, Expression},
    crate::{
        domain::units::{
            conversions::{
                AttributeConversion,
                CssVariableConversion,
                FontRelativeLengthConversion,
                PercentageConversion,
                ViewportPercentageLengthConversion,
            },
            Unit,
        },
        parsers::ParserContext,
        CustomParseError,
    },
    cssparser::{ParseError, Parser, ToCss},
    either::{Either, Right},
    std::fmt,
};

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum CalcExpression<U: Unit> {
    CalculablePropertyValue(CalculablePropertyValue<U>),

    Number(U::Number),

    Parentheses(Box<CalcExpression<U>>),

    Addition(Box<CalcExpression<U>>, Box<CalcExpression<U>>),

    Subtraction(Box<CalcExpression<U>>, Box<CalcExpression<U>>),

    Multiplication(Box<CalcExpression<U>>, Box<CalcExpression<U>>),

    Division(Box<CalcExpression<U>>, Box<CalcExpression<U>>),
}

impl<U: Unit> Default for CalcExpression<U> {
    #[inline(always)]
    fn default() -> Self {
        CalcExpression::CalculablePropertyValue(
            CalculablePropertyValue::default(),
        )
    }
}

impl<U: Unit> ToCss for CalcExpression<U> {
    fn to_css<W: fmt::Write>(&self, dest: &mut W) -> fmt::Result {
        use self::CalcExpression::*;

        match *self {
            CalculablePropertyValue(ref calculable) => calculable.to_css(dest),

            Number(ref number) => number.to_css(dest),

            Parentheses(ref calcFunctionBody) => {
                dest.write_char('(')?;
                calcFunctionBody.to_css(dest)?;
                dest.write_char(')')
            }

            Addition(ref lhs, ref rhs) => {
                lhs.to_css(dest)?;
                // Whitespace should not be needed if the lhs ends in a ')' or the rhs begins with '(' but the spec does not permit this (https://www.w3.org/TR/css3-values/#calc-notation)
                dest.write_str(" + ")?;
                rhs.to_css(dest)
            }

            Subtraction(ref lhs, ref rhs) => {
                lhs.to_css(dest)?;
                // Whitespace should not be needed if the lhs ends in a ')' or the rhs begins with '(' but the spec does not permit this (https://www.w3.org/TR/css3-values/#calc-notation)
                dest.write_str(" - ")?;
                rhs.to_css(dest)
            }

            Multiplication(ref lhs, ref rhs) => {
                lhs.to_css(dest)?;
                dest.write_char('*')?;
                rhs.to_css(dest)
            }

            Division(ref lhs, ref rhs) => {
                lhs.to_css(dest)?;
                dest.write_char('/')?;
                rhs.to_css(dest)
            }
        }
    }
}

impl<U: Unit> Expression<U> for CalcExpression<U> {
    /// Evaluate the calc() expression, returning the numeric value of the canonical dimension
    /// Division by zero is handled by returning the maximum possible f32 value
    /// Subtractions for UnsignedCssNumber that are negative are handled by returning 0.0
    /// Note: We are quite lenient with calculations of unit-less and unit-having quantities, eg 100px * 100px is evaluated to 10,000px, not 10,000px^2, and 50 + 100px is evaluated to 150px
    #[inline(always)]
    fn evaluate<
        Conversion: FontRelativeLengthConversion<U::Number>
            + ViewportPercentageLengthConversion<U::Number>
            + PercentageConversion<U::Number>
            + AttributeConversion<U>
            + CssVariableConversion,
    >(
        &self,
        conversion: &Conversion,
    ) -> Option<U::Number> {
        use self::CalcExpression::*;

        match *self {
            CalculablePropertyValue(ref calculable) => {
                calculable.evaluate(conversion)
            }

            Number(number) => Some(number),

            Parentheses(ref subExpression) => {
                subExpression.evaluate(conversion)
            }

            Addition(ref lhsSubExpression, ref rhsSubExpression) => match (
                lhsSubExpression.evaluate(conversion),
                rhsSubExpression.evaluate(conversion),
            ) {
                (Some(lhs), Some(rhs)) => Some(lhs + rhs),
                _ => None,
            },

            Subtraction(ref lhsSubExpression, ref rhsSubExpression) => match (
                lhsSubExpression.evaluate(conversion),
                rhsSubExpression.evaluate(conversion),
            ) {
                (Some(lhs), Some(rhs)) => Some(lhs - rhs),
                _ => None,
            },

            Multiplication(ref lhsSubExpression, ref rhsSubExpression) => {
                match (
                    lhsSubExpression.evaluate(conversion),
                    rhsSubExpression.evaluate(conversion),
                ) {
                    (Some(lhs), Some(rhs)) => Some(lhs * rhs),
                    _ => None,
                }
            }

            Division(ref lhsSubExpression, ref rhsSubExpression) => match (
                lhsSubExpression.evaluate(conversion),
                rhsSubExpression.evaluate(conversion),
            ) {
                (Some(lhs), Some(rhs)) => Some(lhs / rhs),
                _ => None,
            },
        }
    }
}

impl<U: Unit> CalcExpression<U> {
    /// Parse a top-level `calc` expression, with all nested sub-expressions.
    /// DOES NOT simplify expressions. This is because simplification is harder than it ought to be:-
    /// * Percentages can be treated as multiples of 'x', eg 50% => 0.5x, BUT
    /// * Zero percentages have to be preserved, so detecting 'divide by zero' at parse time isn't easy
    /// * Calc expressions additionally have other unknown quantities when dealing with units:-
    ///   * 4 kinds of font relative units
    ///   * 4 kinds of viewport proportion units
    ///   * Any number of `var()` and `attr()` sub-expressions (although the latter SHOULD have a known type)
    /// * It would be possible to simplify some terms, even then, but the coding cost of doing so (with 9 unknown quantities) seems a bit painful
    ///    * Absolute units could be converted to their canonical unit (eg pixels for AbsoluteLength)
    ///    * Single-term entries in parentheses and expressions could be eliminated
    ///    * nested expressions() could be converted to just ()
    ///
    /// This is in charge of parsing, for example, `2 + 3 * 100%`.
    #[inline(always)]
    pub(crate) fn parse<'i, 't>(
        context: &ParserContext,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i, CustomParseError<'i>>> {
        input.parse_nested_block(|input| Self::parse_sum(context, input))
    }

    /// Parse a `calc` expression, and all the sum that may follow, and stop as soon as a non-sum expression is found.
    ///
    /// This is in charge of parsing, for example, `2 + 3 * 100%`.
    fn parse_sum<'i, 't>(
        context: &ParserContext,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i, CustomParseError<'i>>> {
        use {self::CalcExpression::*, cssparser::Token::*};

        let mut currentSum = Self::parse_product(context, input)?;

        loop {
            let stateToResetParseToIfNotSum = input.state();
            match *input.next_including_whitespace()? {
                WhiteSpace(_) => {
                    // a trailing whitespace
                    if input.is_exhausted() {
                        break;
                    }
                }

                _ => {
                    input.reset(&stateToResetParseToIfNotSum);
                    break;
                }
            }

            let isAddition = match *input.next()? {
                Delim('+') => true,

                Delim('-') => false,

                ref unexpectedToken => {
                    return CustomParseError::unexpectedToken(unexpectedToken)
                }
            };

            currentSum = if isAddition {
                Addition(
                    Box::new(currentSum),
                    Box::new(Self::parse_product(context, input)?),
                )
            } else {
                Subtraction(
                    Box::new(currentSum),
                    Box::new(Self::parse_product(context, input)?),
                )
            }
        }

        Ok(currentSum)
    }

    /// Parse a `calc` expression, and all the products that may follow, and stop as soon as a non-product expression is found.
    ///
    /// This should parse correctly:-
    ///
    /// * `2`
    /// * `2 * 2`
    /// * `2 * 2 + 2` (will leave the `+ 2` unparsed).
    /// * `2 / 2 * 2 + 2` (will leave the `+ 2` unparsed).
    fn parse_product<'i, 't>(
        context: &ParserContext,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i, CustomParseError<'i>>> {
        use {self::CalcExpression::*, cssparser::Token::*};

        let mut currentProduct = Self::parse_one(context, input)?;

        loop {
            let stateToResetParseToIfNotProduct = input.state();
            match *input.next()? {
                Delim('*') => {
                    currentProduct = Multiplication(
                        Box::new(currentProduct),
                        Box::new(Self::parse_one(context, input)?),
                    );
                }

                Delim('/') => {
                    currentProduct = Division(
                        Box::new(currentProduct),
                        Box::new(Self::parse_one(context, input)?),
                    );
                }

                _ => {
                    input.reset(&stateToResetParseToIfNotProduct);
                    break;
                }
            }
        }

        Ok(currentProduct)
    }

    fn parse_one<'i, 't>(
        context: &ParserContext,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i, CustomParseError<'i>>> {
        let either = U::parse_one_inside_calc_function(context, input)?;
        if either.is_left() {
            Ok(CalcExpression::CalculablePropertyValue(
                either.left().unwrap(),
            ))
        } else {
            Ok(either.right().unwrap())
        }
    }

    #[inline(always)]
    pub(crate) fn parse_parentheses<'i, 't>(
        context: &ParserContext,
        input: &mut Parser<'i, 't>,
    ) -> Result<
        Either<CalculablePropertyValue<U>, CalcExpression<U>>,
        ParseError<'i, CustomParseError<'i>>,
    > {
        Ok(Right(CalcExpression::Parentheses(Box::new(
            CalcExpression::parse(context, input)?,
        ))))
    }
}
