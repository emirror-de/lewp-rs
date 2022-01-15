// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.

use {
    crate::domain::{at_rules::media::MediaList, SpecifiedUrl},
    cssparser::ToCss,
    std::fmt,
};

/// The [`@import`][import] at-rule.
///
/// [import]: <https://drafts.csswg.org/css-cascade-3/#at-import>
#[derive(Debug, Clone)]
pub struct ImportAtRule {
    /// The `<url>` this `@import` rule is loading.
    pub url: SpecifiedUrl,

    pub media_list: MediaList,
}

impl ToCss for ImportAtRule {
    fn to_css<W: fmt::Write>(&self, dest: &mut W) -> fmt::Result {
        dest.write_str("@import ")?;
        self.url.to_css(dest)?;

        if self.media_list.is_not_empty() {
            dest.write_char(' ')?;
            self.media_list.to_css(dest)?;
        }

        dest.write_char(';')
    }
}
