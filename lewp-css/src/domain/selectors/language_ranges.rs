// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.

use {super::LanguageRange, cssparser::ToCss, std::fmt};

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct LanguageRanges(pub Vec<LanguageRange>);

impl ToCss for LanguageRanges {
    fn to_css<W: fmt::Write>(&self, dest: &mut W) -> fmt::Result {
        if self.0.is_empty() {
            return Ok(());
        }

        let mut iterator = self.0.iter();
        iterator.next().unwrap().to_css(dest)?;
        for languageRange in iterator {
            dest.write_char(',')?;
            languageRange.to_css(dest)?;
        }
        Ok(())
    }
}

impl LanguageRanges {
    /// Returns whether the language is matched, as defined by [RFC 4647](<https://tools.ietf.org/html/rfc4647#section-3.3.2>).
    pub fn matches_language(&self, tag: &str) -> bool {
        self.0
            .iter()
            .any(|languageRange| languageRange.matches_language(tag))
    }
}
