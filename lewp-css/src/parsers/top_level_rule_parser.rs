// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.

use {
    super::{
        qualified_rule_parser_prelude::QualifiedRuleParserPrelude,
        AtRuleBlockPrelude,
        NestedRuleParser,
    },
    crate::{
        domain::{
            at_rules::{
                import::ImportAtRule,
                media::MediaList,
                namespace::{
                    NamespaceAtRule,
                    NamespacePrefix,
                    NamespaceUrl,
                    Namespaces,
                },
            },
            Atom,
            CssRule,
            SpecifiedUrl,
        },
        parsers::{ParserContext, State},
        CustomParseError,
    },
    cssparser::{
        AtRuleParser,
        BasicParseError,
        BasicParseErrorKind,
        CowRcStr,
        ParseError,
        ParseErrorKind,
        Parser,
        ParserState,
        QualifiedRuleParser,
    },
    std::rc::Rc,
};

/// The parser for the top-level rules in a stylesheet.
pub(crate) struct TopLevelRuleParser {
    /// This won't contain any namespaces, and only nested parsers created with `ParserContext::new_with_rule_type` will.
    pub(crate) context: ParserContext,

    /// The current state of the parser.
    pub(crate) state: State,

    pub(crate) namespaces: Rc<Namespaces>,
}

impl<'i> AtRuleParser<'i> for TopLevelRuleParser {
    type Prelude = AtRuleBlockPrelude;

    type AtRule = CssRule;

    type Error = CustomParseError<'i>;

    fn parse_prelude<'t>(
        &mut self,
        name: CowRcStr<'i>,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self::Prelude, ParseError<'i, Self::Error>> {
        match_ignore_ascii_case! {
            &name,

            "charset" =>
            {
                // @charset is removed by cssparser if it’s the first rule in the stylesheet; anything left is invalid.
                Err(ParseError::from(CustomParseError::UnexpectedCharsetAtRule))
            },

            "import" =>
            {
                if self.state > State::Imports
                {
                    return Err(ParseError::from(CustomParseError::AtRuleImportMustBeBeforeAnyRuleExceptAtRuleCharset));
                }

                Ok(Self::Prelude::Import(self.parseImportAtRule(input)?))
            },

            "namespace" =>
            {
                if self.state > State::Namespaces
                {
                    // "@namespace must be before any rule but @charset and @import"
                    return Err(ParseError::from(CustomParseError::AtRuleNamespaceMustBeBeforeAnyRuleExceptAtRuleCharsetAndAtRuleImport));
                }

                Ok(Self::Prelude::Namespace(self.parseNamespaceAtRule(input)?))
            },

            _ =>
            {
                // Don't allow starting with an invalid state
                if self.state > State::Body
                {
                    self.state = State::Invalid;
                    return Err(ParseError::from(CustomParseError::InvalidParseState));
                }
                self.state = State::Body;

                let mut nested = self.nested();
                <NestedRuleParser as AtRuleParser>::parse_prelude(&mut nested, name.clone(), input)
            }
        }
    }

    //#[inline]
    //fn rule_without_block(
    //    &mut self,
    //    prelude: Self::Prelude,
    //    _start: &ParserState,
    //) -> Result<Self::AtRule, ()> {
    //    use self::State::*;

    //    match prelude {
    //        Import(..) => self.state = Imports,

    //        Namespace(..) => self.state = Namespaces,

    //        _ => {}
    //    }

    //    Ok(prelude)
    //}

    #[inline]
    fn parse_block<'t>(
        &mut self,
        prelude: Self::Prelude,
        _start: &ParserState,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self::AtRule, ParseError<'i, Self::Error>> {
        let result = {
            let mut nested = self.nested();
            <NestedRuleParser as AtRuleParser>::parse_block(
                &mut nested,
                prelude,
                &input.state(),
                input,
            )
        };
        match result {
            Ok(rule) => {
                self.state = State::Body;
                Ok(rule)
            }
            Err(error) => Err(error),
        }
    }
}

impl<'i> QualifiedRuleParser<'i> for TopLevelRuleParser {
    type Prelude = QualifiedRuleParserPrelude;

    type QualifiedRule = CssRule;

    type Error = CustomParseError<'i>;

    #[inline]
    fn parse_prelude<'t>(
        &mut self,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self::Prelude, ParseError<'i, Self::Error>> {
        let mut nested = self.nested();
        <NestedRuleParser as QualifiedRuleParser>::parse_prelude(
            &mut nested,
            input,
        )
    }

    #[inline]
    fn parse_block<'t>(
        &mut self,
        prelude: Self::Prelude,
        _start: &ParserState,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self::QualifiedRule, ParseError<'i, Self::Error>> {
        let result = {
            let mut nested = self.nested();
            <NestedRuleParser as QualifiedRuleParser>::parse_block(
                &mut nested,
                prelude,
                &input.state(),
                input,
            )
        };
        match result {
            Ok(rule) => {
                self.state = State::Body;
                Ok(rule)
            }
            Err(error) => Err(error),
        }
    }
}

impl TopLevelRuleParser {
    #[inline(always)]
    fn nested(&self) -> NestedRuleParser {
        NestedRuleParser {
            context: &self.context,
            namespaces: self.namespaces.clone(),
        }
    }

    #[inline(always)]
    fn parseImportAtRule<'i, 't>(
        &self,
        input: &mut Parser<'i, 't>,
    ) -> Result<ImportAtRule, ParseError<'i, CustomParseError<'i>>> {
        Ok(ImportAtRule {
            url: SpecifiedUrl(
                input.expect_url_or_string()?.as_ref().to_owned(),
            ),
            media_list: MediaList::parse_media_query_list(
                &self.context,
                input,
                false,
            )?,
        })
    }

    #[inline(always)]
    fn parseNamespaceAtRule<'i, 't>(
        &mut self,
        input: &mut Parser<'i, 't>,
    ) -> Result<NamespaceAtRule, ParseError<'i, CustomParseError<'i>>> {
        let prefix: Result<_, ParseError<CustomParseError>> =
            input.r#try(|i| {
                let ident: &CowRcStr = i.expect_ident()?;
                Ok(NamespacePrefix(Atom::from(ident)))
            });

        let prefix = prefix.ok();

        let url = match input.expect_url_or_string() {
            Ok(url_or_string) => NamespaceUrl(Atom::from(url_or_string)),

            Err(BasicParseError {
                kind: BasicParseErrorKind::UnexpectedToken(token),
                ..
            }) => {
                return Err(ParseError {
                    kind: ParseErrorKind::Custom(
                        CustomParseError::UnexpectedTokenForAtNamespaceRuleNamespaceValue(
                            token.clone(),
                        ),
                    ),
                    location: input.state().source_location(),
                })
            }
            Err(error) => return Err(ParseError::from(error)),
        };

        Rc::get_mut(&mut self.namespaces).expect("@namespace rules are parsed before css selectors so no other references to self.namespaces should exist").update(prefix.as_ref(), &url);

        Ok(NamespaceAtRule { prefix, url })
    }
}
