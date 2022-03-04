// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.

use {
    super::ParserContext,
    crate::{
        domain::at_rules::viewport::{
            ViewportDescriptor::{self, *},
            ViewportDescriptorDeclaration,
            ViewportLength,
            ViewportOrientation,
            ViewportUserZoom,
            ViewportZoom,
        },
        CustomParseError,
    },
    cssparser::{
        AtRuleParser,
        CowRcStr,
        DeclarationParser,
        ParseError,
        ParseErrorKind,
        Parser,
        SourceLocation,
    },
};

pub(crate) struct ViewportAtRuleParser<'a> {
    pub(crate) context: &'a ParserContext,
}

impl<'a, 'i> AtRuleParser<'i> for ViewportAtRuleParser<'a> {
    type Prelude = ();

    type AtRule = ViewportDescriptorDeclaration;

    type Error = CustomParseError<'i>;
}

impl<'a, 'i> DeclarationParser<'i> for ViewportAtRuleParser<'a> {
    type Declaration = ViewportDescriptorDeclaration;

    type Error = CustomParseError<'i>;

    fn parse_value<'t>(
        &mut self,
        name: CowRcStr<'i>,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self::Declaration, ParseError<'i, CustomParseError<'i>>> {
        #[inline(always)]
        fn parse_property<
            'i,
            't,
            Parse: FnOnce(
                &mut Parser<'i, 't>,
            ) -> Result<
                Intermediate,
                ParseError<'i, CustomParseError<'i>>,
            >,
            Intermediate,
            Constructor: FnOnce(Intermediate) -> ViewportDescriptor,
        >(
            input: &mut Parser<'i, 't>,
            constructor: Constructor,
            parse: Parse,
        ) -> Result<
            ViewportDescriptorDeclaration,
            ParseError<'i, CustomParseError<'i>>,
        > {
            ViewportDescriptorDeclaration::parse_important(
                constructor((parse(input))?),
                input,
            )
        }

        #[inline(always)]
        fn parse_shorthand_property<
            'a,
            'i,
            't,
            Constructor: FnOnce(
                ViewportLength,
                Option<ViewportLength>,
            ) -> ViewportDescriptor,
        >(
            input: &mut Parser<'i, 't>,
            this: &ViewportAtRuleParser<'a>,
            constructor: Constructor,
        ) -> Result<
            ViewportDescriptorDeclaration,
            ParseError<'i, CustomParseError<'i>>,
        > {
            let minimum = this.parseViewportLength(input)?;
            let maximum =
                match input.r#try(|input| this.parseViewportLength(input)) {
                    Err(_) => None,
                    Ok(maximum) => Some(maximum),
                };
            ViewportDescriptorDeclaration::parse_important(
                constructor(minimum, maximum),
                input,
            )
        }

        match_ignore_ascii_case! {
            &*name,

            "min-width" => parse_property(input, MinWidth, |input| self.parseViewportLength(input)),

            "max-width" => parse_property(input, MaxWidth, |input| self.parseViewportLength(input)),

            "width" => parse_shorthand_property(input, self, |minimum, maximum| Width { minimum, maximum }),

            "min-height" => parse_property(input, MinHeight, |input| self.parseViewportLength(input)),

            "max-height" => parse_property(input, MaxHeight, |input| self.parseViewportLength(input)),

            "height" => parse_shorthand_property(input, self, |minimum, maximum| Height { minimum, maximum }),

            "zoom" => parse_property(input, Zoom, |input| self.parseZoom(input)),

            "min-zoom" => parse_property(input, MinZoom, |input| self.parseZoom(input)),

            "max-zoom" => parse_property(input, MaxZoom, |input| self.parseZoom(input)),

            "user-zoom" => parse_property(input, UserZoom, ViewportUserZoom::parse),

            "orientation" => parse_property(input, Orientation, ViewportOrientation::parse),

            _ => Err(ParseError {
                kind: ParseErrorKind::Custom(CustomParseError::UnexpectedViewportProperty(name.clone())),
                location: SourceLocation {
                    line: 0,
                    column: 0,
                },
            }),
        }
    }
}

impl<'a> ViewportAtRuleParser<'a> {
    #[inline(always)]
    fn parseViewportLength<'i, 't>(
        &self,
        input: &mut Parser<'i, 't>,
    ) -> Result<ViewportLength, ParseError<'i, CustomParseError<'i>>> {
        ViewportLength::parse(self.context, input)
    }

    #[inline(always)]
    fn parseZoom<'i, 't>(
        &self,
        input: &mut Parser<'i, 't>,
    ) -> Result<ViewportZoom, ParseError<'i, CustomParseError<'i>>> {
        ViewportZoom::parse(self.context, input)
    }
}
