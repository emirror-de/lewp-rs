// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.

use super::{CssRule, CssRules};

pub trait HasCssRules {
    fn css_rules(&self) -> &CssRules;

    fn css_rules_mut(&mut self) -> &mut CssRules;

    fn css_rules_slice(&self) -> &[CssRule];

    fn css_rules_vec(&self) -> &Vec<CssRule>;

    fn css_rules_vec_mut(&mut self) -> &mut Vec<CssRule>;
}
