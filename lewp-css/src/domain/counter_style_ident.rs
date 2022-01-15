// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.

use {
    crate::CustomParseError,
    cssparser::{serialize_identifier, ParseError, Parser, ToCss},
    std::fmt,
};

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum CounterStyleIdent {
    decimal,
    decimal_leading_zero,
    arabic_indic,
    armenian,
    upper_armenian,
    lower_armenian,
    bengali,
    cambodian,
    khmer,
    cjk_decimal,
    devanagari,
    georgian,
    gujarati,
    gurmukhi,
    hebrew,
    kannada,
    lao,
    malayalam,
    mongolian,
    myanmar,
    oriya,
    persian,
    lower_roman,
    upper_roman,
    tamil,
    telugu,
    thai,
    tibetan,
    lower_alpha,
    lower_latin,
    upper_alpha,
    upper_latin,
    cjk_earthly_branch,
    cjk_heavenly_stem,
    lower_greek,
    hiragana,
    hiragana_iroha,
    katakana,
    katakana_iroha,
    disc,
    circle,
    square,
    disclosure_open,
    disclosure_closed,
    japanese_informal,
    japanese_formal,
    korean_hangul_formal,
    korean_hanja_informal,
    korean_hanja_formal,
    simp_chinese_informal,
    simp_chinese_formal,
    trad_chinese_informal,
    trad_chinese_formal,
    cjk_ideographic,
    ethiopic_numeric,

    Custom(String),
}

impl ToCss for CounterStyleIdent {
    fn to_css<W: fmt::Write>(&self, dest: &mut W) -> fmt::Result {
        use self::CounterStyleIdent::*;

        let name = match *self {
            decimal => "decimal",
            decimal_leading_zero => "decimal-leading-zero",
            arabic_indic => "arabic-indic",
            armenian => "armenian",
            upper_armenian => "upper-armenian",
            lower_armenian => "lower-armenian",
            bengali => "bengali",
            cambodian => "cambodian",
            khmer => "khmer",
            cjk_decimal => "cjk-decimal",
            devanagari => "devanagari",
            georgian => "georgian",
            gujarati => "gujarati",
            gurmukhi => "gurmukhi",
            hebrew => "hebrew",
            kannada => "kannada",
            lao => "lao",
            malayalam => "malayalam",
            mongolian => "mongolian",
            myanmar => "myanmar",
            oriya => "oriya",
            persian => "persian",
            lower_roman => "lower-roman",
            upper_roman => "upper-roman",
            tamil => "tamil",
            telugu => "telugu",
            thai => "thai",
            tibetan => "tibetan",
            lower_alpha => "lower-alpha",
            lower_latin => "lower-latin",
            upper_alpha => "upper-alpha",
            upper_latin => "upper-latin",
            cjk_earthly_branch => "cjk-earthly-branch",
            cjk_heavenly_stem => "cjk-heavenly-stem",
            lower_greek => "lower-greek",
            hiragana => "hiragana",
            hiragana_iroha => "hiragana-iroha",
            katakana => "katakana",
            katakana_iroha => "katakana-iroha",
            disc => "disc",
            circle => "circle",
            square => "square",
            disclosure_open => "disclosure-open",
            disclosure_closed => "disclosure-closed",
            japanese_informal => "japanese-informal",
            japanese_formal => "japanese-formal",
            korean_hangul_formal => "korean-hangul-formal",
            korean_hanja_informal => "korean-hanja-informal",
            korean_hanja_formal => "korean-hanja-formal",
            simp_chinese_informal => "simp-chinese-informal",
            simp_chinese_formal => "simp-chinese-formal",
            trad_chinese_informal => "trad-chinese-informal",
            trad_chinese_formal => "trad-chinese-formal",
            cjk_ideographic => "cjk-ideographic",
            ethiopic_numeric => "ethiopic-numeric",
            Custom(ref name) => name.as_str(),
        };
        serialize_identifier(name, dest)
    }
}

impl CounterStyleIdent {
    pub(crate) fn parse<'i, 't>(
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i, CustomParseError<'i>>> {
        let ident = input.expect_ident()?;
        Self::from_ident(ident).map_err(|_| {
            ParseError::from(
                CustomParseError::NoneIsNotAllowedInACounterStyleIdent,
            )
        })
    }

    pub(crate) fn parseForCounterStyleAtRule<'i, 't>(
        input: &mut Parser<'i, 't>,
    ) -> Result<Self, ParseError<'i, CustomParseError<'i>>> {
        let counterStyleIdent = Self::parse(input)?;
        if counterStyleIdent.is_not_allowed_in_counter_style_at_rule() {
            Err(ParseError::from(CustomParseError::DecimalOrDiscIsNotAllowedInACounterStyleIdentInACounterStyleAtRule))
        } else {
            Ok(counterStyleIdent)
        }
    }

    pub fn is_not_allowed_in_counter_style_at_rule(&self) -> bool {
        use self::CounterStyleIdent::*;

        match *self {
            decimal | disc => true,
            _ => false,
        }
    }

    pub fn from_ident(anyCaseIdent: &str) -> Result<Self, ()> {
        use self::CounterStyleIdent::*;

        let lowerCaseIdent: String = anyCaseIdent.to_ascii_lowercase();

        static KnownCounterStyleNames: phf::Map<
            &'static str,
            CounterStyleIdent,
        > = phf_macros::phf_map! {
            "decimal" => decimal,
            "decimal-leading-zero" => decimal_leading_zero,
            "arabic-indic" => arabic_indic,
            "armenian" => armenian,
            "upper-armenian" => upper_armenian,
            "lower-armenian" => lower_armenian,
            "bengali" => bengali,
            "cambodian" => cambodian,
            "khmer" => khmer,
            "cjk-decimal" => cjk_decimal,
            "devanagari" => devanagari,
            "georgian" => georgian,
            "gujarati" => gujarati,
            "gurmukhi" => gurmukhi,
            "hebrew" => hebrew,
            "kannada" => kannada,
            "lao" => lao,
            "malayalam" => malayalam,
            "mongolian" => mongolian,
            "myanmar" => myanmar,
            "oriya" => oriya,
            "persian" => persian,
            "lower-roman" => lower_roman,
            "upper-roman" => upper_roman,
            "tamil" => tamil,
            "telugu" => telugu,
            "thai" => thai,
            "tibetan" => tibetan,
            "lower-alpha" => lower_alpha,
            "lower-latin" => lower_latin,
            "upper-alpha" => upper_alpha,
            "upper-latin" => upper_latin,
            "cjk-earthly-branch" => cjk_earthly_branch,
            "cjk-heavenly-stem" => cjk_heavenly_stem,
            "lower-greek" => lower_greek,
            "hiragana" => hiragana,
            "hiragana-iroha" => hiragana_iroha,
            "katakana" => katakana,
            "katakana-iroha" => katakana_iroha,
            "disc" => disc,
            "circle" => circle,
            "square" => square,
            "disclosure-open" => disclosure_open,
            "disclosure-closed" => disclosure_closed,
            "japanese-informal" => japanese_informal,
            "japanese-formal" => japanese_formal,
            "korean-hangul-formal" => korean_hangul_formal,
            "korean-hanja-informal" => korean_hanja_informal,
            "korean-hanja-formal" => korean_hanja_formal,
            "simp-chinese-informal" => simp_chinese_informal,
            "simp-chinese-formal" => simp_chinese_formal,
            "trad-chinese-informal" => trad_chinese_informal,
            "trad-chinese-formal" => trad_chinese_formal,
            "cjk-ideographic" => cjk_ideographic,
            "ethiopic-numeric" => ethiopic_numeric,
        };

        match KnownCounterStyleNames.get(&lowerCaseIdent[..]) {
            Some(value) => Ok(value.clone()),
            None => {
                if lowerCaseIdent == "none" {
                    Err(())
                } else {
                    Ok(Custom(lowerCaseIdent))
                }
            }
        }
    }
}
