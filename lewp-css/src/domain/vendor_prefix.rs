// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.

use {cssparser::ToCss, std::fmt};

//noinspection SpellCheckingInspection
/// Vendor prefixes
/// Sort order is such that -o- sorts before -webkit- and -ms- sorts after -webkit-, but -epub- (which is only supported by Webkit) sorts before -webkit-
/// There ae other, now rare prefixes, such as -vx- (for Opera before -o-), -wap- (for WAP; a very defunct standard from 1999), -khtml- (for Webkit's predecessor) and so on.
/// However, there are hardly ever encountered and so aren't explicitly coded for.
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum VendorPrefix {
    /// -o- prefix (legacy Opera Presto prefix).
    o,

    /// -moz- prefix.
    moz,

    /// -epub- prefix
    epub,

    /// -webkit- prefix (Is sometimes also used by IE, Edge and Blink-based browsers (Chrome and Opera)).
    webkit,

    /// -ms- prefix.
    ms,

    /// -servo- prefix
    servo,

    /// An unrecognised prefix, usually implies unusual or mistaken CSS
    Unrecognised(String),
}

impl ToCss for VendorPrefix {
    fn to_css<W: fmt::Write>(&self, dest: &mut W) -> fmt::Result {
        use self::VendorPrefix::*;

        match *self {
            o => dest.write_str("-o-"),

            moz => dest.write_str("-moz-"),

            webkit => dest.write_str("-webkit-"),

            ms => dest.write_str("-ms-"),

            servo => dest.write_str("-servo-"),

            epub => dest.write_str("-epub-"),

            Unrecognised(ref prefix) => {
                dest.write_char('-')?;
                dest.write_str(prefix.as_str())?;
                dest.write_char('-')
            }
        }
    }
}

impl VendorPrefix {
    /// Finds a prefix for an ascii lower case name, returning the prefix (if any) and the unprefixed name
    /// Is not confused by CSS custom properties which start `--`
    #[inline(always)]
    pub fn findPrefixIfAnyForAsciiLowerCaseName<'i>(
        asciiLowerCaseName: String,
    ) -> (Option<VendorPrefix>, String) {
        if asciiLowerCaseName.len() < 3 {
            return (None, asciiLowerCaseName);
        }

        {
            let (firstCharacter, remainder) = asciiLowerCaseName.split_at(1);

            if firstCharacter == "-" && !remainder.starts_with('-') {
                let mut split = remainder.splitn(2, '-');
                let prefix = split.next().unwrap();
                let unprefixedRemainder = split.next().unwrap();

                use self::VendorPrefix::*;

                return match prefix {
                    "o" => (Some(o), unprefixedRemainder.to_owned()),

                    "moz" => (Some(moz), unprefixedRemainder.to_owned()),

                    "webkit" => (Some(webkit), unprefixedRemainder.to_owned()),

                    "ms" => (Some(ms), unprefixedRemainder.to_owned()),

                    "servo" => (Some(servo), unprefixedRemainder.to_owned()),

                    _ => (
                        Some(Unrecognised(prefix.to_owned())),
                        unprefixedRemainder.to_owned(),
                    ),
                };
            }
        }

        (None, asciiLowerCaseName)
    }

    /// Prefixes a name with a vendor prefix, eg 'background' might become '-moz-background' if Self is `moz`
    #[inline(always)]
    pub fn prefix(&self, name: &str) -> String {
        use self::VendorPrefix::*;

        fn knownPrefix(prefix: &str, name: &str) -> String {
            let mut prefixed = String::with_capacity(prefix.len() + name.len());
            prefixed.push_str(prefix);
            prefixed.push_str(name);
            prefixed
        }

        match self {
            &o => knownPrefix("-o-", name),

            &moz => knownPrefix("-moz-", name),

            &epub => knownPrefix("-epub-", name),

            &webkit => knownPrefix("-webkit-", name),

            &ms => knownPrefix("-ms-", name),

            &servo => knownPrefix("-servo-", name),

            &Unrecognised(ref prefix) => {
                let mut prefixed =
                    String::with_capacity(1 + prefix.len() + 1 + name.len());
                prefixed.push('-');
                prefixed.push_str(prefix);
                prefixed.push('-');
                prefixed.push_str(name);
                prefixed
            }
        }
    }
}
