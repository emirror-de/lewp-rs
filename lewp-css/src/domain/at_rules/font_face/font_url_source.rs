// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.

use {
    crate::domain::SpecifiedUrl,
    cssparser::{serialize_string, ToCss},
    std::fmt,
};

/// A `UrlSource` represents a font-face source that has been specified with a `url()` function.
///
/// <https://drafts.csswg.org/css-fonts/#src-desc>
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FontUrlSource {
    /// The specified url.
    pub url: SpecifiedUrl,

    /// The format hints specified with the `format()` function.
    /// Examples are "truetype", "opentype" and "woff"
    pub format_hints: Vec<String>,
}

impl ToCss for FontUrlSource {
    fn to_css<W: fmt::Write>(&self, dest: &mut W) -> fmt::Result {
        self.url.to_css(dest)?;

        if !self.format_hints.is_empty() {
            dest.write_str(" format(")?;
            let mut formatHintsIterator = self.format_hints.iter();
            serialize_string(formatHintsIterator.next().unwrap(), dest)?;
            for formatHint in formatHintsIterator {
                dest.write_char(',')?;
                serialize_string(formatHint, dest)?;
            }
            dest.write_char(')')?;
        }

        Ok(())
    }
}
