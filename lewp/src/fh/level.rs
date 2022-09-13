/// The file hierarchy level.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Level {
    /// The core level, used for internal purposes only, for example when an
    /// error is thrown by lewp core.
    Core,
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
            Core => "core",
            Module => "modules",
            Page => "pages",
        };
        write!(f, "{}", s)
    }
}
