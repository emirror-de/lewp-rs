// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.

use {
    super::{
        consume_any_value::consume_any_value,
        SupportsCondition::*,
        SupportsPropertyDeclaration,
    },
    crate::CustomParseError,
    cssparser::{
        BasicParseError,
        BasicParseErrorKind,
        ParseError,
        Parser,
        ToCss,
        Token::{Function, Ident, ParenthesisBlock},
    },
    std::fmt,
};

/// An @supports condition
///
/// <https://drafts.csswg.org/css-conditional-3/#at-supports>
#[derive(Debug, Clone)]
pub enum SupportsCondition {
    /// `not (condition)`
    Not(Box<SupportsCondition>),

    /// `(condition)`
    Parenthesized(Box<SupportsCondition>),

    /// `(condition) and (condition) and (condition) ..`
    And(Vec<SupportsCondition>),

    /// `(condition) or (condition) or (condition) ..`
    Or(Vec<SupportsCondition>),

    /// `property-ident: value` (value can be any tokens)
    Declaration(SupportsPropertyDeclaration),

    /// `(any tokens)` or `func(any tokens)`
    FutureSyntax(String),
}

impl ToCss for SupportsCondition {
    fn to_css<W: fmt::Write>(&self, dest: &mut W) -> fmt::Result {
        match *self {
            Not(ref condition) => {
                dest.write_str("not ")?;
                condition.to_css(dest)
            }

            Parenthesized(ref condition) => {
                dest.write_str("(")?;
                condition.to_css(dest)?;
                dest.write_str(")")
            }

            And(ref conditions) => {
                let mut first = true;
                for condition in conditions {
                    if first {
                        first = false;
                    } else {
                        dest.write_str(" and ")?;
                    }
                    condition.to_css(dest)?;
                }
                Ok(())
            }

            Or(ref conditions) => {
                let mut first = true;
                for condition in conditions {
                    if first {
                        first = false;
                    } else {
                        dest.write_str(" or ")?;
                    }
                    condition.to_css(dest)?;
                }
                Ok(())
            }

            Declaration(ref declaration) => {
                dest.write_str("(")?;
                declaration.to_css(dest)?;
                dest.write_str(")")
            }

            FutureSyntax(ref value) => dest.write_str(value),
        }
    }
}

impl SupportsCondition {
    /// Parse a condition
    ///
    /// <https://drafts.csswg.org/css-conditional/#supports_condition>
    pub(crate) fn parse<'i, 't>(
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i, CustomParseError<'i>>> {
        if input.r#try(|i| i.expect_ident_matching("not")).is_ok() {
            let inner = Self::parse_in_parentheses(input)?;
            return Ok(Not(Box::new(inner)));
        }

        let in_parentheses = Self::parse_in_parentheses(input)?;

        let (keyword, wrapper) = match input.next() {
            // End of input
            Err(_) => return Ok(in_parentheses),

            Ok(&Ident(ref ident)) => {
                match_ignore_ascii_case! {
                    ident,
                    "and" => ("and", And as fn(_) -> _),
                    "or" => ("or", Or as fn(_) -> _),
                    _ => return Err(ParseError::from(CustomParseError::InvalidSupportsCondition(ident.clone())))
                }
            }

            Ok(unexpectedToken) => {
                return Err(ParseError::from(BasicParseError {
                    kind: BasicParseErrorKind::UnexpectedToken(
                        unexpectedToken.clone(),
                    ),
                    location: input.state().source_location(),
                }))
            }
        };

        let mut conditions = Vec::with_capacity(2);
        conditions.push(in_parentheses);
        loop {
            conditions.push(Self::parse_in_parentheses(input)?);
            if input
                .r#try(|input| input.expect_ident_matching(keyword))
                .is_err()
            {
                // Did not find the expected keyword.
                // If we found some other token,
                // it will be rejected by `Parser::parse_entirely` somewhere up the stack.
                return Ok(wrapper(conditions));
            }
        }
    }

    /// <https://drafts.csswg.org/css-conditional-3/#supports_condition_in_parentheses>
    fn parse_in_parentheses<'i, 't>(
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i, CustomParseError<'i>>> {
        // Whitespace is normally taken care of in `Parser::next` but we want to not include it in `pos` for the SupportsCondition::FutureSyntax cases.
        while input.r#try(Parser::expect_whitespace).is_ok() {}

        let pos = input.position();
        let token = input.next()?;

        match token {
            ParenthesisBlock => {
                let nested = input.r#try(|input| {
                    input.parse_nested_block(|i| {
                        Self::parse_condition_or_declaration(i)
                    })
                });

                if nested.is_ok() {
                    return nested;
                }
            }

            Function(_) => {}
            _ => {
                return Err(ParseError::from(BasicParseError {
                    kind: BasicParseErrorKind::UnexpectedToken(token.clone()),
                    location: input.state().source_location(),
                }))
            }
        }
        input.parse_nested_block(consume_any_value)?;
        Ok(FutureSyntax(input.slice_from(pos).to_owned()))
    }

    /// supports_condition | declaration
    /// <https://drafts.csswg.org/css-conditional/#dom-css-supports-conditiontext-conditiontext>
    fn parse_condition_or_declaration<'i, 't>(
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i, CustomParseError<'i>>> {
        if let Ok(condition) = input.r#try(Self::parse) {
            Ok(Parenthesized(Box::new(condition)))
        } else {
            SupportsPropertyDeclaration::parse(input).map(Declaration)
        }
    }
}
