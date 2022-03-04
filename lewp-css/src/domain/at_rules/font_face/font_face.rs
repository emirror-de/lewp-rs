// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.

use {
    super::{
        FamilyName,
        FontDisplay,
        FontFaceAtRule,
        FontFeatureSettings,
        FontLanguageOverride,
        FontStretch,
        FontStyle,
        FontWeight,
        Source,
    },
    cssparser::UnicodeRange,
};

/// A @font-face rule that is known to have font-family and src declarations.
pub struct FontFace<'a>(pub &'a FontFaceAtRule);

impl<'a> FontFace<'a> {
    /// The name of this font face
    pub fn family(&self) -> &FamilyName {
        self.0.family.as_ref().unwrap()
    }

    /// The alternative sources for this font face.
    pub fn sources(&self) -> &Vec<Source> {
        self.0.sources.as_ref().unwrap()
    }

    /// The style of this font face
    pub fn style(&self) -> FontStyle {
        if let Some(ref value) = self.0.style {
            *value
        } else {
            FontStyle::normal
        }
    }

    /// The style of this font face
    pub fn weight(&self) -> FontWeight {
        if let Some(ref value) = self.0.weight {
            *value
        } else {
            FontWeight::normal
        }
    }

    /// The stretch of this font face
    pub fn stretch(&self) -> FontStretch {
        if let Some(ref value) = self.0.stretch {
            *value
        } else {
            FontStretch::normal
        }
    }

    /// The display of this font face
    pub fn display(&self) -> FontDisplay {
        if let Some(ref value) = self.0.display {
            *value
        } else {
            FontDisplay::auto
        }
    }

    /// The ranges of code points outside of which this font face should not be used.
    pub fn unicode_range(&self) -> Vec<UnicodeRange> {
        if let Some(ref value) = self.0.unicode_range {
            value.clone()
        } else {
            vec![UnicodeRange {
                start: 0,
                end: 0x10FFFF,
            }]
        }
    }

    /// The feature settings of this font face.
    pub fn feature_settings(&self) -> FontFeatureSettings {
        if let Some(ref value) = self.0.feature_settings {
            value.clone()
        } else {
            FontFeatureSettings::normal()
        }
    }

    /// The language override of this font face.
    pub fn language_override(&self) -> FontLanguageOverride {
        if let Some(ref value) = self.0.language_override {
            *value
        } else {
            FontLanguageOverride::normal
        }
    }
}
