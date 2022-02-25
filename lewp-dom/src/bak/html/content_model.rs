use super::ContentCategory;

/// Wrapper for the different content model possibilities.
pub enum ContentModel {
    /// The content model allows the given [ContentCategory].
    Category(ContentCategory),
    /// Defines a criteria of the content model by a closure. If returning true, the node is
    /// allowed.
    Specific(bool),
}
