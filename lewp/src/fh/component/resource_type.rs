/// The possible resource types that can be stored or retrieved from the file
/// hierarchy.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ResourceType {
    /// A text file.
    Text,
}
