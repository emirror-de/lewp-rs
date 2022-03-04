/// Parameter for the script element.
pub enum Script<'a> {
    /// Represents a script pulled by the src attribute.
    Src(&'a str),
    /// Represents an inline script.
    Inline(&'a str),
}
