/// The different storage level available.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Level {
    /// The [Component](crate::component::Component) level.
    Component,
    /// The [Page](crate::page::Page) level.
    Page,
}

impl std::fmt::Display for Level {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> Result<(), std::fmt::Error> {
        use Level::*;
        let s = match self {
            Component => "components",
            Page => "pages",
        };
        write!(f, "{s}")
    }
}
