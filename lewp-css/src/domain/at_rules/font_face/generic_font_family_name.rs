// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.

use {cssparser::ToCss, std::fmt};

#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum GenericFontFamilyName {
    serif,
    sans_serif,
    cursive,
    fantasy,
    monospace,
}

impl ToCss for GenericFontFamilyName {
    fn to_css<W: fmt::Write>(&self, dest: &mut W) -> fmt::Result {
        use self::GenericFontFamilyName::*;

        let name = match *self {
            serif => "serif",
            sans_serif => "sans-serif",
            cursive => "cursive",
            fantasy => "fantasy",
            monospace => "monospace",
        };

        // All generic values accepted by the parser are known to not require escaping.
        write!(dest, "{}", name)
    }
}
