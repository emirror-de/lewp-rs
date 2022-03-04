//! Easy handling of an HTML document.

/// Methods for easy handling of an HTML document.
pub trait DocumentExt {
    /// Converts the given node to an HTML string.
    fn into_html(self) -> String;
}
