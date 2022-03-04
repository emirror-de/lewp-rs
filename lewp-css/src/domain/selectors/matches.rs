// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.

use selectors::{
    context::{MatchingMode, QuirksMode},
    matching::{matches_selector, MatchingContext},
    parser::{AncestorHashes, Selector},
    Element,
    NthIndexCache,
};

/// Returns whether the given element matches this selector.
#[inline]
pub fn matches<E: Element>(selector: &Selector<E::Impl>, element: &E) -> bool {
    const offset: usize = 0;
    const hashes: Option<&AncestorHashes> = None;
    const nth_index_cache: Option<&mut NthIndexCache> = None;
    let mut context = MatchingContext::new(
        MatchingMode::Normal,
        None,
        nth_index_cache,
        QuirksMode::NoQuirks,
    );
    matches_selector(
        selector,
        offset,
        hashes,
        element,
        &mut context,
        &mut |_, _| {},
    )
}
