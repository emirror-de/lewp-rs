use crate::{config::PageConfig, Charset, LanguageTag};

/// Contains details about the web page such as title, description and language.
pub trait Metadata {
    /// Title of the page.
    fn title(&self) -> &str;

    /// The page description.
    fn description(&self) -> &str;

    /// Language of the page.
    fn language(&self) -> LanguageTag;

    /// Contains the charset of the HTML page.
    fn charset(&self) -> Charset;

    /// Returns a reference to the page config.
    fn config(&self) -> &PageConfig;
}
