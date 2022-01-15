// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.

use {
    crate::{domain::Atom, parsers::ParserContext, CustomParseError},
    cssparser::{Delimiter, ParseError, Parser, ToCss, TokenSerializationType},
    std::{borrow::Cow, collections::HashSet, fmt},
};

/// A specified value for a property is just a set of tokens.
///
/// The original CSS is preserved for serialization, as are variable references to other property names.
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct SpecifiedValue {
    pub originalCss: String,
    //first_token_type: TokenSerializationType,
    //last_token_type: TokenSerializationType,
    // References to property names in var() functions.
    //pub references: HashSet<Atom>,
}

impl ToCss for SpecifiedValue {
    fn to_css<W: fmt::Write>(&self, dest: &mut W) -> fmt::Result {
        dest.write_str(&self.originalCss)
    }
}

impl SpecifiedValue {
    /// Parse a custom property SpecifiedValue.
    pub(crate) fn parse<'i, 't>(
        _context: &ParserContext,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i, CustomParseError<'i>>> {
        let mut references = Some(HashSet::new());
        let (_first, css, _last) =
            Self::parse_self_contained_declaration_value(
                input,
                &mut references,
            )?;
        Ok(SpecifiedValue {
            originalCss: css.into_owned(),
            //first_token_type: first,
            //last_token_type: last,
            //references: references.unwrap(),
        })
    }

    fn parse_self_contained_declaration_value<'i, 't>(
        input: &mut Parser<'i, 't>,
        references: &mut Option<HashSet<Atom>>,
    ) -> Result<
        (TokenSerializationType, Cow<'i, str>, TokenSerializationType),
        ParseError<'i, CustomParseError<'i>>,
    > {
        let start_position = input.position();
        let mut missing_closing_characters = String::new();
        let (first, last) = Self::parse_declaration_value(
            input,
            references,
            &mut missing_closing_characters,
        )?;
        let mut css: Cow<str> = input.slice_from(start_position).into();
        if !missing_closing_characters.is_empty() {
            // Unescaped backslash at EOF in a quoted string is ignored.
            if css.ends_with('\\') {
                let first = missing_closing_characters.as_bytes()[0];
                if first == b'"' || first == b'\'' {
                    css.to_mut().pop();
                }
            }
            css.to_mut().push_str(&missing_closing_characters);
        }
        Ok((first, css, last))
    }

    /// <https://drafts.csswg.org/css-syntax-3/#typedef-declaration-value>
    fn parse_declaration_value<'i, 't>(
        input: &mut Parser<'i, 't>,
        references: &mut Option<HashSet<Atom>>,
        missing_closing_characters: &mut String,
    ) -> Result<
        (TokenSerializationType, TokenSerializationType),
        ParseError<'i, CustomParseError<'i>>,
    > {
        input.parse_until_before(
            Delimiter::Bang | Delimiter::Semicolon,
            |input| {
                // Need at least one token
                let start = input.state();
                input.next_including_whitespace()?;
                input.reset(&start);

                Self::parse_declaration_value_block(
                    input,
                    references,
                    missing_closing_characters,
                )
            },
        )
    }

    // Like parse_declaration_value, but accept `!` and `;` since they are only invalid at the top level
    fn parse_declaration_value_block<'i, 't>(
        input: &mut Parser<'i, 't>,
        references: &mut Option<HashSet<Atom>>,
        missing_closing_characters: &mut String,
    ) -> Result<
        (TokenSerializationType, TokenSerializationType),
        ParseError<'i, CustomParseError<'i>>,
    > {
        let mut token_start = input.position();
        let mut token = match input.next_including_whitespace_and_comments() {
            Ok(token) => token.clone(),
            Err(_) => {
                return Ok((
                    TokenSerializationType::nothing(),
                    TokenSerializationType::nothing(),
                ))
            }
        };

        let first_token_type = token.serialization_type();
        loop {
            macro_rules! nested {
                () => {
                    input.parse_nested_block(|input| {
                        Self::parse_declaration_value_block(
                            input,
                            references,
                            missing_closing_characters,
                        )
                    })?
                };
            }

            macro_rules! check_closed {
                ($closing: expr) => {
                    if !input.slice_from(token_start).ends_with($closing) {
                        missing_closing_characters.push_str($closing)
                    }
                };
            }

            use cssparser::Token::*;

            let last_token_type = match token {
                Comment(_) => {
                    let token_slice = input.slice_from(token_start);
                    if !token_slice.ends_with("*/") {
                        missing_closing_characters.push_str(if token_slice.ends_with('*') {
                            "/"
                        } else {
                            "*/"
                        })
                    }
                    token.serialization_type()
                }

                BadUrl(url) => {
                    return Err(ParseError::from(
                        CustomParseError::BadUrlInDeclarationValueBlock(url),
                    ))
                }

                BadString(string) => {
                    return Err(ParseError::from(
                        CustomParseError::BadStringInDeclarationValueBlock(string),
                    ))
                }

                CloseParenthesis => {
                    return Err(ParseError::from(
                        CustomParseError::UnbalancedCloseParenthesisInDeclarationValueBlock,
                    ))
                }

                CloseSquareBracket => {
                    return Err(ParseError::from(
                        CustomParseError::UnbalancedCloseSquareBracketInDeclarationValueBlock,
                    ))
                }

                CloseCurlyBracket => {
                    return Err(ParseError::from(
                        CustomParseError::UnbalancedCloseCurlyBracketInDeclarationValueBlock,
                    ))
                }

                Function(ref name) => {
                    if name.eq_ignore_ascii_case("var") {
                        let args_start = input.state();
                        input.parse_nested_block(|input| {
                            Self::parse_var_function(input, references)
                        })?;
                        input.reset(&args_start);
                    }
                    nested!();
                    check_closed!(")");
                    CloseParenthesis.serialization_type()
                }

                ParenthesisBlock => {
                    nested!();
                    check_closed!(")");
                    CloseParenthesis.serialization_type()
                }

                CurlyBracketBlock => {
                    nested!();
                    check_closed!("}");
                    CloseCurlyBracket.serialization_type()
                }

                SquareBracketBlock => {
                    nested!();
                    check_closed!("]");
                    CloseSquareBracket.serialization_type()
                }

                QuotedString(_) => {
                    let token_slice = input.slice_from(token_start);
                    let quote = &token_slice[..1];
                    if !(token_slice.ends_with(quote) && token_slice.len() > 1) {
                        missing_closing_characters.push_str(quote)
                    }
                    token.serialization_type()
                }

                Ident(ref value)
                | AtKeyword(ref value)
                | Hash(ref value)
                | IDHash(ref value)
                | UnquotedUrl(ref value)
                | Dimension {
                    unit: ref value, ..
                } => {
                    if value.ends_with('�') && input.slice_from(token_start).ends_with('\\') {
                        // Unescaped backslash at EOF in these contexts is interpreted as U+FFFD
                        // Check the value in case the final backslash was itself escaped.
                        // Serialize as escaped U+FFFD, which is also interpreted as U+FFFD.
                        // (Unescaped U+FFFD would also work, but removing the backslash is annoying.)
                        missing_closing_characters.push('�')
                    }

                    match token {
                        UnquotedUrl(_) => check_closed!(")"),
                        _ => {}
                    }

                    token.serialization_type()
                }
                _ => token.serialization_type(),
            };

            token_start = input.position();
            token = match input.next_including_whitespace_and_comments() {
                Ok(token) => token.clone(),
                Err(..) => return Ok((first_token_type, last_token_type)),
            };
        }
    }

    // If the var function is valid, return Ok((custom_property_name, fallback))
    fn parse_var_function<'i, 't>(
        input: &mut Parser<'i, 't>,
        references: &mut Option<HashSet<Atom>>,
    ) -> Result<(), ParseError<'i, CustomParseError<'i>>> {
        let name = input.expect_ident_cloned()?;
        if input.r#try(|input| input.expect_comma()).is_ok() {
            // Exclude `!` and `;` at the top level
            // https://drafts.csswg.org/css-syntax/#typedef-declaration-value
            input.parse_until_before(
                Delimiter::Bang | Delimiter::Semicolon,
                |input| {
                    // At least one non-comment token.
                    input.next_including_whitespace()?;

                    // Skip until the end.
                    while input.next_including_whitespace_and_comments().is_ok()
                    {
                    }

                    Ok(())
                },
            )?;
        }
        if let Some(ref mut refs) = *references {
            refs.insert(Atom::from(name));
        }
        Ok(())
    }
}
