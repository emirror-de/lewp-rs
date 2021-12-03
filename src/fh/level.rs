/// The file hierarchy level.
#[derive(Debug, Clone)]
pub enum Level {
    /// The module level.
    Module,
    /// The page level.
    Page,
}

impl std::fmt::Display for Level {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> Result<(), std::fmt::Error> {
        use Level::*;
        let s = match self {
            Module => "modules",
            Page => "pages",
        };
        write!(f, "{}", s)
    }
}
