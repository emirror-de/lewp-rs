// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.

use {
    crate::domain::at_rules::counter_style::System,
    cssparser::{
        BasicParseErrorKind,
        CowRcStr,
        ParseError,
        ParseErrorKind,
        SourceLocation,
        Token,
    },
    selectors::parser::{SelectorParseError, SelectorParseErrorKind},
};

/// Represents all the things that can go wrong when parsing.
#[derive(Debug)]
pub enum CustomParseError<'i> {
    // @-rule
    UnsupportedAtRule(CowRcStr<'i>),
    InvalidParseState,

    // @charset
    UnexpectedCharsetAtRule,

    // @counter-style
    UnsupportedCounterStyleProperty(CowRcStr<'i>),
    InvalidCounterStyleWithoutSymbols(System),
    InvalidCounterStyleNotEnoughSymbols(System),
    InvalidCounterStyleWithoutAdditiveSymbols,
    InvalidCounterStyleExtendsWithSymbols,
    InvalidCounterStyleExtendsWithAdditiveSymbols,
    CounterStyleSystemIsNotKnown(CowRcStr<'i>),
    CounterStyleSymbolsCanNotBeEmpty,
    CounterStyleRangesCanNotHaveStartGreaterThanEnd(i32, i32),
    CounterStylePadMinLengthCanNotBeNegative(i32),
    CounterStyleAdditiveTupleWeightCanNotBeNegative(i32),
    CounterStyleAdditiveSymbolsCanNotHaveASecondWeightEqualToOrGreaterThanTheFirst,
    DecimalOrDiscIsNotAllowedInACounterStyleIdentInACounterStyleAtRule,
    NoneIsNotAllowedInACounterStyleIdent,

    // @document
    DocumentAtRuleUrlMatchingFunctionWasInvalid,
    BadUrlInDeclarationValueBlock(CowRcStr<'i>),
    BadStringInDeclarationValueBlock(CowRcStr<'i>),
    UnbalancedCloseParenthesisInDeclarationValueBlock,
    UnbalancedCloseSquareBracketInDeclarationValueBlock,
    UnbalancedCloseCurlyBracketInDeclarationValueBlock,

    // @font-face
    UnsupportedFontFaceProperty(CowRcStr<'i>),

    // @font-feature-values
    InvalidFontLanguageOverrideIdentifier(CowRcStr<'i>),
    InvalidFontLanguageOverrideOpenTypeLanguageTag(CowRcStr<'i>),
    FontFeatureSettingOpenTypeFeatureTagMustBeFourCharacters(CowRcStr<'i>),
    FontFeatureSettingOpenTypeFeatureTagMustBePrintableAscii(CowRcStr<'i>),
    FontFeatureSettingIfNotAnIntegerMustBeOnOrOff(CowRcStr<'i>),
    FontFeatureSettingIntegerMustBePositive(i32),
    FontFaceAtRuleFontWeightWasNotAValidIdentifierOrInteger,
    FontFaceAtRuleFontFamilyCanNotBeGeneric,

    // @import
    AtRuleImportMustBeBeforeAnyRuleExceptAtRuleCharset,

    // @keyframes
    KeyframePercentageWasNotBetweenZeroAndOneInclusive(f32),
    ImportantIsNotAllowedInKeyframePropertyDeclarationValues,
    UnexpectedTokenWhenParsingZoom(Token<'i>),

    // @media
    InvalidMediaType(CowRcStr<'i>),
    DeprecatedMediaType(CowRcStr<'i>),
    UnrecognisedMediaType(CowRcStr<'i>),
    DeprecatedMediaQueryExpression(CowRcStr<'i>),
    UnsupportedMediaQueryExpression(CowRcStr<'i>),
    RatioNumeratorCanNotBeNegativeOrZero(i32),
    RatioDivisorCanNotBeNegativeOrZero(i32),
    MediaGridMustBeEitherZeroOrOne(i32),
    MediaTransform3DMustBeEitherZeroOrOne(i32),
    MediaTypeIsOnlyOptionalIfQualifiedIsNotSpecified,

    // @namespace
    AtRuleNamespaceMustBeBeforeAnyRuleExceptAtRuleCharsetAndAtRuleImport,
    UnexpectedTokenForAtNamespaceRuleNamespaceValue(Token<'i>),

    // @page
    InvalidPageSelectorPseudoClass(CowRcStr<'i>),
    FontRelativeLengthsAreNotAllowedInAPageAtRule,
    ViewportLengthsAreNotAllowedInAPageAtRule,

    // @supports
    InvalidSupportsCondition(CowRcStr<'i>),

    // @viewport
    UnexpectedViewportProperty(CowRcStr<'i>),

    // selectors
    SpecificSelectorParseError(Box<SelectorParseError<'i>>),
    ThereAreNoSelectors,
    SelectorIsInvalidInContext(String),
    UnsupportedPseudoClassOrElement(String),
    NonTreeStructuralPseudoClassScopeIsObsoleteAsOfFirefox55,

    // custom ident
    UnexpectedCustomIdent(CowRcStr<'i>),
    CustomIdentWasExcluded(CowRcStr<'i>),

    // numbers & units
    CouldNotParseCssSignedNumber(
        crate::domain::numbers::CssNumberConversionError,
        f32,
    ),
    CouldNotParseCssUnsignedNumber(
        crate::domain::numbers::CssNumberConversionError,
        f32,
    ),
    CouldNotParseDimensionLessNumber(f32),
    CouldNotParseDimension(f32, CowRcStr<'i>),
    UnsignedIntegersCanNotBeNegative(i32),
    UnsignedIntegersCanNotBeFloats(f32),

    // expressions (calc(), var(), attr())
    UnknownFunctionInValueExpression(CowRcStr<'i>),
    CssVariablesInVarExpressionsMustStartWithTwoDashes(CowRcStr<'i>),

    // required for From<SelectorParseErrorKind>
    SelectorParseErrorKind(SelectorParseErrorKind<'i>),
}

impl<'i> CustomParseError<'i> {
    #[inline(always)]
    pub(crate) fn unexpectedToken<T>(
        unexpectedToken: &Token<'i>,
    ) -> Result<T, ParseError<'i, CustomParseError<'i>>> {
        Err(ParseError {
            kind: ParseErrorKind::Basic(BasicParseErrorKind::UnexpectedToken(
                unexpectedToken.clone(),
            )),
            location: SourceLocation { line: 0, column: 0 },
        })
    }

    #[inline(always)]
    pub(crate) fn dimensionless<T>(
        value: f32,
    ) -> Result<T, ParseError<'i, CustomParseError<'i>>> {
        Err(ParseError {
            kind: ParseErrorKind::Custom(
                CustomParseError::CouldNotParseDimensionLessNumber(value),
            ),
            location: SourceLocation {
                line: value as u32,
                column: 0,
            },
        })
    }
}

impl<'i> From<CustomParseError<'i>> for ParseError<'i, CustomParseError<'i>> {
    fn from(error: CustomParseError) -> ParseError<CustomParseError> {
        ParseError {
            kind: ParseErrorKind::Custom(error),
            location: SourceLocation { line: 0, column: 0 },
        }
    }
}

impl<'i> From<SelectorParseErrorKind<'i>> for CustomParseError<'i> {
    fn from(error: SelectorParseErrorKind) -> CustomParseError {
        CustomParseError::SelectorParseErrorKind(error)
    }
}
