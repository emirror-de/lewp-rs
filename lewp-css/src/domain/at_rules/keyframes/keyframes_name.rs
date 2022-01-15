// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.

use {
    crate::{
        domain::{Atom, CustomIdent},
        CustomParseError,
    },
    cssparser::{
        BasicParseError,
        BasicParseErrorKind,
        ParseError,
        Parser,
        ToCss,
        Token,
    },
    std::{
        fmt,
        hash::{Hash, Hasher},
    },
};

/// <https://drafts.csswg.org/css-animations/#typedef-keyframes-name>
#[derive(Debug, Clone)]
pub enum KeyframesName {
    /// <custom-ident>
    Ident(CustomIdent),

    /// <string>
    QuotedString(Atom),
}

impl KeyframesName {
    /// <https://drafts.csswg.org/css-animations/#dom-csskeyframesrule-name>
    pub fn from_ident(value: &str) -> Self {
        let custom_ident =
            CustomIdent::from_ident(&value.into(), &["none"]).ok();
        match custom_ident {
            Some(ident) => KeyframesName::Ident(ident),
            None => KeyframesName::QuotedString(value.into()),
        }
    }

    /// The name as an Atom
    pub fn as_atom(&self) -> &Atom {
        match *self {
            KeyframesName::Ident(ref ident) => &ident.0,
            KeyframesName::QuotedString(ref atom) => atom,
        }
    }
}

impl PartialEq for KeyframesName {
    fn eq(&self, other: &Self) -> bool {
        self.as_atom() == other.as_atom()
    }
}

impl Eq for KeyframesName {}

impl Hash for KeyframesName {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.as_atom().hash(state)
    }
}

impl ToCss for KeyframesName {
    fn to_css<W: fmt::Write>(&self, dest: &mut W) -> fmt::Result {
        match *self {
            KeyframesName::Ident(ref ident) => ident.to_css(dest),
            KeyframesName::QuotedString(ref atom) => atom.to_css(dest),
        }
    }
}

impl KeyframesName {
    pub(crate) fn parse<'i, 't>(
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i, CustomParseError<'i>>> {
        match input.next() {
            Ok(&Token::Ident(ref s)) => {
                Ok(KeyframesName::Ident(CustomIdent::from_ident(s, &["none"])?))
            }
            Ok(&Token::QuotedString(ref s)) => {
                Ok(KeyframesName::QuotedString(Atom::from(s.as_ref())))
            }
            Ok(t) => Err(ParseError::from(BasicParseError {
                kind: BasicParseErrorKind::UnexpectedToken(t.clone()),
                location: input.state().source_location(),
            })),
            Err(e) => Err(e.into()),
        }
    }
}
