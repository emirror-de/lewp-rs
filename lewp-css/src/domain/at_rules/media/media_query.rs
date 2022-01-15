// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.

use {
    super::{MediaQueryType, Qualifier},
    crate::{
        domain::at_rules::media::MediaExpression,
        parsers::{
            separators::{Comma, Separated},
            ParserContext,
        },
        CustomParseError,
    },
    cssparser::{ParseError, Parser, ToCss},
    either::{Left, Right},
    std::fmt::{self},
};

/// A [media query][mq].
///
/// [mq]: <https://drafts.csswg.org/mediaqueries/>
#[derive(Clone, Debug, PartialEq)]
pub struct MediaQuery {
    /// The qualifier for this query.
    pub qualifier: Option<Qualifier>,

    /// The media type for this query, that can be known, unknown, or "all".
    pub media_type: MediaQueryType,

    /// The set of expressions that this media query contains.
    pub expressions: Vec<MediaExpression>,
}

impl Separated for MediaQuery {
    type Delimiter = Comma;
}

impl ToCss for MediaQuery {
    fn to_css<W: fmt::Write>(&self, dest: &mut W) -> fmt::Result {
        if let Some(qualifier) = self.qualifier {
            qualifier.to_css(dest)?;
            dest.write_char(' ')?;
        }

        use self::MediaQueryType::*;

        match self.media_type {
            All => {
                // We need to print "all" if there's a qualifier, or there's just an empty list of expressions.
                // Otherwise, we'd serialize media queries like "(min-width: 40px)" in "all (min-width: 40px)", which is unexpected.
                if self.qualifier.is_some() || self.expressions.is_empty() {
                    dest.write_str("all")?;
                }
            }

            Concrete(ref value) => value.to_css(dest)?,
        }

        if self.expressions.is_empty() {
            return Ok(());
        }

        if self.media_type != All || self.qualifier.is_some() {
            dest.write_str(" and ")?;
        }

        self.expressions[0].to_css(dest)?;
        for expression in self.expressions.iter().skip(1) {
            dest.write_str(" and ")?;
            expression.to_css(dest)?;
        }

        Ok(())
    }
}

impl MediaQuery {
    /// Return a media query that never matches, used for when we fail to parse a given media query.
    #[inline(always)]
    pub(crate) fn never_matching() -> Self {
        Self {
            qualifier: Some(Qualifier::Not),
            media_type: MediaQueryType::All,
            expressions: vec![],
        }
    }

    /// Parse a media query given css input.
    ///
    /// Returns an error if any of the expressions is unknown.
    pub(crate) fn parse<'i, 't>(
        context: &ParserContext,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i, CustomParseError<'i>>> {
        let mut expressions = vec![];

        use self::Qualifier::*;

        let qualifier = if input
            .r#try(|input| input.expect_ident_matching("only"))
            .is_ok()
        {
            Some(Only)
        } else if input
            .r#try(|input| input.expect_ident_matching("not"))
            .is_ok()
        {
            Some(Not)
        } else {
            None
        };

        let isThereAValidMediaType = input.r#try(|input| {
            if let Ok(ident) = input.expect_ident() {
                match MediaQueryType::parse(ident).map_err(ParseError::from) {
                    Ok(mediaType) => Ok(Left(mediaType)),
                    Err(error) => Ok(Right(error)),
                }
            } else {
                Err(())
            }
        });

        let media_type = match isThereAValidMediaType {
            Ok(Left(media_type)) => media_type,

            Ok(Right(error)) => return Err(error),

            Err(()) => {
                // Media type is only optional if qualifier is not specified.
                if qualifier.is_some() {
                    return Err(ParseError::from(
                        CustomParseError::MediaTypeIsOnlyOptionalIfQualifiedIsNotSpecified,
                    ));
                }

                // Without a media type, require at least one expression.
                expressions.push(MediaExpression::parse(context, input)?);

                MediaQueryType::All
            }
        };

        // Parse any subsequent expressions
        loop {
            if input
                .r#try(|input| input.expect_ident_matching("and"))
                .is_err()
            {
                return Ok(Self {
                    qualifier,
                    media_type,
                    expressions,
                });
            }
            expressions.push(MediaExpression::parse(context, input)?)
        }
    }
}
