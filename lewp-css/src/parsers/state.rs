// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.


/// The current state of the parser.
#[derive(Clone, Copy, Eq, Ord, PartialEq, PartialOrd)]
pub(crate) enum State
{
	/// We haven't started parsing rules.
	Start = 1,
	
	/// We're parsing `@import` rules.
	Imports = 2,
	
	/// We're parsing `@namespace` rules.
	Namespaces = 3,
	
	/// We're parsing the main body of the stylesheet.
	Body = 4,

	/// An unrecoverable error has occurred in parsing
	Invalid = 5,
}
