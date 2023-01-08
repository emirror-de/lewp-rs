//! Traits and data structures to create, run, assemble and render a web page.

use {
    crate::{
        css::{Entireness, Register as CssRegister},
        fh::{ComponentInformation, ComponentType, FileHierarchy, Level},
        html::{
            api::{
                body,
                charset,
                description,
                document,
                head,
                style,
                text,
                title,
                viewport,
            },
            Node,
            NodeList,
        },
        view::PageView,
        Charset,
        LanguageTag,
    },
    html5ever::serialize,
    markup5ever_rcdom::SerializableHandle,
    state::*,
    std::sync::Arc,
};

mod state;

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
    /// Adds a viewport node to the head of the page. Set to `None` if you want
    /// to disable.
    fn viewport(&self) -> Option<Node> {
        Some(viewport())
    }
    /// Creates a new [PageWrapper] which is able to attach different
    /// resources and rendering the page.
    fn new(
        model: Self,
    ) -> PageWrapper<Self, WithoutFileHierarchy, WithoutCss, PagePreparing>
    {
        PageWrapper::from(model)
    }
}

/// A wrapper around the implemented [Page] trait. Contains all necessary code
/// to execute the behavior and assemble the view of your page.
pub struct PageWrapper<P: Page, FH: FhState, CSS: CssState, E: ExecutionState> {
    model: P,
    view: PageView,
    fh: Option<Arc<FileHierarchy>>,
    css_register: Option<Arc<CssRegister>>,
    fs_state: std::marker::PhantomData<FH>,
    css_state: std::marker::PhantomData<CSS>,
    execution_state: std::marker::PhantomData<E>,
}

impl<P: Page> PageWrapper<P, WithoutFileHierarchy, WithoutCss, PagePreparing> {
    /// Attaches the given [FileHierarchy].
    pub fn with_file_hierarchy(
        self,
        fh: Arc<FileHierarchy>,
    ) -> PageWrapper<P, WithFileHierarchy, WithoutCss, PagePreparing> {
        PageWrapper {
            model: self.model,
            view: self.view,
            fh: Some(fh),
            css_register: self.css_register,
            fs_state: std::marker::PhantomData,
            css_state: std::marker::PhantomData,
            execution_state: std::marker::PhantomData,
        }
    }
}

impl<P: Page, CSS: CssState>
    PageWrapper<P, WithoutFileHierarchy, CSS, PagePreparing>
{
    /// Returns the attached [FileHierarchy].
    pub fn file_hierarchy(&self) -> Arc<FileHierarchy> {
        // state enforces a Some to file_hierarchy
        Arc::clone(self.fh.as_ref().unwrap())
    }
}

impl<P: Page> PageWrapper<P, WithFileHierarchy, WithoutCss, PagePreparing> {
    /// Creates a new [CssRegister] instance with the given [CSSRegisterOptions]
    /// and attaches it to the page. For this, the previously given file hierarchy
    /// is used.
    pub fn with_css_register(
        self,
        register: Arc<CssRegister>,
    ) -> PageWrapper<P, WithFileHierarchy, WithoutCss, PagePreparing> {
        PageWrapper {
            model: self.model,
            view: self.view,
            fh: self.fh,
            css_register: Some(register),
            fs_state: std::marker::PhantomData,
            css_state: std::marker::PhantomData,
            execution_state: std::marker::PhantomData,
        }
    }
}

impl<P: Page, FH: FhState> PageWrapper<P, FH, WithCss, PagePreparing> {
    /// Returns the attached [CssRegister].
    pub fn css_register(&self) -> Arc<CssRegister> {
        // state enforces a Some to css_register
        Arc::clone(self.css_register.as_ref().unwrap())
    }
}

impl<P: Page, FH: FhState, CSS: CssState>
    PageWrapper<P, FH, CSS, PagePreparing>
{
    /// This is your main entry point to processing your implemented page.
    pub fn main(mut self) -> PageWrapper<P, FH, CSS, PageFinished> {
        self.model.main(&mut self.view);

        PageWrapper {
            model: self.model,
            view: self.view,
            fh: self.fh,
            css_register: self.css_register,
            fs_state: std::marker::PhantomData,
            css_state: std::marker::PhantomData,
            execution_state: std::marker::PhantomData,
        }
    }
}

impl<P: Page, FH: FhState, CSS: CssState>
    PageWrapper<P, FH, CSS, PageFinished>
{
    /// Renders the page to valid `HTML5` code.
    pub fn render(self) -> String {
        log::debug!(
            "Full dependency list on rendering:\n{}",
            self.view.dependency_list()
        );

        let document = document(
            self.model.language(),
            head(self.assemble_head()),
            body(self.view.body()),
        );

        let mut bytes = vec![];
        let document: SerializableHandle = document.document.into();
        serialize(&mut bytes, &document, Default::default()).unwrap();
        String::from_utf8(bytes).unwrap()
    }

    /// This method collects all nodes that belong to the head node.
    fn assemble_head(&self) -> NodeList {
        let mut head = NodeList::new();

        let mut prelude = vec![
            charset(&self.model.charset()),
            title(&self.model.title()),
            description(&self.model.description()),
        ];

        // add viewport if available
        if let Some(v) = self.model.viewport() {
            prelude.push(v);
        }

        head.append(&mut prelude);

        match self.assemble_page_css() {
            Some(css) => head.push(css),
            None => (),
        };

        match self.assemble_component_css() {
            Some(css) => head.push(css),
            None => (),
        };

        head.append(&mut self.view.head());

        head
    }

    fn assemble_page_css(&self) -> Option<Node> {
        let comp_css = Arc::new(ComponentInformation {
            id: self.model.id(),
            level: Level::Page,
            kind: ComponentType::Css,
        });

        match &self.css_register {
            Some(r) => {
                match r.query(comp_css, Entireness::Full) {
                    Some(css) => {
                        let css = format!("{}", css);
                        // create a style tag for inline css
                        if css.is_empty() {
                            return None;
                        } else {
                            return Some(style(text(&css)));
                        }
                    }
                    None => None,
                }
            }
            None => None,
        }
    }

    fn assemble_component_css(&self) -> Option<Node> {
        let css_register = match &self.css_register {
            Some(r) => r,
            None => return None,
        };
        let mut collected_css = vec![];
        for component in self.view.dependency_list().list() {
            if let Some(css) = css_register.query(
                Arc::new(ComponentInformation {
                    id: component.into(),
                    level: Level::Component,
                    kind: ComponentType::Css,
                }),
                Entireness::Full,
            ) {
                collected_css.push(css.clone());
            };
        }

        let collected_css = &collected_css
            .into_iter()
            .fold(String::new(), |acc, e| format!("{}{}", acc, *e));

        if collected_css.is_empty() {
            return None;
        }
        Some(style(text(collected_css)))
    }
}

impl<P: Page> From<P>
    for PageWrapper<P, WithoutFileHierarchy, WithoutCss, PagePreparing>
{
    fn from(model: P) -> Self {
        Self {
            model,
            view: PageView::default(),
            fh: None,
            css_register: None,
            fs_state: std::marker::PhantomData,
            css_state: std::marker::PhantomData,
            execution_state: std::marker::PhantomData,
        }
    }
}
