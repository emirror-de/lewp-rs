//! Contains the classification of different CSS properties, for example if they are render critical or not.

use lewp_css::domain::properties::{HasImportance, PropertyDeclaration};

/// Methods used to classify a CSS property.
pub trait PropertyClassification {
    /// True if the property is critical for rendering. Most simple example is
    /// the `display` property.
    fn is_render_critical(&self) -> bool;
}

impl<I: HasImportance> PropertyClassification for PropertyDeclaration<I> {
    fn is_render_critical(&self) -> bool {
        CSS_PROPERTY_RENDER_CRITICAL
            .iter()
            .any(|e| self.name.starts_with(e))
    }
}

const CSS_PROPERTY_RENDER_CRITICAL: &[&str] = &[
    "--",
    "height",
    "max-height",
    "max-width",
    "min-height",
    "min-width",
    "width",
    "align",
    "flex",
    "font",
    "justify-",
    "order",
    "margin",
    "column",
    "columns",
    "padding",
    "caption-side",
    "empty-cells",
    "table-layout",
    "direction",
    "display",
    "position",
    "top",
    "right",
    "bottom",
    "left",
    "float",
    "clear",
    "z-index",
    "overflow",
    "resize",
    "clip",
    "visibility",
    "box-sizing",
    "grid",
    "pointer-events",
    "writing-mode",
    "background",
];
