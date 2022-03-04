// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.

use crate::define_css_keyword_enum;

define_css_keyword_enum! {
    TypeOrUnit:
    "string" => string,
    "color" => color,
    "url" => url,
    "integer" => integer,
    "number" => number,
    "length" => length,
    "em" => em,
    "ex" => ex,
    "px" => px,
    "rem" => rem,
    "vw" => vw,
    "vh" => vh,
    "vmin" => vmin,
    "vmax" => vmax,
    "mm" => mm,
    "cm" => cm,
    "in" => in_,
    "pt" => pt,
    "pc" => pc,
    "angle" => angle,
    "deg" => deg,
    "rad" => rad,
    "time" => time,
    "s" => s,
    "ms" => ms,
    "frequency" => frequency,
    "hz" => Hz,
    "khz" => kHz,
    "%" => percentage,
}

impl Default for TypeOrUnit {
    #[inline(always)]
    fn default() -> Self {
        TypeOrUnit::string
    }
}

impl TypeOrUnit {
    // Returns Err(()) if support not currently present
    #[inline(always)]
    pub fn value_to_css(&self, valueFromAttribute: &str) -> Result<String, ()> {
        use self::TypeOrUnit::*;

        match *self {
            string => Ok(format!("\"{}\"", valueFromAttribute)),

            color => Ok(valueFromAttribute.to_owned()),

            url => Ok(format!("url({})", valueFromAttribute)),

            integer => Ok(valueFromAttribute.to_owned()),

            number => Ok(valueFromAttribute.to_owned()),

            length => Ok(valueFromAttribute.to_owned()),

            em => Ok(format!("{}em", valueFromAttribute)),

            ex => Ok(format!("{}ex", valueFromAttribute)),

            px => Ok(format!("{}px", valueFromAttribute)),

            rem => Ok(format!("{}rem", valueFromAttribute)),

            vw => Ok(format!("{}vw", valueFromAttribute)),

            vh => Ok(format!("{}vh", valueFromAttribute)),

            vmin => Ok(format!("{}vmin", valueFromAttribute)),

            vmax => Ok(format!("{}vmax", valueFromAttribute)),

            mm => Ok(format!("{}mm", valueFromAttribute)),

            cm => Ok(format!("{}cm", valueFromAttribute)),

            in_ => Ok(format!("{}in", valueFromAttribute)),

            pt => Ok(format!("{}pt", valueFromAttribute)),

            pc => Ok(format!("{}pc", valueFromAttribute)),

            angle => Ok(valueFromAttribute.to_owned()),

            deg => Ok(format!("{}deg", valueFromAttribute)),

            rad => Ok(format!("{}rad", valueFromAttribute)),

            time => Ok(valueFromAttribute.to_owned()),

            s => Ok(format!("{}s", valueFromAttribute)),

            ms => Ok(format!("{}ms", valueFromAttribute)),

            frequency => Ok(valueFromAttribute.to_owned()),

            Hz => Ok(format!("{}hz", valueFromAttribute)),

            kHz => Ok(format!("{}khz", valueFromAttribute)),

            percentage => Ok(format!("{}%", valueFromAttribute)),
        }
    }
}
