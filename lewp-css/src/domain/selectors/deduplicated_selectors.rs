// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.

use {super::OurSelectorImpl, cssparser::ToCss, selectors::parser::Selector};

/// There is at least one selector
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct DeduplicatedSelectors(pub Vec<Selector<OurSelectorImpl>>);

impl ToCss for DeduplicatedSelectors {
    fn to_css<W: std::fmt::Write>(&self, dest: &mut W) -> std::fmt::Result {
        let mut iter = self.0.iter();
        dest.write_str(&iter.next().unwrap().to_css_string())?;
        for selector in iter {
            dest.write_char(',')?;
            dest.write_str(&selector.to_css_string())?;
        }
        Ok(())
    }
}
