// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.

use {cssparser::ToCss, std::fmt};

/// <https://drafts.csswg.org/mediaqueries/#mq-prefix>
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum Qualifier {
    /// Hide a media query from legacy UAs:
    /// <https://drafts.csswg.org/mediaqueries/#mq-only>
    Only,

    /// Negate a media query:
    /// <https://drafts.csswg.org/mediaqueries/#mq-not>
    Not,
}

impl ToCss for Qualifier {
    fn to_css<W: fmt::Write>(&self, dest: &mut W) -> fmt::Result {
        use self::Qualifier::*;

        let ident = match *self {
            Only => "only",

            Not => "not",
        };

        dest.write_str(ident)
    }
}
