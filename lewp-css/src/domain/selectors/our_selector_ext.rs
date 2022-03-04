// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.

use {
    super::OurSelector,
    selectors::parser::{Combinator, Component},
};

pub trait OurSelectorExt {
    fn is_false_if_any_selector_is_simple_and_only_uses_the_descendant_combinator(
        &self,
    ) -> bool;
}

impl OurSelectorExt for OurSelector {
    #[inline(always)]
    fn is_false_if_any_selector_is_simple_and_only_uses_the_descendant_combinator(
        &self,
    ) -> bool {
        for component in self.iter_raw_match_order() {
            match *component {
                // Combinators are not allowed except for descendant
                Component::Combinator(ref combinator) => {
                    if combinator != &Combinator::Descendant {
                        return true;
                    }
                }

                // Only simple selectors are allowed (http://www.w3.org/TR/css3-selectors/#simple-selectors)
                Component::PseudoElement(..) => return true,

                _ => {}
            }
        }

        false
    }
}
