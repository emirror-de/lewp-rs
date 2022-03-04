// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.

use {
    super::{HasImportance, PropertyDeclaration},
    crate::{
        domain::HasPropertyDeclarations,
        parsers::{
            property_declaration_parser::PropertyDeclarationParser,
            ParserContext,
        },
        CustomParseError,
    },
    cssparser::{DeclarationListParser, ParseError, Parser, ToCss},
    std::{fmt, marker::PhantomData},
};

/// A list of property declarations
#[derive(Default, Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct PropertyDeclarations<I: HasImportance>(
    pub Vec<PropertyDeclaration<I>>,
);

impl<I: HasImportance> ToCss for PropertyDeclarations<I> {
    fn to_css<W: fmt::Write>(&self, dest: &mut W) -> fmt::Result {
        let length = self.0.len();
        if length != 0 {
            for index in 0..(length - 1) {
                (unsafe { self.0.get_unchecked(index) }).to_css(dest)?;
            }

            (unsafe { self.0.get_unchecked(length - 1) })
                .to_css_without_trailing_semicolon(dest)
        } else {
            Ok(())
        }
    }
}

impl<I: HasImportance> HasPropertyDeclarations<I> for PropertyDeclarations<I> {
    #[inline(always)]
    fn property_declarations(&self) -> &PropertyDeclarations<I> {
        self
    }

    #[inline(always)]
    fn property_declarations_mut(&mut self) -> &mut PropertyDeclarations<I> {
        self
    }

    #[inline(always)]
    fn property_declarations_slice(&self) -> &[PropertyDeclaration<I>] {
        &self.0[..]
    }

    #[inline(always)]
    fn property_declarations_vec(&self) -> &Vec<PropertyDeclaration<I>> {
        &self.0
    }

    #[inline(always)]
    fn property_declarations_vec_mut(
        &mut self,
    ) -> &mut Vec<PropertyDeclaration<I>> {
        &mut self.0
    }
}

impl<I: HasImportance> PropertyDeclarations<I> {
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    // Parse a list of property declarations and return a property declaration block.
    pub(crate) fn parse_property_declaration_list<'i: 't, 't>(
        context: &ParserContext,
        input: &mut Parser<'i, 't>,
    ) -> Result<PropertyDeclarations<I>, ParseError<'i, CustomParseError<'i>>>
    {
        let mut propertyDeclarations = Vec::new();
        let parsedPropertyDeclarations = DeclarationListParser::new(
            input,
            PropertyDeclarationParser {
                context,
                marker: PhantomData,
            },
        );

        for propertyDeclaration in parsedPropertyDeclarations
        {
            match propertyDeclaration {
                Ok(propertyDeclaration) => {
                    propertyDeclarations.push(propertyDeclaration)
                }
                Err(preciseParseError) => return Err(preciseParseError.0),
            }
        }

        Ok(PropertyDeclarations(propertyDeclarations))
    }
}
