// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.

use {
    crate::{
        domain::Atom,
        parsers::separators::{Comma, Separated},
    },
    cssparser::{serialize_identifier, serialize_string, ToCss},
    std::fmt,
};

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct LanguageRange(pub Atom);

impl Separated for LanguageRange {
    type Delimiter = Comma;
}

impl ToCss for LanguageRange {
    fn to_css<W: fmt::Write>(&self, dest: &mut W) -> fmt::Result {
        let value = &(self.0).0;

        let isCssIdentifier = if value.is_empty() {
            false
        } else {
            let mut characters = value.chars();

            match characters.next().unwrap() {
                'a'..='z' | 'A'..='Z' | '-' => {
                    let mut isCssIdentifier = true;
                    for character in characters {
                        match character {
                            'a'..='z' | 'A'..='Z' | '-' | '0'..='9' => continue,
                            _ => {
                                isCssIdentifier = false;
                                break;
                            }
                        }
                    }
                    isCssIdentifier
                }
                _ => false,
            }
        };

        if isCssIdentifier {
            serialize_identifier(value, dest)
        } else {
            serialize_string(value, dest)
        }
    }
}

impl LanguageRange {
    /// Returns whether the language is matched, as defined by [RFC 4647](<https://tools.ietf.org/html/rfc4647#section-3.3.2>).
    pub fn matches_language(&self, tag: &str) -> bool {
        // step 1
        let mut range_subtags = (self.0).0.split('\x2d');
        let mut tag_subtags = tag.split('\x2d');

        // step 2
        // Note: [Level-4 spec](https://drafts.csswg.org/selectors/#lang-pseudo) check for wild card
        if let (Some(range_subtag), Some(tag_subtag)) =
            (range_subtags.next(), tag_subtags.next())
        {
            if !(range_subtag.eq_ignore_ascii_case(tag_subtag)
                || range_subtag.eq_ignore_ascii_case("*"))
            {
                return false;
            }
        }

        let mut current_tag_subtag = tag_subtags.next();

        // step 3
        for range_subtag in range_subtags {
            // step 3a
            if range_subtag == "*" {
                continue;
            }

            match current_tag_subtag {
                Some(tag_subtag) => {
                    // step 3c
                    if range_subtag.eq_ignore_ascii_case(tag_subtag) {
                        current_tag_subtag = tag_subtags.next();
                        continue;
                    }

                    // step 3d
                    if tag_subtag.len() == 1 {
                        return false;
                    }

                    // else step 3e - continue with loop
                    current_tag_subtag = tag_subtags.next();
                    if current_tag_subtag.is_none() {
                        return false;
                    }
                }

                // step 3b
                None => {
                    return false;
                }
            }
        }

        // step 4
        true
    }
}
