/// A browsing context is an environment in which Document objects are presented to the user.
#[derive(Debug)]
pub enum BrowsingContext {
    /// Represents an empty browsing context.
    Empty,
    /// Represents a "_blank" browsing context.
    Blank,
    /// Represents a "_self" browsing context.
    Self_,
    /// Represents a "_parent" browsing context.
    Parent,
    /// Represents a "_top" browsing context.
    Top,
}

impl std::fmt::Display for BrowsingContext {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> Result<(), std::fmt::Error> {
        let s = match self {
            Self::Empty => String::from(""),
            Self::Self_ => String::from("Self"),
            _ => format!("{:?}", self),
        };
        write!(f, "_{}", s.to_lowercase())
    }
}
