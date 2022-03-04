// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.

use {
    crate::{
        blocking_io_only_std_fmt_write_to_std_io_write_adaptor::BlockingIoOnlyStdFmtWriteToStdIoWriteAdaptor,
        domain::{
            at_rules::namespace::Namespaces,
            CssRule,
            CssRules,
            HasCssRules,
        },
        parsers::{
            top_level_rule_parser::TopLevelRuleParser,
            ParserContext,
            ParsingMode,
            State,
        },
        quick_error::ResultExt,
        CustomParseError,
        StylesheetError,
    },
    cssparser::{ParseError, Parser, ParserInput, RuleListParser, ToCss},
    std::{fmt, fs::File, io::Read, path::Path},
};

/// Represents an entire CSS stylesheet.
/// The values of property declarations are currently stored as a string. Parsing property declarations is a monster job. If you feel like helping...
#[derive(Debug, Clone)]
pub struct Stylesheet {
    /// The stylesheet's rules.
    pub rules: CssRules,

    /// An optional source map for this stylesheet.
    pub source_map_url: Option<String>,

    /// An optional source URL for this stylesheet.
    pub source_url: Option<String>,
}

impl HasCssRules for Stylesheet {
    #[inline(always)]
    fn css_rules(&self) -> &CssRules {
        &self.rules
    }

    #[inline(always)]
    fn css_rules_mut(&mut self) -> &mut CssRules {
        &mut self.rules
    }

    #[inline(always)]
    fn css_rules_slice(&self) -> &[CssRule] {
        &self.rules.0[..]
    }

    #[inline(always)]
    fn css_rules_vec(&self) -> &Vec<CssRule> {
        &self.rules.0
    }

    #[inline(always)]
    fn css_rules_vec_mut(&mut self) -> &mut Vec<CssRule> {
        &mut self.rules.0
    }
}

impl Stylesheet {
    /// Serializes a Stylesheet to a file path, optionally including source-map and source-url comments.
    /// Will create or truncate `stylesheet_file_path` as required.
    /// Convenience method wrapped `to_css()`.
    #[inline(always)]
    pub fn to_file_path<P: AsRef<Path>>(
        &self,
        stylesheet_file_path: P,
        include_source_urls: bool,
    ) -> Result<(), StylesheetError> {
        let path = stylesheet_file_path.as_ref();
        let file = File::create(path).context(path)?;
        self.to_css(
            &mut BlockingIoOnlyStdFmtWriteToStdIoWriteAdaptor(file),
            include_source_urls,
        )
        .context(path)?;
        Ok(())
    }

    /// Serializes a Stylesheet as a string, optionally including source-map and source-url comments.
    /// Convenience method wrapped `to_css()`.
    #[inline(always)]
    pub fn to_css_string(&self, include_source_urls: bool) -> String {
        let mut string = String::new();
        self.to_css(&mut string, include_source_urls).unwrap();
        string
    }

    /// Serializes a Stylesheet to a vector of UTF-8 encoded bytes.
    /// Convenience method wrapped `to_css_string()`.
    #[inline(always)]
    pub fn to_bytes(&self, include_source_urls: bool) -> Vec<u8> {
        self.to_css_string(include_source_urls).into_bytes()
    }

    /// Serializes a Stylesheet, optionally including source-map and source-url comments.
    pub fn to_css<W: fmt::Write>(
        &self,
        destination: &mut W,
        include_source_urls: bool,
    ) -> fmt::Result {
        if include_source_urls {
            // An older convention was to use '@' instead of '#'

            if let Some(ref source_map_url) = self.source_map_url {
                writeln!(
                    destination,
                    "//# sourceMappingURL=<{}>",
                    source_map_url
                )?;
            }

            if let Some(ref source_url) = self.source_url {
                writeln!(destination, "//# sourceURL=<{}>", source_url)?;
            }
        }

        self.rules.to_css(destination)?;

        Ok(())
    }

    /// Loads and parses a Stylesheet.
    #[inline(always)]
    pub fn from_file_path<P: AsRef<Path>>(
        html_document_file_path: P,
    ) -> Result<Self, StylesheetError> {
        let path = html_document_file_path.as_ref();
        let metadata = path.metadata().context(path)?;

        let mut file = File::open(path).context(path)?;
        let mut css = String::with_capacity(metadata.len() as usize);
        file.read_to_string(&mut css).context(path)?;

        let result = Self::parse(&css);

        match result {
            Ok(stylesheet) => Ok(stylesheet),
            Err(cause) => Err(StylesheetError::Parse(
                path.to_path_buf(),
                cause.location,
                format!("{:?}", cause),
            )),
        }
    }

    /// Parses a string of CSS to produce a stylesheet.
    /// Can be used with the contents of a CSS file.
    /// Assumes the string is UTF-8 encoded.
    /// Does not use a stream of bytes as parsing CSS involves going backwards and forwards a lot... CSS parsing is somewhat evil and is not particularly efficient.
    /// The parser does apply a few small modifications to the incoming CSS, normalizing some pseudo-class, psuedo-element and media query names.
    /// The parser does not parse properties as such, simply keeping them as a CSS string. Hopefully it will one day - there are only 200 odd specialist rules to implement.
    pub fn parse(css: &str) -> Result<Self, ParseError<CustomParseError>> {
        const LineNumberingIsZeroBased: u32 = 0;

        let mut parserInput = ParserInput::new_with_line_number_offset(
            css,
            LineNumberingIsZeroBased,
        );
        let mut input = Parser::new(&mut parserInput);

        let mut rules = Vec::new();

        let topLevelRuleParser = TopLevelRuleParser {
            context: ParserContext {
                rule_type: None,
                parsing_mode: ParsingMode::Default,
            },
            state: State::Start,
            namespaces: Namespaces::empty(),
        };

        {
            let iter = RuleListParser::new_for_stylesheet(
                &mut input,
                topLevelRuleParser,
            );

            for result in iter {
                match result {
                    Ok(rule) => rules.push(rule),
                    Err(preciseParseError) => return Err(preciseParseError.0),
                }
            }
        }

        Ok(Self {
            rules: CssRules(rules),
            source_map_url: input.current_source_map_url().map(String::from),
            source_url: input.current_source_url().map(String::from),
        })
    }
}
