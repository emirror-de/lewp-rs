// This file is part of css. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of css. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/css/master/COPYRIGHT.

use {
    super::VendorPrefixablePseudoElementName,
    crate::{
        domain::{
            selectors::OurSelectorImpl,
            VendorPrefix::{self, *},
        },
        parsers::OurSelectorParser,
        CustomParseError,
    },
    cssparser::{CowRcStr, ParseError, Parser, ToCss},
    std::{collections::HashMap, fmt},
    PseudoElement::*,
};

//noinspection SpellCheckingInspection
/// A pseudo-element, both public and private.
/// Includes browser-specific pseudo-elements and pseudo-elements which, although named differently, map to near equivalency
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[allow(missing_docs)]
pub enum PseudoElement {
    after,
    backdrop(Option<VendorPrefix>),
    before,
    cue,
    file_selector_button,
    first_letter,
    first_line,
    marker,
    grammar_error,
    placeholder(Option<VendorPrefix>),
    selection(Option<VendorPrefix>),
    spelling_error,

    /// Not standardized but there are near equivalents amongst the various browsers under different names (we use the Mozilla name without the -moz- prefix)
    progress_bar(Option<VendorPrefix>),

    /// Not standardized but there are near equivalents amongst the various browsers under different names (we use the Mozilla name without the -moz- prefix)
    range_progress(Option<VendorPrefix>),

    /// Not standardized but there are near equivalents amongst the various browsers under different names (we use the Mozilla name without the -moz- prefix)
    range_thumb(Option<VendorPrefix>),

    /// Not standardized but there are near equivalents amongst the various browsers under different names (we use the Mozilla name without the -moz- prefix)
    range_track(Option<VendorPrefix>),

    /// -servo- and -moz- only
    anonymous_block(Option<VendorPrefix>),

    /// -servo- only
    details_summary(Option<VendorPrefix>),

    /// -servo- only
    details_content(Option<VendorPrefix>),

    /// -servo- only
    text(Option<VendorPrefix>),

    /// -servo- only
    input_text(Option<VendorPrefix>),

    /// -servo- only
    table_wrapper(Option<VendorPrefix>),

    /// -servo- only
    anonymous_table_wrapper(Option<VendorPrefix>),

    /// -servo- only
    anonymous_table(Option<VendorPrefix>),

    /// -servo- only
    anonymous_table_row(Option<VendorPrefix>),

    /// -servo- only
    anonymous_table_cell(Option<VendorPrefix>),

    /// -servo- only
    inline_block_wrapper(Option<VendorPrefix>),

    /// -servo- only
    inline_absolute(Option<VendorPrefix>),

    /// -ms- only
    browse(Option<VendorPrefix>),

    /// -ms- only
    check(Option<VendorPrefix>),

    /// -ms- only
    clear(Option<VendorPrefix>),

    /// -ms- only
    expand(Option<VendorPrefix>),

    /// -ms- only
    fill_lower(Option<VendorPrefix>),

    /// -ms- only
    reveal(Option<VendorPrefix>),

    /// -ms- only
    value(Option<VendorPrefix>),

    /// -moz- only
    anonymous_positioned_block(Option<VendorPrefix>),

    /// -moz- only
    canvas(Option<VendorPrefix>),

    /// -moz- only
    cell_content(Option<VendorPrefix>),

    /// -moz- only
    focus_inner(Option<VendorPrefix>),

    /// -moz- only
    focus_outer(Option<VendorPrefix>),

    /// -moz- only
    inline_table(Option<VendorPrefix>),

    /// -moz- only
    list_bullet(Option<VendorPrefix>),

    /// -moz- only
    page(Option<VendorPrefix>),

    /// -moz- only
    page_sequence(Option<VendorPrefix>),

    /// -moz- only
    pagebreak(Option<VendorPrefix>),

    /// -moz- only
    pagecontent(Option<VendorPrefix>),

    /// -moz- only
    scrolled_canvas(Option<VendorPrefix>),

    /// -moz- only
    scrolled_content(Option<VendorPrefix>),

    /// -moz- only
    scrolled_page_sequence(Option<VendorPrefix>),

    /// -moz- only
    svg_foreign_content(Option<VendorPrefix>),

    /// -moz- only
    table(Option<VendorPrefix>),

    /// -moz- only
    table_cell(Option<VendorPrefix>),

    /// -moz- only
    table_column(Option<VendorPrefix>),

    /// -moz- only
    table_column_group(Option<VendorPrefix>),

    /// -moz- only
    table_outer(Option<VendorPrefix>),

    /// -moz- only
    table_row(Option<VendorPrefix>),

    /// -moz- only
    table_row_group(Option<VendorPrefix>),

    /// -moz- only
    viewport(Option<VendorPrefix>),

    /// -moz- only
    viewport_scroll(Option<VendorPrefix>),

    /// -moz- only
    xul_anonymous_block(Option<VendorPrefix>),

    // -moz- only (but MDN incorrectly lists them as pseudo-classes)
    tree_cell_text(Option<VendorPrefix>),

    // -moz- only (but MDN incorrectly lists them as pseudo-classes)
    tree_row(Option<VendorPrefix>),

    /// -webkit- only
    file_upload_button(Option<VendorPrefix>),

    /// -webkit- only
    inner_spin_button(Option<VendorPrefix>),

    /// -webkit- only
    meter_bar(Option<VendorPrefix>),

    /// -webkit- only
    meter_even_less_good_value(Option<VendorPrefix>),

    /// -webkit- only
    meter_inner_element(Option<VendorPrefix>),

    /// -webkit- only
    meter_optimum_value(Option<VendorPrefix>),

    /// -webkit- only
    meter_suboptimum_value(Option<VendorPrefix>),

    /// -webkit- only
    outer_spin_button(Option<VendorPrefix>),

    /// -webkit- only
    progress_inner_element(Option<VendorPrefix>),

    /// -webkit- only
    progress_value(Option<VendorPrefix>),

    /// -webkit- only
    search_cancel_button(Option<VendorPrefix>),

    /// -webkit- only
    search_results_button(Option<VendorPrefix>),

    /// -webkit- only (not documented on MDN on 7th November 2017)
    search_decoration(Option<VendorPrefix>),

    /// -webkit- and -moz-
    color_swatch_wrapper(Option<VendorPrefix>),

    /// -webkit- and -moz-
    color_swatch(Option<VendorPrefix>),

    /// -webkit- only
    calendar_picker_indicator(Option<VendorPrefix>),

    /// -webkit- only
    details_marker(Option<VendorPrefix>),
}

impl ToCss for self::PseudoElement {
    //noinspection SpellCheckingInspection
    fn to_css<W: fmt::Write>(&self, dest: &mut W) -> fmt::Result {
        #[inline(always)]
        fn write<W: fmt::Write>(
            dest: &mut W,
            classWithColonColon: &str,
        ) -> fmt::Result {
            dest.write_str(classWithColonColon)
        }

        #[inline(always)]
        fn write_with_vendor_prefix<W: fmt::Write>(
            dest: &mut W,
            vendorPrefix: &Option<VendorPrefix>,
            classWithoutColonColon: &str,
        ) -> fmt::Result {
            dest.write_str("::")?;
            if let &Some(ref vendorPrefix) = vendorPrefix {
                vendorPrefix.to_css(dest)?;
            }
            dest.write_str(classWithoutColonColon)
        }

        match *self {
            after => write(dest, "::after"),

            backdrop(ref vendorPrefix) => {
                write_with_vendor_prefix(dest, vendorPrefix, "backdrop")
            }

            before => write(dest, "::before"),

            cue => write(dest, "::cue"),

            file_selector_button => write(dest, "::file-selector-button"),

            first_letter => write(dest, "::first-letter"),

            first_line => write(dest, "::first-line"),

            grammar_error => write(dest, "::grammar-error"),

            marker => write(dest, "::marker"),

            placeholder(ref vendorPrefix) => match *vendorPrefix {
                Some(webkit) => write(dest, "::-webkit-input-placeholder"),
                Some(ms) => write(dest, "::-ms-input-placeholder"),
                _ => {
                    write_with_vendor_prefix(dest, vendorPrefix, "placeholder")
                }
            },

            selection(ref vendorPrefix) => {
                write_with_vendor_prefix(dest, vendorPrefix, "selection")
            }

            spelling_error => write(dest, "::spelling-error"),

            progress_bar(ref vendorPrefix) => match *vendorPrefix {
                Some(moz) => write(dest, "::-moz-progress-bar"),
                Some(webkit) => write(dest, "::-webkit-progress-bar"),
                Some(ms) => write(dest, "::-ms-fill"),
                _ => {
                    write_with_vendor_prefix(dest, vendorPrefix, "progress-bar")
                } // almost certainly wrong
            },

            range_progress(ref vendorPrefix) => match *vendorPrefix {
                Some(moz) => write(dest, "::-moz-range-progress"),
                Some(ms) => write(dest, "::-ms-fill-upper"),
                _ => write_with_vendor_prefix(
                    dest,
                    vendorPrefix,
                    "range-progress",
                ), // almost certainly wrong
            },

            range_thumb(ref vendorPrefix) => match *vendorPrefix {
                Some(moz) => write(dest, "::-moz-range-thumb"),
                Some(webkit) => write(dest, "::-webkit-slider-thumb"),
                Some(ms) => write(dest, "::-ms-thumb"),
                _ => {
                    write_with_vendor_prefix(dest, vendorPrefix, "range-thumb")
                } // almost certainly wrong
            },

            range_track(ref vendorPrefix) => match *vendorPrefix {
                Some(moz) => write(dest, "::-moz-range-track"),
                Some(webkit) => write(dest, "::-webkit-slider-runnable-track"),
                Some(ms) => write(dest, "::-ms-track"),
                _ => {
                    write_with_vendor_prefix(dest, vendorPrefix, "range-track")
                } // almost certainly wrong
            },

            details_summary(ref vendorPrefix) => {
                write_with_vendor_prefix(dest, vendorPrefix, "details-summary")
            }

            details_content(ref vendorPrefix) => {
                write_with_vendor_prefix(dest, vendorPrefix, "details-content")
            }

            text(ref vendorPrefix) => {
                write_with_vendor_prefix(dest, vendorPrefix, "text")
            }

            input_text(ref vendorPrefix) => {
                write_with_vendor_prefix(dest, vendorPrefix, "input-text")
            }

            table_wrapper(ref vendorPrefix) => {
                write_with_vendor_prefix(dest, vendorPrefix, "table-wrapper")
            }

            anonymous_table_wrapper(ref vendorPrefix) => {
                write_with_vendor_prefix(
                    dest,
                    vendorPrefix,
                    "anonymous-table-wrapper",
                )
            }

            anonymous_table(ref vendorPrefix) => {
                write_with_vendor_prefix(dest, vendorPrefix, "anonymous-table")
            }

            anonymous_table_row(ref vendorPrefix) => write_with_vendor_prefix(
                dest,
                vendorPrefix,
                "anonymous-table-row",
            ),

            anonymous_table_cell(ref vendorPrefix) => write_with_vendor_prefix(
                dest,
                vendorPrefix,
                "anonymous-table-cell",
            ),

            anonymous_block(ref vendorPrefix) => {
                write_with_vendor_prefix(dest, vendorPrefix, "anonymous-block")
            }

            inline_block_wrapper(ref vendorPrefix) => write_with_vendor_prefix(
                dest,
                vendorPrefix,
                "inline-block-wrapper",
            ),

            inline_absolute(ref vendorPrefix) => {
                write_with_vendor_prefix(dest, vendorPrefix, "inline-absolute")
            }

            browse(ref vendorPrefix) => {
                write_with_vendor_prefix(dest, vendorPrefix, "browse")
            }

            check(ref vendorPrefix) => {
                write_with_vendor_prefix(dest, vendorPrefix, "check")
            }

            clear(ref vendorPrefix) => {
                write_with_vendor_prefix(dest, vendorPrefix, "clear")
            }

            expand(ref vendorPrefix) => {
                write_with_vendor_prefix(dest, vendorPrefix, "expand")
            }

            fill_lower(ref vendorPrefix) => {
                write_with_vendor_prefix(dest, vendorPrefix, "fill-lower")
            }

            reveal(ref vendorPrefix) => {
                write_with_vendor_prefix(dest, vendorPrefix, "reveal")
            }

            value(ref vendorPrefix) => {
                write_with_vendor_prefix(dest, vendorPrefix, "value")
            }

            anonymous_positioned_block(ref vendorPrefix) => {
                write_with_vendor_prefix(
                    dest,
                    vendorPrefix,
                    "anonymous-positioned-block",
                )
            }

            canvas(ref vendorPrefix) => {
                write_with_vendor_prefix(dest, vendorPrefix, "canvas")
            }

            cell_content(ref vendorPrefix) => {
                write_with_vendor_prefix(dest, vendorPrefix, "cell-content")
            }

            focus_inner(ref vendorPrefix) => {
                write_with_vendor_prefix(dest, vendorPrefix, "focus-inner")
            }

            focus_outer(ref vendorPrefix) => {
                write_with_vendor_prefix(dest, vendorPrefix, "focus-outer")
            }

            inline_table(ref vendorPrefix) => {
                write_with_vendor_prefix(dest, vendorPrefix, "inline-table")
            }

            list_bullet(ref vendorPrefix) => {
                write_with_vendor_prefix(dest, vendorPrefix, "list-bullet")
            }

            page(ref vendorPrefix) => {
                write_with_vendor_prefix(dest, vendorPrefix, "page")
            }

            page_sequence(ref vendorPrefix) => {
                write_with_vendor_prefix(dest, vendorPrefix, "page-sequence")
            }

            pagebreak(ref vendorPrefix) => {
                write_with_vendor_prefix(dest, vendorPrefix, "pagebreak")
            }

            pagecontent(ref vendorPrefix) => {
                write_with_vendor_prefix(dest, vendorPrefix, "pagecontent")
            }

            scrolled_canvas(ref vendorPrefix) => {
                write_with_vendor_prefix(dest, vendorPrefix, "scrolled-canvas")
            }

            scrolled_content(ref vendorPrefix) => {
                write_with_vendor_prefix(dest, vendorPrefix, "scrolled-content")
            }

            scrolled_page_sequence(ref vendorPrefix) => {
                write_with_vendor_prefix(
                    dest,
                    vendorPrefix,
                    "scrolled-page-sequence",
                )
            }

            svg_foreign_content(ref vendorPrefix) => write_with_vendor_prefix(
                dest,
                vendorPrefix,
                "svg-foreign-content",
            ),

            table(ref vendorPrefix) => {
                write_with_vendor_prefix(dest, vendorPrefix, "table")
            }

            table_cell(ref vendorPrefix) => {
                write_with_vendor_prefix(dest, vendorPrefix, "table-cell")
            }

            table_column(ref vendorPrefix) => {
                write_with_vendor_prefix(dest, vendorPrefix, "table-column")
            }

            table_column_group(ref vendorPrefix) => write_with_vendor_prefix(
                dest,
                vendorPrefix,
                "table-column-group",
            ),

            table_outer(ref vendorPrefix) => {
                write_with_vendor_prefix(dest, vendorPrefix, "table-outer")
            }

            table_row(ref vendorPrefix) => {
                write_with_vendor_prefix(dest, vendorPrefix, "table-row")
            }

            table_row_group(ref vendorPrefix) => {
                write_with_vendor_prefix(dest, vendorPrefix, "table-row-group")
            }

            viewport(ref vendorPrefix) => {
                write_with_vendor_prefix(dest, vendorPrefix, "viewport")
            }

            viewport_scroll(ref vendorPrefix) => {
                write_with_vendor_prefix(dest, vendorPrefix, "viewport-scroll")
            }

            xul_anonymous_block(ref vendorPrefix) => write_with_vendor_prefix(
                dest,
                vendorPrefix,
                "xul-anonymous-block",
            ),

            tree_cell_text(ref vendorPrefix) => {
                write_with_vendor_prefix(dest, vendorPrefix, "tree-cell-text")
            }

            tree_row(ref vendorPrefix) => {
                write_with_vendor_prefix(dest, vendorPrefix, "tree-row")
            }

            file_upload_button(ref vendorPrefix) => write_with_vendor_prefix(
                dest,
                vendorPrefix,
                "file-upload-button",
            ),

            inner_spin_button(ref vendorPrefix) => write_with_vendor_prefix(
                dest,
                vendorPrefix,
                "inner-spin-button",
            ),

            meter_bar(ref vendorPrefix) => {
                write_with_vendor_prefix(dest, vendorPrefix, "meter-bar")
            }

            meter_even_less_good_value(ref vendorPrefix) => {
                write_with_vendor_prefix(
                    dest,
                    vendorPrefix,
                    "meter-even-less-good-value",
                )
            }

            meter_inner_element(ref vendorPrefix) => write_with_vendor_prefix(
                dest,
                vendorPrefix,
                "meter-inner-element",
            ),

            meter_optimum_value(ref vendorPrefix) => write_with_vendor_prefix(
                dest,
                vendorPrefix,
                "meter-optimum-value",
            ),

            meter_suboptimum_value(ref vendorPrefix) => {
                write_with_vendor_prefix(
                    dest,
                    vendorPrefix,
                    "meter-suboptimum-value",
                )
            }

            outer_spin_button(ref vendorPrefix) => write_with_vendor_prefix(
                dest,
                vendorPrefix,
                "outer-spin-button",
            ),

            progress_inner_element(ref vendorPrefix) => {
                write_with_vendor_prefix(
                    dest,
                    vendorPrefix,
                    "progress-inner-element",
                )
            }

            progress_value(ref vendorPrefix) => {
                write_with_vendor_prefix(dest, vendorPrefix, "progress-value")
            }

            search_cancel_button(ref vendorPrefix) => write_with_vendor_prefix(
                dest,
                vendorPrefix,
                "search-cancel-button",
            ),

            search_results_button(ref vendorPrefix) => {
                write_with_vendor_prefix(
                    dest,
                    vendorPrefix,
                    "search-results-button",
                )
            }

            search_decoration(ref vendorPrefix) => write_with_vendor_prefix(
                dest,
                vendorPrefix,
                "search-decoration",
            ),

            color_swatch_wrapper(ref vendorPrefix) => write_with_vendor_prefix(
                dest,
                vendorPrefix,
                "color-swatch-wrapper",
            ),

            color_swatch(ref vendorPrefix) => {
                write_with_vendor_prefix(dest, vendorPrefix, "color-swatch")
            }
            calendar_picker_indicator(ref vendorPrefix) => {
                write_with_vendor_prefix(
                    dest,
                    vendorPrefix,
                    "calendar-picker-indicator",
                )
            }
            details_marker(ref vendorPrefix) => {
                write_with_vendor_prefix(dest, vendorPrefix, "details-marker")
            }
        }
    }
}

impl selectors::parser::PseudoElement for PseudoElement {
    type Impl = OurSelectorImpl;
}

impl PseudoElement {
    /// Whether this pseudo-element supports user action selectors.
    pub fn supports_user_action_state(&self) -> bool {
        match *self {
            after => false,
            before => false,
            backdrop(..) => false,
            cue => false,
            file_selector_button => false,
            first_letter => false,
            first_line => false,
            progress_bar(..) => true,
            range_track(..) => true,
            range_progress(..) => true,
            range_thumb(..) => true,
            placeholder(..) => true,
            _ => false,
        }
    }

    #[inline(always)]
    fn applyVendorPrefix(
        pseudoElementName: VendorPrefixablePseudoElementName,
        applyVendorPrefixToPseudoElements: &HashMap<
            VendorPrefixablePseudoElementName,
            VendorPrefix,
        >,
    ) -> Option<VendorPrefix> {
        applyVendorPrefixToPseudoElements
            .get(&pseudoElementName)
            .cloned()
    }

    //noinspection SpellCheckingInspection
    #[inline(always)]
    pub(crate) fn parse_without_arguments<'i>(
        applyVendorPrefixToPseudoElements: &HashMap<
            VendorPrefixablePseudoElementName,
            VendorPrefix,
        >,
        name: CowRcStr<'i>,
    ) -> Result<Self, ParseError<'i, CustomParseError<'i>>> {
        match_ignore_ascii_case! {
            &name,

            "after" => Ok(after),

            "backdrop" => Ok(backdrop(Self::applyVendorPrefix(VendorPrefixablePseudoElementName::backdrop, applyVendorPrefixToPseudoElements))),

            "-ms-backdrop" => Ok(backdrop(Some(ms))),

            "-webkit-backdrop" => Ok(backdrop(Some(webkit))),

            "before" => Ok(before),

            "cue" => Ok(cue),

            "file-selector-button" => Ok(file_selector_button),

            "first-letter" => Ok(first_letter),

            "first-line" => Ok(first_line),

            "grammar-error" => Ok(grammar_error),

            "marker" => Ok(marker),

            "placeholder" => Ok(placeholder(Self::applyVendorPrefix(VendorPrefixablePseudoElementName::placeholder, applyVendorPrefixToPseudoElements))),

            "-ms-placeholder" => Ok(placeholder(Some(ms))),

            "-webkit-input-placeholder" => Ok(placeholder(Some(ms))),

            "selection" => Ok(selection(Self::applyVendorPrefix(VendorPrefixablePseudoElementName::selection, applyVendorPrefixToPseudoElements))),

            "-moz-selection" => Ok(selection(Some(moz))),

            "spelling-error" => Ok(spelling_error),


            // Nearly the same

            "progress-bar" => Ok(progress_bar(Self::applyVendorPrefix(VendorPrefixablePseudoElementName::progress_bar, applyVendorPrefixToPseudoElements))),

            "-moz-progress-bar" => Ok(progress_bar(Some(moz))),

            "-webkit-progress-bar" => Ok(progress_bar(Some(webkit))),

            "-ms-fill" => Ok(progress_bar(Some(ms))),


            // Nearly the same

            "range-progress" => Ok(range_progress(Self::applyVendorPrefix(VendorPrefixablePseudoElementName::range_progress, applyVendorPrefixToPseudoElements))),

            "-moz-range-progress" => Ok(range_progress(Some(moz))),

            "-ms-fill-upper" => Ok(range_progress(Some(ms))),


            // Nearly the same

            "range-thumb" => Ok(range_thumb(Self::applyVendorPrefix(VendorPrefixablePseudoElementName::range_thumb, applyVendorPrefixToPseudoElements))),

            "-moz-range-thumb" => Ok(range_thumb(Some(moz))),

            "-webkit-slider-thumb" => Ok(range_thumb(Some(webkit))),

            "-ms-thumb" => Ok(range_thumb(Some(ms))),


            // Nearly the same

            "range-track" => Ok(range_track(Self::applyVendorPrefix(VendorPrefixablePseudoElementName::range_track, applyVendorPrefixToPseudoElements))),

            "-moz-range-track" => Ok(range_track(Some(moz))),

            "-webkit-slider-runnable-track" => Ok(range_track(Some(webkit))),

            "-ms-track" => Ok(range_track(Some(ms))),


            // -servo- and -moz- only

            "-servo-anonymous-block" => Ok(anonymous_block(Some(servo))),

            "-moz-anonymous-block" => Ok(anonymous_block(Some(moz))),


            // -servo- only

            "-servo-details-summary" => Ok(details_summary(Some(servo))),

            "-servo-details-content" => Ok(details_content(Some(servo))),

            "-servo-text" => Ok(text(Some(servo))),

            "-servo-input-text" => Ok(input_text(Some(servo))),

            "-servo-table-wrapper" => Ok(table_wrapper(Some(servo))),

            "-servo-anonymous-table-wrapper" => Ok(anonymous_table_wrapper(Some(servo))),

            "-servo-anonymous-table" => Ok(anonymous_table(Some(servo))),

            "-servo-anonymous-table-row" => Ok(anonymous_table_row(Some(servo))),

            "-servo-anonymous-table-cell" => Ok(anonymous_table_cell(Some(servo))),

            "-servo-inline-block-wrapper" => Ok(inline_block_wrapper(Some(servo))),

            "-servo-inline-absolute" => Ok(inline_absolute(Some(servo))),


            // -ms- only

            "-ms-browse" => Ok(browse(Some(ms))),

            "-ms-check" => Ok(check(Some(ms))),

            "-ms-clear" => Ok(clear(Some(ms))),

            "-ms-expand" => Ok(expand(Some(ms))),

            "-ms-fill-lower" => Ok(fill_lower(Some(ms))),

            "-ms-reveal" => Ok(reveal(Some(ms))),

            "-ms-value" => Ok(value(Some(ms))),


            // -moz- only

            "-moz-anonymous-positioned-block" => Ok(anonymous_positioned_block(Some(moz))),

            "-moz-canvas" => Ok(canvas(Some(moz))),

            "-moz-cell-content" => Ok(cell_content(Some(moz))),

            "-moz-focus-inner" => Ok(focus_inner(Some(moz))),

            "-moz-focus-outer" => Ok(focus_outer(Some(moz))),

            "-moz-inline-table" => Ok(inline_table(Some(moz))),

            "-moz-list-bullet" => Ok(list_bullet(Some(moz))),

            "-moz-page" => Ok(page(Some(moz))),

            "-moz-page-sequence" => Ok(page_sequence(Some(moz))),

            "-moz-pagebreak" => Ok(pagebreak(Some(moz))),

            "-moz-pagecontent" => Ok(pagecontent(Some(moz))),

            "-moz-scrolled-canvas" => Ok(scrolled_canvas(Some(moz))),

            "-moz-scrolled-content" => Ok(scrolled_content(Some(moz))),

            "-moz-scrolled-page-sequence" => Ok(scrolled_page_sequence(Some(moz))),

            "-moz-svg-foreign-content" => Ok(svg_foreign_content(Some(moz))),

            "-moz-table" => Ok(table(Some(moz))),

            "-moz-table-cell" => Ok(table_cell(Some(moz))),

            "-moz-table-column" => Ok(table_column(Some(moz))),

            "-moz-table-column-group" => Ok(table_column_group(Some(moz))),

            "-moz-table-outer" => Ok(table_outer(Some(moz))),

            "-moz-table-row" => Ok(table_row(Some(moz))),

            "-moz-table-row-group" => Ok(table_row_group(Some(moz))),

            "-moz-viewport" => Ok(viewport(Some(moz))),

            "-moz-viewport-scroll" => Ok(viewport_scroll(Some(moz))),

            "-moz-xul-anonymous-block" => Ok(xul_anonymous_block(Some(moz))),


            // -moz- only (but MDN incorrectly lists them as pseudo-classes)

            "-moz-tree-cell-text" => Ok(tree_cell_text(Some(moz))),

            "-moz-tree-row" => Ok(tree_row(Some(moz))),

            "-moz-color-swatch" => Ok(color_swatch(Some(moz))),


            // -webkit- only

            "-webkit-calendar-picker-indicator" => Ok(calendar_picker_indicator(Some(webkit))),

            "-webkit-color-swatch-wrapper" => Ok(color_swatch_wrapper(Some(webkit))),

            "-webkit-color-swatch" => Ok(color_swatch(Some(webkit))),

            "-webkit-details-marker" => Ok(details_marker(Some(webkit))),

            "-webkit-file-upload-button" => Ok(file_upload_button(Some(webkit))),

            "-webkit-inner-spin-button" => Ok(inner_spin_button(Some(webkit))),

            "-webkit-meter-bar" => Ok(meter_bar(Some(webkit))),

            "-webkit-meter-even-less-good-value" => Ok(meter_even_less_good_value(Some(webkit))),

            "-webkit-meter-inner-element" => Ok(meter_inner_element(Some(webkit))),

            "-webkit-meter-optimum-value" => Ok(meter_optimum_value(Some(webkit))),

            "-webkit-meter-suboptimum-value" => Ok(meter_suboptimum_value(Some(webkit))),

            "-webkit-outer-spin-button" => Ok(outer_spin_button(Some(webkit))),

            "-webkit-progress-inner-element" => Ok(progress_inner_element(Some(webkit))),

            "-webkit-progress-value" => Ok(progress_value(Some(webkit))),

            "-webkit-search-cancel-button" => Ok(search_cancel_button(Some(webkit))),

            "-webkit-search-results-button" => Ok(search_results_button(Some(webkit))),

            "-webkit-search-decoration" => Ok(search_decoration(Some(webkit))),


            _ => Err(ParseError::from(CustomParseError::UnsupportedPseudoClassOrElement(name.to_string()))),
        }
    }

    #[inline(always)]
    pub(crate) fn parse_with_arguments<'i, 't>(
        _applyVendorPrefixToPseudoElements: &HashMap<
            VendorPrefixablePseudoElementName,
            VendorPrefix,
        >,
        name: CowRcStr<'i>,
        _arguments: &mut Parser<'i, 't>,
        _ourSelectorParser: &OurSelectorParser,
    ) -> Result<Self, ParseError<'i, CustomParseError<'i>>> {
        Err(ParseError::from(
            CustomParseError::UnsupportedPseudoClassOrElement(name.to_string()),
        ))
    }
}
