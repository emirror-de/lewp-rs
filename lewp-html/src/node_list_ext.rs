use crate::{Document, DocumentExt, NodeList};

/// Useful functions and methods for working with a [NodeList].
pub trait NodeListExt
where
    Self: Sized,
{
    /// Parses the given HTML string and returns as [NodeList].
    fn from_string(s: String) -> Result<NodeList, std::io::Error> {
        let s = Document::from_string(s)?;
        Ok(s.document.children.take())
    }
}
