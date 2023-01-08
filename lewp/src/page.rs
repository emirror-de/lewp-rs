//! Traits and data structures to create, run, assemble and render a web page.

use {
    crate::{
        html::api::{body, document, head},
        view::PageView,
        Charset,
        LanguageTag,
    },
    html5ever::serialize,
    markup5ever_rcdom::SerializableHandle,
};

/// Defines the unique page ID. This ID is used to identify eg. the page resources
/// on the file system.
pub type PageId = String;

/// Defines your web page.
pub trait Page
where
    Self: Sized,
{
    /// Implements the main behavior of the page. This method is mainly used for
    /// creating necessary components and adding them to the [PageView].
    /// The added components are automatically executed when added to the [PageView].
    fn main(&self, view: &mut PageView);
    /// Returns a reference to the page ID.
    fn id(&self) -> PageId;
    /// Title of the page. Will land in the `title` tag.
    fn title(&self) -> String {
        "My first page created with lewp-rs".into()
    }
    /// The page description. Will land in a `meta` tag in the `head` of a page.
    fn description(&self) -> String {
        "This page has been created using lewp-rs".into()
    }
    /// Language of the page. Will be added as the `lang` attribute to the `doctype`.
    fn language(&self) -> LanguageTag {
        LanguageTag::parse("en-US").unwrap()
    }
    /// Contains the charset of the HTML page. Will be added to the `head` of the page.
    fn charset(&self) -> Charset {
        Charset::Utf8
    }
    /// Renders the given page model.
    fn render(model: Self) -> String {
        let mut page: PageWrapper<Self> = model.into();
        page.main();
        page.render()
    }
}

/// A wrapper around the implemented [Page] trait. Contains all necessary code
/// to execute the behavior and assemble the view of your page.
pub struct PageWrapper<P: Page> {
    model: P,
    view: PageView,
}

impl<P: Page> PageWrapper<P> {
    /// This is your main entry point to processing your implemented page.
    pub fn main(&mut self) {
        self.model.main(&mut self.view);
    }

    /// Renders the page to valid `HTML5` code.
    pub fn render(self) -> String {
        log::debug!(
            "Full dependency list on rendering:\n{}",
            self.view.dependency_list()
        );

        let document = document(
            self.model.language(),
            head(vec![]),
            body(self.view.body()),
        );

        let mut bytes = vec![];
        let document: SerializableHandle = document.document.into();
        serialize(&mut bytes, &document, Default::default()).unwrap();
        String::from_utf8(bytes).unwrap()
    }
}

impl<P: Page> From<P> for PageWrapper<P> {
    fn from(model: P) -> Self {
        Self {
            model,
            view: PageView::default(),
        }
    }
}
