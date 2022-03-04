// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.

use {
    super::{Expression, TypeOrUnit},
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
    cssparser::{serialize_identifier, ParseError, Parser, ToCss},
    std::fmt,
};

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct AttrExpression {
    pub attribute_lower_case_name: String,

    pub type_or_unit: TypeOrUnit,

    pub default_value_css: Option<String>,

    pub is_not_in_page_rule: bool,
}

impl ToCss for AttrExpression {
    fn to_css<W: fmt::Write>(&self, dest: &mut W) -> fmt::Result {
        serialize_identifier(&self.attribute_lower_case_name, dest)?;

        if self.type_or_unit != TypeOrUnit::string {
            dest.write_char(' ')?;
            self.type_or_unit.to_css(dest)?;
        }

        if let Some(ref default_value_css) = self.default_value_css {
            dest.write_char(',')?;
            dest.write_str(default_value_css)?;
        }

        Ok(())
    }
}

impl<U: Unit> Expression<U> for AttrExpression {
    /// Division by zero is handled by returning the maximum possible f32 value
    /// Subtractions for UnsignedCssNumber that are negative are handled by returning 0.0
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
        self.to_unit(conversion).map(|unit| unit.to_CssNumber())
    }
}

impl AttrExpression {
    pub fn to_unit<U: Unit, Conversion: AttributeConversion<U>>(
        &self,
        conversion: &Conversion,
    ) -> Option<U> {
        let (possibleValue, propertyDefaultOrIfNoPropertyDefaultTheUnitDefault) =
            conversion.attributeValue(&self.attribute_lower_case_name);
        if let Some(value) = possibleValue {
            if let Ok(ref value_css) = self.type_or_unit.value_to_css(value)
            {
                U::from_raw_css_for_var_expression_evaluation(
                    value_css,
                    self.is_not_in_page_rule,
                )
            } else if let Some(ref value_css) = self.default_value_css {
                U::from_raw_css_for_var_expression_evaluation(
                    value_css,
                    self.is_not_in_page_rule,
                )
            } else {
                Some(propertyDefaultOrIfNoPropertyDefaultTheUnitDefault)
            }
        } else if let Some(ref value_css) = self.default_value_css {
            U::from_raw_css_for_var_expression_evaluation(
                value_css,
                self.is_not_in_page_rule,
            )
        } else {
            Some(propertyDefaultOrIfNoPropertyDefaultTheUnitDefault)
        }
    }

    #[inline(always)]
    pub(crate) fn parse<'i, 't>(
        context: &ParserContext,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i, CustomParseError<'i>>> {
        input.parse_nested_block(|input| {
            let attribute_lower_case_name = {
                let attribute = input.expect_ident()?;
                attribute.to_ascii_lowercase()
            };

            input.skip_whitespace();

            if input.is_exhausted() {
                Ok(Self {
                    attribute_lower_case_name,
                    type_or_unit: TypeOrUnit::default(),
                    default_value_css: None,
                    is_not_in_page_rule: context.isNotInPageRule(),
                })
            } else {
                let type_or_unit = if let Ok(type_or_unit) = input.r#try(TypeOrUnit::parse)
                {
                    type_or_unit
                } else {
                    TypeOrUnit::default()
                };

                let result = input.r#try(|input| {
                    input.skip_whitespace();
                    input.expect_comma()?;
                    input.skip_whitespace();

                    let startPosition = input.position();
                    let result: Result<_, ParseError<CustomParseError>> = input
                        .parse_entirely(|input| {
                            Ok(input.slice_from(startPosition).to_owned())
                        });
                    result
                });

                let default_value_css = if let Ok(default_value_css) = result {
                    if default_value_css.is_empty() {
                        None
                    } else {
                        Some(default_value_css)
                    }
                } else {
                    None
                };

                Ok(Self {
                    attribute_lower_case_name,
                    type_or_unit,
                    default_value_css,
                    is_not_in_page_rule: context.isNotInPageRule(),
                })
            }
        })
    }
}
