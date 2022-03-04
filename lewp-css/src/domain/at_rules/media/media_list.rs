// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.

use {
    super::{Device, MediaQuery, Qualifier},
    crate::{parsers::ParserContext, CustomParseError},
    cssparser::{Delimiter, ParseError, Parser, ParserInput, ToCss, Token},
    std::fmt,
};

/// A type that encapsulates a media query list.
#[derive(Debug, Clone)]
pub struct MediaList {
    /// The list of media queries.
    pub media_queries: Vec<MediaQuery>,
}

impl ToCss for MediaList {
    fn to_css<W: fmt::Write>(&self, dest: &mut W) -> fmt::Result {
        if self.media_queries.is_empty() {
            return Ok(());
        }

        let mut iterator = self.media_queries.iter();
        iterator.next().unwrap().to_css(dest)?;
        for mediaQuery in iterator {
            dest.write_char(',')?;
            mediaQuery.to_css(dest)?;
        }

        Ok(())
    }
}

impl MediaList {
    /// Is this media list empty?
    pub fn is_not_empty(&self) -> bool {
        !self.media_queries.is_empty()
    }

    /// Create an empty MediaList.
    pub fn empty() -> Self {
        Self {
            media_queries: vec![],
        }
    }

    /// Parse a media query list from CSS.
    ///
    /// Always returns a media query list. If any invalid media query is found, the media query list is only filled with the equivalent of "not all", see:-
    /// <https://drafts.csswg.org/mediaqueries/#error-handling>
    pub(crate) fn parse_media_query_list<'i: 't, 't>(
        context: &ParserContext,
        input: &mut Parser<'i, 't>,
        allowInvalidMediaQueries: bool,
    ) -> Result<MediaList, ParseError<'i, CustomParseError<'i>>> {
        if input.is_exhausted() {
            return Ok(MediaList::empty());
        }

        let mut media_queries = vec![];
        loop {
            match input.parse_until_before(Delimiter::Comma, |i| {
                MediaQuery::parse(context, i)
            }) {
                Ok(mediaQuery) => media_queries.push(mediaQuery),
                Err(error) => {
                    if !allowInvalidMediaQueries {
                        return Err(error);
                    }

                    media_queries.push(MediaQuery::never_matching());
                }
            }

            match input.next() {
                Ok(&Token::Comma) => {}

                Ok(_) => unreachable!(),

                Err(_) => break,
            }
        }

        Ok(MediaList { media_queries })
    }

    /// Evaluate a whole `MediaList` against `Device`.
    pub fn evaluate<D: Device>(&self, device: &D) -> bool {
        // Check if it is an empty media query list or any queries match (OR condition)
        // https://drafts.csswg.org/mediaqueries-4/#mq-list
        self.is_empty()
            || self.media_queries.iter().any(|mediaQuery| {
                use super::MediaQueryType::*;

                let media_match = match mediaQuery.media_type {
                    All => true,
                    Concrete(mediaType) => device.mediaTypeMatches(mediaType),
                };

                // Check if all conditions match (AND condition)
                let query_match = media_match
                    && mediaQuery
                        .expressions
                        .iter()
                        .all(|expression| expression.matches(device));

                // Apply the logical NOT qualifier to the result
                match mediaQuery.qualifier {
                    Some(Qualifier::Not) => !query_match,
                    _ => query_match,
                }
            })
    }

    /// Whether this `MediaList` contains no media queries.
    pub fn is_empty(&self) -> bool {
        self.media_queries.is_empty()
    }

    /// Append a new media query item to the media list.
    /// <https://drafts.csswg.org/cssom/#dom-medialist-appendmedium>
    ///
    /// Returns true if added, false if failed to parse the medium string.
    pub fn append_medium(
        &mut self,
        context: &ParserContext,
        new_medium: &str,
    ) -> bool {
        let mut input = ParserInput::new(new_medium);
        let mut parser = Parser::new(&mut input);
        let new_query = match MediaQuery::parse(context, &mut parser) {
            Ok(query) => query,
            Err(_) => {
                return false;
            }
        };

        // This algorithm doesn't actually matches the current spec, but it matches the behavior of Gecko and Edge.
        // See https://github.com/w3c/csswg-drafts/issues/697
        self.media_queries.retain(|query| query != &new_query);
        self.media_queries.push(new_query);
        true
    }

    /// Delete a media query from the media list.
    /// <https://drafts.csswg.org/cssom/#dom-medialist-deletemedium>
    ///
    /// Returns true if found and deleted, false otherwise.
    pub fn delete_medium(
        &mut self,
        context: &ParserContext,
        old_medium: &str,
    ) -> bool {
        let mut input = ParserInput::new(old_medium);
        let mut parser = Parser::new(&mut input);
        let old_query = match MediaQuery::parse(context, &mut parser) {
            Ok(query) => query,
            Err(_) => {
                return false;
            }
        };

        let old_len = self.media_queries.len();
        self.media_queries.retain(|query| query != &old_query);
        old_len != self.media_queries.len()
    }
}
