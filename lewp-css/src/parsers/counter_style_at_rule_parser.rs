// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.

use {
    super::Parse,
    crate::{
        domain::at_rules::counter_style::CounterStyleAtRule,
        parsers::ParserContext,
        CustomParseError,
    },
    cssparser::{
        AtRuleParser,
        CowRcStr,
        DeclarationParser,
        ParseError,
        Parser,
    },
};

// NOTE: impl<'a, 'b, 'i> DeclarationParser<'i> for CounterStyleRuleParser<'a, 'b> is in the macro definition 'counter_style_descriptors' in 'CounterStyleAtRule.rs'
pub(crate) struct CounterStyleAtRuleParser<'a> {
    pub(crate) context: &'a ParserContext,
    pub(crate) rule: &'a mut CounterStyleAtRule,
}

/// Default methods reject all at rules.
impl<'a, 'i> AtRuleParser<'i> for CounterStyleAtRuleParser<'a> {
    type Prelude = ();

    type AtRule = ();

    type Error = CustomParseError<'i>;
}

impl<'a, 'i> DeclarationParser<'i> for CounterStyleAtRuleParser<'a> {
    type Declaration = ();

    type Error = CustomParseError<'i>;

    fn parse_value<'t>(
        &mut self,
        name: CowRcStr<'i>,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self::Declaration, ParseError<'i, Self::Error>> {
        match_ignore_ascii_case! {
            &name,

            "system" => self.rule.system = Some(input.parse_entirely(|input| Parse::parse(self.context, input))?),

            "negative" => self.rule.negative = Some(input.parse_entirely(|input| Parse::parse(self.context, input))?),

            "prefix" => self.rule.prefix = Some(input.parse_entirely(|input| Parse::parse(self.context, input))?),

            "suffix" => self.rule.suffix = Some(input.parse_entirely(|input| Parse::parse(self.context, input))?),

            "range" => self.rule.range = Some(input.parse_entirely(|input| Parse::parse(self.context, input))?),

            "pad" => self.rule.pad = Some(input.parse_entirely(|input| Parse::parse(self.context, input))?),

            "fallback" => self.rule.fallback = Some(input.parse_entirely(|input| Parse::parse(self.context, input))?),

            "symbols" => self.rule.symbols = Some(input.parse_entirely(|input| Parse::parse(self.context, input))?),

            "additive-symbols" => self.rule.additive_symbols = Some(input.parse_entirely(|input| Parse::parse(self.context, input))?),

            "speak-as" => self.rule.speak_as = Some(input.parse_entirely(|input| Parse::parse(self.context, input))?),

            _ => return Err(ParseError::from(CustomParseError::UnsupportedCounterStyleProperty(name.clone())))
        }

        Ok(())
    }
}
