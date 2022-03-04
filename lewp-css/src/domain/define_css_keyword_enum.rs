// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.

#[doc(hidden)]
#[macro_export]
macro_rules! define_css_keyword_enum
{
    ($name: ident: values { $( $css: expr => $variant: ident),+, }
                   aliases { $( $alias: expr => $alias_variant: ident ),+, }) => {
        __define_css_keyword_enum__add_optional_traits!($name [ $( $css => $variant ),+ ]
                                                              [ $( $alias => $alias_variant ),+ ]);
    };
    ($name: ident: values { $( $css: expr => $variant: ident),+, }
                   aliases { $( $alias: expr => $alias_variant: ident ),* }) => {
        __define_css_keyword_enum__add_optional_traits!($name [ $( $css => $variant ),+ ]
                                                              [ $( $alias => $alias_variant ),* ]);
    };
    ($name: ident: values { $( $css: expr => $variant: ident),+ }
                   aliases { $( $alias: expr => $alias_variant: ident ),+, }) => {
        __define_css_keyword_enum__add_optional_traits!($name [ $( $css => $variant ),+ ]
                                                              [ $( $alias => $alias_variant ),+ ]);
    };
    ($name: ident: values { $( $css: expr => $variant: ident),+ }
                   aliases { $( $alias: expr => $alias_variant: ident ),* }) => {
        __define_css_keyword_enum__add_optional_traits!($name [ $( $css => $variant ),+ ]
                                                              [ $( $alias => $alias_variant ),* ]);
    };
    ($name: ident: $( $css: expr => $variant: ident ),+,) => {
        __define_css_keyword_enum__add_optional_traits!($name [ $( $css => $variant ),+ ] []);
    };
    ($name: ident: $( $css: expr => $variant: ident ),+) => {
        __define_css_keyword_enum__add_optional_traits!($name [ $( $css => $variant ),+ ] []);
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __define_css_keyword_enum__add_optional_traits
{
    ($name: ident [ $( $css: expr => $variant: ident ),+ ]
                  [ $( $alias: expr => $alias_variant: ident),* ]) => {
        __define_css_keyword_enum__actual! {
            $name [ PartialOrd, Ord ]
                  [ $( $css => $variant ),+ ]
                  [ $( $alias => $alias_variant ),* ]
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __define_css_keyword_enum__actual
{
	($name: ident [ $( $derived_trait: ident),* ]
                  [ $( $css: expr => $variant: ident ),+ ]
                  [ $( $alias: expr => $alias_variant: ident ),* ]) =>
	{
		#[allow(non_camel_case_types, missing_docs)]
		#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq$(, $derived_trait )* )]
		pub enum $name
		{
			$( $variant ),+
		}

		impl $name
		{
			/// Parse this property from a CSS input stream.
			pub(crate) fn parse<'i, 't>(input: &mut ::cssparser::Parser<'i, 't>) -> Result<$name, ::cssparser::ParseError<'i, $crate::CustomParseError<'i>>>
			{
				let ident = input.expect_ident()?;
                let ident2 = ident.clone();
				Self::from_ident(&ident).map_err(
                    |()| ::cssparser::ParseError {
                        kind: ::cssparser::ParseErrorKind::Basic(::cssparser::BasicParseErrorKind::UnexpectedToken(::cssparser::Token::Ident(ident2.clone()))),
                        location: input.current_source_location()
                    }
                )
			}

			/// Parse this property from an already-tokenized identifier.
			pub(crate) fn from_ident(ident: &str) -> Result<$name, ()>
			{
				match_ignore_ascii_case!
				{
					ident,

					$( $css => Ok($name::$variant), )+

					$( $alias => Ok($name::$alias_variant), )*

					_ => Err(())
				}
			}
		}

		impl ::cssparser::ToCss for $name
		{
			fn to_css<W: ::std::fmt::Write>(&self, dest: &mut W) -> ::std::fmt::Result
			{
				match *self
				{
					$( $name::$variant => dest.write_str($css) ),+
				}
			}
		}
	}
}
