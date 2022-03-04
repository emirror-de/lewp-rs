// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.

use {
    super::{
        ViewportLength,
        ViewportOrientation,
        ViewportUserZoom,
        ViewportZoom,
    },
    cssparser::ToCss,
    std::fmt,
};

#[derive(Clone, Debug, PartialEq)]
#[allow(missing_docs)]
pub enum ViewportDescriptor {
    MinWidth(ViewportLength),

    MaxWidth(ViewportLength),

    /// Width with no maximum is similar to MinWidth; with both values, it is equivalent to MinWidth and MaxWidth
    Width {
        minimum: ViewportLength,
        maximum: Option<ViewportLength>,
    },

    MinHeight(ViewportLength),

    MaxHeight(ViewportLength),

    /// Height with no maximum is similar to MinHeight; with both values, it is equivalent to MinHeight and MaxHeight
    Height {
        minimum: ViewportLength,
        maximum: Option<ViewportLength>,
    },

    Zoom(ViewportZoom),

    MinZoom(ViewportZoom),

    MaxZoom(ViewportZoom),

    UserZoom(ViewportUserZoom),

    Orientation(ViewportOrientation),
}

impl ToCss for ViewportDescriptor {
    fn to_css<W: fmt::Write>(&self, dest: &mut W) -> fmt::Result {
        use self::ViewportDescriptor::*;

        match *self {
            MinWidth(ref descriptor_value) => {
                dest.write_str("min-width")?;
                dest.write_char(':')?;
                descriptor_value.to_css(dest)?;
            }

            MaxWidth(ref descriptor_value) => {
                dest.write_str("max-width")?;
                dest.write_char(':')?;
                descriptor_value.to_css(dest)?;
            }

            Width {
                ref minimum,
                ref maximum,
            } => {
                dest.write_str("width")?;
                dest.write_char(':')?;
                minimum.to_css(dest)?;
                if let &Some(ref maximum) = maximum {
                    dest.write_char(' ')?;
                    maximum.to_css(dest)?;
                }
            }

            MinHeight(ref descriptor_value) => {
                dest.write_str("min-height")?;
                dest.write_char(':')?;
                descriptor_value.to_css(dest)?;
            }

            MaxHeight(ref descriptor_value) => {
                dest.write_str("max-height")?;
                dest.write_char(':')?;
                descriptor_value.to_css(dest)?;
            }

            Height {
                ref minimum,
                ref maximum,
            } => {
                dest.write_str("height")?;
                dest.write_char(':')?;
                minimum.to_css(dest)?;
                if let &Some(ref maximum) = maximum {
                    dest.write_char(' ')?;
                    maximum.to_css(dest)?;
                }
            }

            Zoom(ref descriptor_value) => {
                dest.write_str("zoom")?;
                dest.write_char(':')?;
                descriptor_value.to_css(dest)?;
            }

            MinZoom(ref descriptor_value) => {
                dest.write_str("min-zoom")?;
                dest.write_char(':')?;
                descriptor_value.to_css(dest)?;
            }

            MaxZoom(ref descriptor_value) => {
                dest.write_str("max-zoom")?;
                dest.write_char(':')?;
                descriptor_value.to_css(dest)?;
            }

            UserZoom(ref descriptor_value) => {
                dest.write_str("user-zoom")?;
                dest.write_char(':')?;
                descriptor_value.to_css(dest)?;
            }

            Orientation(ref descriptor_value) => {
                dest.write_str("orientation")?;
                dest.write_char(':')?;
                descriptor_value.to_css(dest)?;
            }
        }
        dest.write_char(';')
    }
}

impl ViewportDescriptor {
    #[inline(always)]
    pub fn css_name(&self) -> &'static str {
        use self::ViewportDescriptor::*;

        match *self {
            MinWidth(_) => "min-width",

            MaxWidth(_) => "max-width",

            Width { .. } => "width",

            MinHeight(_) => "min-height",

            MaxHeight(_) => "max-height",

            Height { .. } => "height",

            Zoom(_) => "zoom",

            MinZoom(_) => "min-zoom",

            MaxZoom(_) => "max-zoom",

            UserZoom(_) => "user-zoom",

            Orientation(_) => "orientation",
        }
    }
}
