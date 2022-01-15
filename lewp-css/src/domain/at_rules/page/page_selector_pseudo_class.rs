// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.

use std::fmt;
use cssparser::{ToCss, Parser, ParseError};
use crate::CustomParseError;
use PageSelectorPseudoClass::*;
use either::{Left, Right, Either};

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum PageSelectorPseudoClass
{
	blank,
	first,
	left,
	right,
	recto,
	verso,
}

impl ToCss for PageSelectorPseudoClass
{
	fn to_css<W: fmt::Write>(&self, dest: &mut W) -> fmt::Result
	{
		use self::PageSelectorPseudoClass::*;
		
		let value = match *self
		{
			blank => ":blank",
			first => ":first",
			left => ":left",
			right => ":right",
			recto => ":recto",
			verso => ":verso",
		};
		dest.write_str(value)
	}
}

impl PageSelectorPseudoClass
{
	pub(crate) fn parse<'i, 't>(input: &mut Parser<'i, 't>) -> Result<Option<Self>, ParseError<'i, CustomParseError<'i>>>
	{
		let result: Result<Either<Self, ParseError<'i, CustomParseError<'i>>>, ()> = input.r#try(|input|
		{
			input.expect_colon().map_err(|_| ())?;
			
			match input.expect_ident()
			{
				Err(basicParseError) => Ok(Right(ParseError::from(basicParseError))),
				
				Ok(pageSelectorName) => match_ignore_ascii_case!
				{
					&*pageSelectorName,
				
					"blank" => Ok(Left(blank)),
				
					"first" => Ok(Left(first)),
				
					"left" => Ok(Left(left)),
				
					"right" => Ok(Left(right)),
				
					"recto" => Ok(Left(recto)),
				
					"verso" => Ok(Left(verso)),
					
					_ => Ok(Right(ParseError::from(CustomParseError::InvalidPageSelectorPseudoClass(pageSelectorName.clone()))))
				},
			}
		});
		
		match result
		{
			Ok(Left(pageSelector)) => Ok(Some(pageSelector)),
			Ok(Right(error)) => Err(error),
			Err(_) => Ok(None),
		}
	}
}
