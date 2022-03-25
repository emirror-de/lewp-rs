// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.

use {
    super::{
        DeduplicatedSelectors,
        LanguageRange,
        OurSelectorExt,
        VendorPrefixablePseudoClassName,
    },
    crate::{
        domain::{
            selectors::{
                LanguageRanges,
                OurSelectorImpl,
                SystemMetric,
                TextDirectionality,
                TreeHover,
            },
            Atom,
            VendorPrefix::{self, *},
        },
        parsers::OurSelectorParser,
        CustomParseError,
    },
    cssparser::{CowRcStr, ParseError, Parser, ToCss},
    selectors::parser::NonTSPseudoClass,
    std::{collections::HashMap, fmt},
};

//noinspection SpellCheckingInspection
/// A non tree-structural pseudo-class.
/// See <https://drafts.csswg.org/selectors-4/#structural-pseudos>
#[derive(Clone, Debug, Eq, PartialEq)]
#[allow(missing_docs)]
pub enum NonTreeStructuralPseudoClass {
    active,
    any(Option<VendorPrefix>, DeduplicatedSelectors),
    any_link(Option<VendorPrefix>),
    checked,
    default,
    dir(Option<VendorPrefix>, TextDirectionality),
    disabled,
    enabled,
    /// Only valid in @page
    first,
    focus,
    focus_within,
    in_range,
    invalid,
    is(DeduplicatedSelectors),
    fullscreen(Option<VendorPrefix>),
    hover,
    indeterminate,
    lang(LanguageRanges),
    /// Only valid in @page
    left,
    link,
    optional,
    out_of_range,
    /// The obsolete (as of Firefox 51) `:-moz-placeholder` is re-written when parsed as this.
    placeholder_shown(Option<VendorPrefix>),
    read_only(Option<VendorPrefix>),
    read_write(Option<VendorPrefix>),
    required,
    /// Only valid in @page
    right,
    target,
    valid,
    visited,
    where_(DeduplicatedSelectors),

    /// -servo- only
    case_sensitive_type_attr(Option<VendorPrefix>, Atom),

    /// -servo- only
    non_zero_border(Option<VendorPrefix>),

    /// -moz- only
    broken(Option<VendorPrefix>),

    /// -moz- only
    drag_over(Option<VendorPrefix>),

    /// -moz- only
    first_node(Option<VendorPrefix>),

    /// -moz- only
    focusring(Option<VendorPrefix>),

    /// -moz- only
    full_screen_ancestor(Option<VendorPrefix>),

    /// -moz- only
    handler_blocked(Option<VendorPrefix>),

    /// -moz- only
    handler_crashed(Option<VendorPrefix>),

    /// -moz- only
    handler_disabled(Option<VendorPrefix>),

    /// -moz- only
    last_node(Option<VendorPrefix>),

    /// -moz- only
    list_bullet(Option<VendorPrefix>),

    /// -moz- only
    list_number(Option<VendorPrefix>),

    /// -moz- only
    loading(Option<VendorPrefix>),

    //  -moz- only
    locale_dir(Option<VendorPrefix>, TextDirectionality),

    /// -moz- only
    lwtheme(Option<VendorPrefix>),

    /// -moz- only
    lwtheme_brighttext(Option<VendorPrefix>),

    /// -moz- only
    lwtheme_darktext(Option<VendorPrefix>),

    /// -moz- only
    native_anonymous(Option<VendorPrefix>),

    /// -moz- only
    only_whitespace(Option<VendorPrefix>),

    /// -moz- only
    submit_invalid(Option<VendorPrefix>),

    /// -moz- only
    suppressed(Option<VendorPrefix>),

    /// -moz- only (not listed with other pseudo-classes)
    system_metric(Option<VendorPrefix>, SystemMetric),

    /// -moz- only
    tree_cell(Option<VendorPrefix>),

    /// -moz- only.
    // A psuedo-class function with one value, hover
    tree_cell_text(Option<VendorPrefix>, TreeHover),

    /// -moz- only
    tree_checkbox(Option<VendorPrefix>),

    /// -moz- only
    tree_column(Option<VendorPrefix>),

    /// -moz- only
    tree_drop_feedback(Option<VendorPrefix>),

    /// -moz- only
    tree_image(Option<VendorPrefix>),

    /// -moz- only
    tree_indentation(Option<VendorPrefix>),

    /// -moz- only
    tree_line(Option<VendorPrefix>),

    /// -moz- only
    tree_progressmeter(Option<VendorPrefix>),

    /// -moz- only.
    // A psuedo-class function with one value, hover
    tree_row(Option<VendorPrefix>, TreeHover),

    /// -moz- only
    tree_separator(Option<VendorPrefix>),

    /// -moz- only
    tree_twisty(Option<VendorPrefix>),

    /// -moz- only
    ui_invalid(Option<VendorPrefix>),

    /// -moz- only
    ui_valid(Option<VendorPrefix>),

    /// -moz- only
    user_disabled(Option<VendorPrefix>),

    /// -moz- only
    window_inactive(Option<VendorPrefix>),

    /// -webkit- only, with potential Mozilla support coming.
    autofill(Option<VendorPrefix>),
}

impl NonTSPseudoClass for NonTreeStructuralPseudoClass {
    type Impl = OurSelectorImpl;

    fn is_active_or_hover(&self) -> bool {
        matches!(self, Self::active | Self::hover)
    }

    fn is_user_action_state(&self) -> bool {
        matches!(
            self,
            Self::active
                | Self::hover
                | Self::visited
                | Self::link
                | Self::focus
        )
    }
}

impl ToCss for NonTreeStructuralPseudoClass {
    //noinspection SpellCheckingInspection
    fn to_css<W: fmt::Write>(&self, dest: &mut W) -> fmt::Result {
        #[inline(always)]
        fn write<W: fmt::Write>(
            dest: &mut W,
            classWithColon: &str,
        ) -> fmt::Result {
            dest.write_str(classWithColon)
        }

        #[inline(always)]
        fn write_with_vendor_prefix<W: fmt::Write>(
            dest: &mut W,
            vendorPrefix: &Option<VendorPrefix>,
            classWithoutColon: &str,
        ) -> fmt::Result {
            dest.write_char(':')?;
            if let &Some(ref vendorPrefix) = vendorPrefix {
                vendorPrefix.to_css(dest)?;
            }
            dest.write_str(classWithoutColon)
        }

        #[inline(always)]
        fn write_with_vendor_prefix_value<W: fmt::Write, T: ToCss>(
            dest: &mut W,
            vendorPrefix: &Option<VendorPrefix>,
            classWithoutColon: &str,
            value: &T,
        ) -> fmt::Result {
            dest.write_char(':')?;
            if let &Some(ref vendorPrefix) = vendorPrefix {
                vendorPrefix.to_css(dest)?;
            }
            dest.write_str(classWithoutColon)?;
            dest.write_char('(')?;
            value.to_css(dest)?;
            dest.write_char(')')
        }

        #[inline(always)]
        fn write_with_value<W: fmt::Write, T: ToCss>(
            dest: &mut W,
            classWithoutColon: &str,
            value: &T,
        ) -> fmt::Result {
            dest.write_char(':')?;
            dest.write_str(classWithoutColon)?;
            dest.write_char('(')?;
            value.to_css(dest)?;
            dest.write_char(')')
        }

        match &*self {
            Self::active => write(dest, ":active"),

            Self::any(ref vendorPrefix, ref value) => {
                write_with_vendor_prefix_value(dest, vendorPrefix, "any", value)
            }

            Self::any_link(ref vendorPrefix) => {
                write_with_vendor_prefix(dest, vendorPrefix, "any-link")
            }

            Self::checked => write(dest, ":checked"),

            Self::default => write(dest, ":default"),

            Self::disabled => write(dest, ":disabled"),

            Self::dir(ref vendorPrefix, ref value) => {
                write_with_vendor_prefix_value(dest, vendorPrefix, "dir", value)
            }

            Self::enabled => write(dest, ":enabled"),

            Self::first => write(dest, ":first"),

            Self::focus => write(dest, ":focus"),

            Self::focus_within => write(dest, ":focus-within"),

            Self::fullscreen(ref vendorPrefix) => {
                dest.write_char(':')?;
                let name = if let &Some(ref vendorPrefix) = vendorPrefix {
                    vendorPrefix.to_css(dest)?;

                    match *vendorPrefix {
                        moz => "full-screen",
                        webkit => "full-screen",
                        _ => "fullscreen",
                    }
                } else {
                    "fullscreen"
                };
                dest.write_str(name)
            }

            Self::hover => write(dest, ":hover"),

            Self::indeterminate => write(dest, ":indeterminate"),

            Self::in_range => write(dest, ":in-range"),

            Self::invalid => write(dest, ":invalid"),

            Self::is(ref value) => write_with_value(dest, "is", value),

            Self::lang(ref languages) => {
                dest.write_str(":lang(")?;
                languages.to_css(dest)?;
                dest.write_char(')')
            }

            Self::left => write(dest, ":left"),

            Self::link => write(dest, ":link"),

            Self::optional => write(dest, ":optional"),

            Self::out_of_range => write(dest, ":out-of-range"),

            Self::placeholder_shown(ref vendorPrefix) => {
                write_with_vendor_prefix(
                    dest,
                    vendorPrefix,
                    "placeholder-shown",
                )
            }

            Self::read_only(ref vendorPrefix) => {
                write_with_vendor_prefix(dest, vendorPrefix, "read-only")
            }

            Self::read_write(ref vendorPrefix) => {
                write_with_vendor_prefix(dest, vendorPrefix, "read-write")
            }

            Self::required => write(dest, ":required"),

            Self::right => write(dest, ":right"),

            Self::target => write(dest, ":target"),

            Self::valid => write(dest, ":valid"),

            Self::visited => write(dest, ":visited"),

            Self::where_(ref value) => write_with_value(dest, "where", value),

            // -servo- only
            Self::case_sensitive_type_attr(ref vendorPrefix, ref value) => {
                write_with_vendor_prefix_value(
                    dest,
                    vendorPrefix,
                    "case-sensitive-type-attr",
                    value,
                )
            }

            Self::non_zero_border(ref vendorPrefix) => {
                write_with_vendor_prefix(dest, vendorPrefix, "non-zero-border")
            }

            // -moz- only
            Self::broken(ref vendorPrefix) => {
                write_with_vendor_prefix(dest, vendorPrefix, "broken")
            }

            Self::drag_over(ref vendorPrefix) => {
                write_with_vendor_prefix(dest, vendorPrefix, "drag-over")
            }

            Self::first_node(ref vendorPrefix) => {
                write_with_vendor_prefix(dest, vendorPrefix, "first-node")
            }

            Self::focusring(ref vendorPrefix) => {
                write_with_vendor_prefix(dest, vendorPrefix, "focusring")
            }

            Self::full_screen_ancestor(ref vendorPrefix) => {
                write_with_vendor_prefix(
                    dest,
                    vendorPrefix,
                    "full-screen-ancestor",
                )
            }

            Self::handler_blocked(ref vendorPrefix) => {
                write_with_vendor_prefix(dest, vendorPrefix, "handler-blocked")
            }

            Self::handler_crashed(ref vendorPrefix) => {
                write_with_vendor_prefix(dest, vendorPrefix, "handler-crashed")
            }

            Self::handler_disabled(ref vendorPrefix) => {
                write_with_vendor_prefix(dest, vendorPrefix, "handler-disabled")
            }

            Self::last_node(ref vendorPrefix) => {
                write_with_vendor_prefix(dest, vendorPrefix, "last-node")
            }

            Self::list_bullet(ref vendorPrefix) => {
                write_with_vendor_prefix(dest, vendorPrefix, "list-bullet")
            }

            Self::list_number(ref vendorPrefix) => {
                write_with_vendor_prefix(dest, vendorPrefix, "list-number")
            }

            Self::loading(ref vendorPrefix) => {
                write_with_vendor_prefix(dest, vendorPrefix, "loading")
            }

            Self::locale_dir(ref vendorPrefix, ref value) => {
                write_with_vendor_prefix_value(
                    dest,
                    vendorPrefix,
                    "locale-dir",
                    value,
                )
            }

            Self::lwtheme(ref vendorPrefix) => {
                write_with_vendor_prefix(dest, vendorPrefix, "lwtheme")
            }

            Self::lwtheme_brighttext(ref vendorPrefix) => {
                write_with_vendor_prefix(
                    dest,
                    vendorPrefix,
                    "lwtheme-brighttext",
                )
            }

            Self::lwtheme_darktext(ref vendorPrefix) => {
                write_with_vendor_prefix(dest, vendorPrefix, "lwtheme-darktext")
            }

            Self::native_anonymous(ref vendorPrefix) => {
                write_with_vendor_prefix(dest, vendorPrefix, "native-anonymous")
            }

            Self::only_whitespace(ref vendorPrefix) => {
                write_with_vendor_prefix(dest, vendorPrefix, "only-whitespace")
            }

            Self::submit_invalid(ref vendorPrefix) => {
                write_with_vendor_prefix(dest, vendorPrefix, "submit-invalid")
            }

            Self::suppressed(ref vendorPrefix) => {
                write_with_vendor_prefix(dest, vendorPrefix, "suppressed")
            }

            Self::system_metric(ref vendorPrefix, ref value) => {
                write_with_vendor_prefix_value(
                    dest,
                    vendorPrefix,
                    "system-metric",
                    value,
                )
            }

            Self::tree_cell(ref vendorPrefix) => {
                write_with_vendor_prefix(dest, vendorPrefix, "tree-cell")
            }

            Self::tree_cell_text(ref vendorPrefix, ref value) => {
                write_with_vendor_prefix_value(
                    dest,
                    vendorPrefix,
                    "tree-cell-text",
                    value,
                )
            }

            Self::tree_checkbox(ref vendorPrefix) => {
                write_with_vendor_prefix(dest, vendorPrefix, "tree-checkbox")
            }

            Self::tree_column(ref vendorPrefix) => {
                write_with_vendor_prefix(dest, vendorPrefix, "tree-column")
            }

            Self::tree_drop_feedback(ref vendorPrefix) => {
                write_with_vendor_prefix(
                    dest,
                    vendorPrefix,
                    "tree-drop-feedback",
                )
            }

            Self::tree_image(ref vendorPrefix) => {
                write_with_vendor_prefix(dest, vendorPrefix, "tree-image")
            }

            Self::tree_indentation(ref vendorPrefix) => {
                write_with_vendor_prefix(dest, vendorPrefix, "tree-indentation")
            }

            Self::tree_line(ref vendorPrefix) => {
                write_with_vendor_prefix(dest, vendorPrefix, "tree-line")
            }

            Self::tree_progressmeter(ref vendorPrefix) => {
                write_with_vendor_prefix(
                    dest,
                    vendorPrefix,
                    "tree-progressmeter",
                )
            }

            Self::tree_row(ref vendorPrefix, ref value) => {
                write_with_vendor_prefix_value(
                    dest,
                    vendorPrefix,
                    "tree-row",
                    value,
                )
            }

            Self::tree_separator(ref vendorPrefix) => {
                write_with_vendor_prefix(dest, vendorPrefix, "tree-separator")
            }

            Self::tree_twisty(ref vendorPrefix) => {
                write_with_vendor_prefix(dest, vendorPrefix, "tree-twisty")
            }

            Self::ui_invalid(ref vendorPrefix) => {
                write_with_vendor_prefix(dest, vendorPrefix, "ui-invalid")
            }

            Self::ui_valid(ref vendorPrefix) => {
                write_with_vendor_prefix(dest, vendorPrefix, "ui-valid")
            }

            Self::user_disabled(ref vendorPrefix) => {
                write_with_vendor_prefix(dest, vendorPrefix, "user-disabled")
            }

            Self::window_inactive(ref vendorPrefix) => {
                write_with_vendor_prefix(dest, vendorPrefix, "window-inactive")
            }

            Self::autofill(ref vendorPrefix) => {
                write_with_vendor_prefix(dest, vendorPrefix, "autofill")
            }
        }
    }
}

impl NonTreeStructuralPseudoClass {
    /// Returns true if the evaluation of the pseudo-class depends on the element's attributes.
    pub fn is_attr_based(&self) -> bool {
        use self::NonTreeStructuralPseudoClass::*;

        matches!(*self, lang(..))
    }

    /// <https://drafts.csswg.org/selectors-4/#useraction-pseudos>
    ///
    /// We intentionally skip the link-related ones.
    pub fn is_safe_user_action_state(&self) -> bool {
        use self::NonTreeStructuralPseudoClass::*;

        matches!(*self, active | focus | hover)
    }

    #[inline(always)]
    fn applyVendorPrefix(
        pseudoClassName: VendorPrefixablePseudoClassName,
        applyVendorPrefixToPseudoClasses: &HashMap<
            VendorPrefixablePseudoClassName,
            VendorPrefix,
        >,
    ) -> Option<VendorPrefix> {
        applyVendorPrefixToPseudoClasses
            .get(&pseudoClassName)
            .cloned()
    }

    //noinspection SpellCheckingInspection
    #[inline(always)]
    pub(crate) fn parse_without_arguments<'i>(
        applyVendorPrefixToPseudoClasses: &HashMap<
            VendorPrefixablePseudoClassName,
            VendorPrefix,
        >,
        name: CowRcStr<'i>,
    ) -> Result<Self, ParseError<'i, CustomParseError<'i>>> {
        match_ignore_ascii_case! {
            &name,

            "active" => Ok(Self::active),

            "any-link" => Ok(Self::any_link(Self::applyVendorPrefix(VendorPrefixablePseudoClassName::any_link, applyVendorPrefixToPseudoClasses))),

            "-moz-any-link" => Ok(Self::any_link(Some(moz))),

            "-webkit-any-link" => Ok(Self::any_link(Some(webkit))),

            "checked" => Ok(Self::checked),

            "default" => Ok(Self::default),

            "disabled" => Ok(Self::disabled),

            "enabled" => Ok(Self::enabled),

            "first" => Ok(Self::first),

            "focus" => Ok(Self::focus),

            "focus-within" => Ok(Self::focus_within),

            "fullscreen" => Ok(Self::fullscreen(Self::applyVendorPrefix(VendorPrefixablePseudoClassName::fullscreen, applyVendorPrefixToPseudoClasses))),

            "-ms-fullscreen" => Ok(Self::fullscreen(Some(ms))),

            "-moz-full-screen" => Ok(Self::fullscreen(Some(moz))),

            "-webkit-full-screen" => Ok(Self::fullscreen(Some(webkit))),

            "hover" => Ok(Self::hover),

            "indeterminate" => Ok(Self::indeterminate),

            "in-range" => Ok(Self::in_range),

            "invalid" => Ok(Self::invalid),

            "left" => Ok(Self::left),

            "link" => Ok(Self::link),

            "optional" => Ok(Self::optional),

            "out-of-range" => Ok(Self::out_of_range),

            "placeholder-shown" => Ok(Self::placeholder_shown(Self::applyVendorPrefix(VendorPrefixablePseudoClassName::placeholder_shown, applyVendorPrefixToPseudoClasses))),

            "-moz-placeholder-shown" => Ok(Self::placeholder_shown(Some(moz))),

            // See https://developer.mozilla.org/en-US/docs/Web/CSS/:-moz-placeholder
            "-moz-placeholder" => Ok(Self::placeholder_shown(Self::applyVendorPrefix(VendorPrefixablePseudoClassName::placeholder_shown, applyVendorPrefixToPseudoClasses))),

            "read-only" => Ok(Self::read_only(Self::applyVendorPrefix(VendorPrefixablePseudoClassName::read_only, applyVendorPrefixToPseudoClasses))),

            "-moz-read-only" => Ok(Self::read_only(Some(moz))),

            "read-write" => Ok(Self::read_write(Self::applyVendorPrefix(VendorPrefixablePseudoClassName::read_write, applyVendorPrefixToPseudoClasses))),

            "-moz-read-write" => Ok(Self::read_write(Some(moz))),

            "required" => Ok(Self::required),

            "right" => Ok(Self::right),

            "scope" => Err(ParseError::from(CustomParseError::NonTreeStructuralPseudoClassScopeIsObsoleteAsOfFirefox55)),

            "target" => Ok(Self::target),

            "valid" => Ok(Self::valid),

            "visited" => Ok(Self::visited),


            // -servo-only

            "-servo-non-zero-border" => Ok(Self::non_zero_border(Some(servo))),


            // -moz- only

            "-moz-broken" => Ok(Self::broken(Some(moz))),

            "-moz-drag-over" => Ok(Self::drag_over(Some(moz))),

            "-moz-first-node" => Ok(Self::first_node(Some(moz))),

            "-moz-focusring" => Ok(Self::focusring(Some(moz))),

            "-moz-full-screen-ancestor" => Ok(Self::full_screen_ancestor(Some(moz))),

            "-moz-handler-blocked" => Ok(Self::handler_blocked(Some(moz))),

            "-moz-handler-crashed" => Ok(Self::handler_crashed(Some(moz))),

            "-moz-handler-disabled" => Ok(Self::handler_disabled(Some(moz))),

            "-moz-last-node" => Ok(Self::last_node(Some(moz))),

            "-moz-list-bullet" => Ok(Self::list_bullet(Some(moz))),

            "-moz-list-number" => Ok(Self::list_number(Some(moz))),

            "-moz-loading" => Ok(Self::loading(Some(moz))),

            "-moz-lwtheme" => Ok(Self::lwtheme(Some(moz))),

            "-moz-lwtheme-brighttext" => Ok(Self::lwtheme_brighttext(Some(moz))),

            "-moz-lwtheme-darktext" => Ok(Self::lwtheme_darktext(Some(moz))),

            "-moz-native-anonymous" => Ok(Self::native_anonymous(Some(moz))),

            "-moz-only-whitespace" => Ok(Self::only_whitespace(Some(moz))),

            "-moz-submit-invalid" => Ok(Self::submit_invalid(Some(moz))),

            "-moz-suppressed" => Ok(Self::suppressed(Some(moz))),

            "-moz-tree-cell" => Ok(Self::tree_cell(Some(moz))),

            "-moz-tree-checkbox" => Ok(Self::tree_checkbox(Some(moz))),

            "-moz-tree-column" => Ok(Self::tree_column(Some(moz))),

            "-moz-tree-drop-feedback" => Ok(Self::tree_drop_feedback(Some(moz))),

            "-moz-tree-image" => Ok(Self::tree_image(Some(moz))),

            "-moz-tree-indentation" => Ok(Self::tree_indentation(Some(moz))),

            "-moz-tree-line" => Ok(Self::tree_line(Some(moz))),

            "-moz-tree-progressmeter" => Ok(Self::tree_progressmeter(Some(moz))),

            "-moz-tree-separator" => Ok(Self::tree_separator(Some(moz))),

            "-moz-tree-twisty" => Ok(Self::tree_twisty(Some(moz))),

            "-moz-ui-invalid" => Ok(Self::ui_invalid(Some(moz))),

            "-moz-ui-valid" => Ok(Self::ui_valid(Some(moz))),

            "-moz-user-disabled" => Ok(Self::user_disabled(Some(moz))),

            "-moz-window-inactive" => Ok(Self::window_inactive(Some(moz))),


            // -webkit- only, with potential Mozilla support coming

            "-webkit-autofill" => Ok(Self::autofill(Some(webkit))),

            "-moz-autofill" => Ok(Self::autofill(Some(moz))),


            _ => Err(ParseError::from(CustomParseError::UnsupportedPseudoClassOrElement(name.to_string()))),
        }
    }

    #[inline(always)]
    pub(crate) fn parse_with_arguments<'i, 't>(
        applyVendorPrefixToPseudoClasses: &HashMap<
            VendorPrefixablePseudoClassName,
            VendorPrefix,
        >,
        name: CowRcStr<'i>,
        input: &mut Parser<'i, 't>,
        ourSelectorParser: &OurSelectorParser,
    ) -> Result<Self, ParseError<'i, CustomParseError<'i>>> {
        use self::{NonTreeStructuralPseudoClass::*, VendorPrefix::*};

        match_ignore_ascii_case! {
            &name,

            "any" => Ok(any(Self::applyVendorPrefix(VendorPrefixablePseudoClassName::any, applyVendorPrefixToPseudoClasses), Self::parse_any(input, ourSelectorParser)?)),

            "-moz-any" => Ok(any(Some(moz), Self::parse_any(input, ourSelectorParser)?)),

            "-webkit-any" => Ok(any(Some(webkit), Self::parse_any(input, ourSelectorParser)?)),

            "dir" => Ok(dir(Self::applyVendorPrefix(VendorPrefixablePseudoClassName::dir, applyVendorPrefixToPseudoClasses), Self::parse_text_directionality(input)?)),

            "-moz-dir" => Ok(dir(Some(moz), Self::parse_text_directionality(input)?)),

            "lang" => Ok(lang(Self::parse_lang(input)?)),

            "where" => Ok(Self::where_(ourSelectorParser.parse_internal(input, OurSelectorExt::is_false_if_any_selector_is_simple_and_only_uses_the_descendant_combinator)?)),

            "is" => Ok(Self::is(ourSelectorParser.parse_internal(input, OurSelectorExt::is_false_if_any_selector_is_simple_and_only_uses_the_descendant_combinator)?)),

            // -servo- only

            "-servo-case-sensitive-type-attr" => Ok(case_sensitive_type_attr(Some(servo), Atom::from(input.expect_ident()?))),


            // -moz- only

            "-moz-locale-dir" => Ok(locale_dir(Some(moz), Self::parse_text_directionality(input)?)),

            "-moz-system-metric" => Ok(system_metric(Some(moz), Self::parse_system_metric(input)?)),

            "-moz-tree-cell-text" => Ok(tree_cell_text(Some(moz), Self::parse_tree_hover(input)?)),

            "-moz-tree-row" => Ok(tree_row(Some(moz), Self::parse_tree_hover(input)?)),


            _ => Err(ParseError::from(CustomParseError::UnsupportedPseudoClassOrElement(name.to_string()))),
        }
    }

    #[inline(always)]
    pub(crate) fn parse_any<'i, 't>(
        input: &mut Parser<'i, 't>,
        ourSelectorParser: &OurSelectorParser,
    ) -> Result<DeduplicatedSelectors, ParseError<'i, CustomParseError<'i>>>
    {
        ourSelectorParser
            .parse_internal(
                input,
                OurSelectorExt::is_false_if_any_selector_is_simple_and_only_uses_the_descendant_combinator
            )
    }

    #[inline(always)]
    pub(crate) fn parse_text_directionality<'i, 't>(
        input: &mut Parser<'i, 't>,
    ) -> Result<TextDirectionality, ParseError<'i, CustomParseError<'i>>> {
        TextDirectionality::parse(input)
    }

    #[inline(always)]
    pub(crate) fn parse_system_metric<'i, 't>(
        input: &mut Parser<'i, 't>,
    ) -> Result<SystemMetric, ParseError<'i, CustomParseError<'i>>> {
        SystemMetric::parse(input)
    }

    #[inline(always)]
    pub(crate) fn parse_tree_hover<'i, 't>(
        input: &mut Parser<'i, 't>,
    ) -> Result<TreeHover, ParseError<'i, CustomParseError<'i>>> {
        TreeHover::parse(input)
    }

    #[inline(always)]
    pub(crate) fn parse_lang<'i, 't>(
        input: &mut Parser<'i, 't>,
    ) -> Result<LanguageRanges, ParseError<'i, CustomParseError<'i>>> {
        // the :lang() pseudo-class represents an element that is in one of the languages listed in its argument. It accepts a comma-separated list of one or more language ranges as its argument. Each language range in :lang() must be a valid CSS <ident> or <string>. (Language ranges containing asterisks, for example, must be quoted as strings.)
        let languages = input.parse_comma_separated(|input| {
            Ok(LanguageRange(Atom::from(
                input.expect_ident_or_string()?.as_ref(),
            )))
        })?;
        Ok(LanguageRanges(languages))
        //.map(LanguageRanges)
    }
}
