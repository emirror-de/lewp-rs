// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.

use {
    crate::{
        domain::{
            properties::{
                CssWideKeyword,
                HasImportance,
                Importance,
                PropertyDeclaration,
                SpecifiedValue,
                UnparsedPropertyValue,
            },
            Atom,
            VendorPrefix,
        },
        parsers::ParserContext,
        CustomParseError,
    },
    cssparser::{
        AtRuleParser,
        CowRcStr,
        DeclarationParser,
        Delimiter,
        ParseError,
        Parser,
    },
    std::marker::PhantomData,
};

/// A struct to parse property declarations.
pub(crate) struct PropertyDeclarationParser<'a, I: 'a + HasImportance> {
    pub(crate) context: &'a ParserContext,
    pub(crate) marker: PhantomData<&'a I>,
}

/// In theory, @rules may be present. In practice, none are currently defined (Sep 2017)
impl<'a, 'i, I: HasImportance> AtRuleParser<'i>
    for PropertyDeclarationParser<'a, I>
{
    type Prelude = ();

    type AtRule = PropertyDeclaration<I>;

    type Error = CustomParseError<'i>;
}

impl<'a, 'i, I: HasImportance> DeclarationParser<'i>
    for PropertyDeclarationParser<'a, I>
{
    type Declaration = PropertyDeclaration<I>;

    type Error = CustomParseError<'i>;

    fn parse_value<'t>(
        &mut self,
        name: CowRcStr<'i>,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self::Declaration, ParseError<'i, Self::Error>> {
        let (vendor_prefix, unprefixedPropertyName) =
            VendorPrefix::findPrefixIfAnyForAsciiLowerCaseName(
                name.to_ascii_lowercase(),
            );

        let name = Atom::from(unprefixedPropertyName);

        let value = input.parse_until_before(Delimiter::Bang, |input| {
            if let Ok(cssWideKeyword) = input.r#try(CssWideKeyword::parse) {
                Ok(UnparsedPropertyValue::CssWideKeyword(cssWideKeyword))
            } else {
                Ok(UnparsedPropertyValue::SpecifiedValue(
                    SpecifiedValue::parse(self.context, input)?,
                ))
            }
        })?;

        let importance = I::validateParsedImportance(Importance::parse(input))?;

        input.expect_exhausted()?;

        Ok(PropertyDeclaration {
            vendor_prefix,
            name,
            value,
            importance,
        })
    }
}
