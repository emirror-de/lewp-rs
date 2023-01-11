//! Traits and data structures to create, run, assemble and render a web page.

use {
    crate::{
        css::{
            Entireness,
            Register as CssRegister,
            RegisterOptions as CssRegisterOptions,
        },
        fh::{ComponentInformation, ComponentType, FileHierarchy, Level},
        html::{
            api::{
                body,
                charset,
                description,
                document,
                head,
                script,
                style,
                text,
                title,
                viewport,
            },
            Node,
            NodeExt,
            NodeList,
            Script,
        },
        js::{Register as JsRegister, RegisterOptions as JsRegisterOptions},
        view::PageView,
        Charset,
        LanguageTag,
        LewpError,
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
    /// Prepends the returned [NodeList] to the `<head>` of the page.
    fn head(&self) -> NodeList {
        vec![]
    }
    /// Creates a new [PageWrapper] which is able to attach different
    /// resources and rendering the page.
    fn new(
        model: Self,
    ) -> PageWrapper<
        Self,
        WithoutFileHierarchy,
        WithoutCss,
        WithoutJs,
        PagePreparing,
    > {
        PageWrapper::from(model)
    }
}

/// A wrapper around the implemented [Page] trait. Contains all necessary code
/// to execute the behavior and assemble the view of your page.
pub struct PageWrapper<
    P: Page,
    FH: FhState,
    CSS: CssState,
    JS: JsState,
    E: ExecutionState,
> {
    model: P,
    view: PageView,
    fh: Option<Arc<FileHierarchy>>,
    css_register: Option<Arc<CssRegister>>,
    js_register: Option<Arc<JsRegister>>,
    fs_state: std::marker::PhantomData<FH>,
    css_state: std::marker::PhantomData<CSS>,
    js_state: std::marker::PhantomData<JS>,
    execution_state: std::marker::PhantomData<E>,
}

impl<P: Page>
    PageWrapper<P, WithoutFileHierarchy, WithoutCss, WithoutJs, PagePreparing>
{
    /// Attaches the given [FileHierarchy].
    pub fn with_file_hierarchy(
        self,
        fh: Arc<FileHierarchy>,
    ) -> PageWrapper<P, WithFileHierarchy, WithoutCss, WithoutJs, PagePreparing>
    {
        PageWrapper {
            model: self.model,
            view: self.view,
            fh: Some(fh),
            css_register: self.css_register,
            js_register: self.js_register,
            fs_state: std::marker::PhantomData,
            css_state: std::marker::PhantomData,
            js_state: std::marker::PhantomData,
            execution_state: std::marker::PhantomData,
        }
    }
}

impl<P: Page, CSS: CssState, JS: JsState>
    PageWrapper<P, WithoutFileHierarchy, CSS, JS, PagePreparing>
{
    /// Returns the attached [FileHierarchy].
    pub fn file_hierarchy(&self) -> Arc<FileHierarchy> {
        // state enforces a Some to file_hierarchy
        Arc::clone(self.fh.as_ref().unwrap())
    }
}

impl<P: Page, JS: JsState>
    PageWrapper<P, WithFileHierarchy, WithoutCss, JS, PagePreparing>
{
    /// Creates a new [CssRegister] instance with the given [CSSRegisterOptions]
    /// and attaches it to the page. For this, the previously given file hierarchy
    /// is used.
    pub fn with_css_register(
        self,
        options: CssRegisterOptions,
    ) -> Result<
        PageWrapper<P, WithFileHierarchy, WithCss, JS, PagePreparing>,
        LewpError,
    > {
        let register =
            CssRegister::new(Arc::clone(&self.fh.as_ref().unwrap()), options)?;
        Ok(PageWrapper {
            model: self.model,
            view: self.view,
            fh: self.fh,
            css_register: Some(Arc::new(register)),
            js_register: self.js_register,
            fs_state: std::marker::PhantomData,
            css_state: std::marker::PhantomData,
            js_state: std::marker::PhantomData,
            execution_state: std::marker::PhantomData,
        })
    }
}

impl<P: Page, CSS: CssState>
    PageWrapper<P, WithFileHierarchy, CSS, WithoutJs, PagePreparing>
{
    /// Creates a new [JsRegister] instance with the given [JSRegisterOptions]
    /// and attaches it to the page. For this, the previously given file hierarchy
    /// is used.
    pub fn with_js_register(
        self,
        options: JsRegisterOptions,
    ) -> Result<
        PageWrapper<P, WithFileHierarchy, CSS, WithJs, PagePreparing>,
        LewpError,
    > {
        let register =
            JsRegister::new(Arc::clone(&self.fh.as_ref().unwrap()), options)?;
        Ok(PageWrapper {
            model: self.model,
            view: self.view,
            fh: self.fh,
            css_register: self.css_register,
            js_register: Some(Arc::new(register)),
            fs_state: std::marker::PhantomData,
            css_state: std::marker::PhantomData,
            js_state: std::marker::PhantomData,
            execution_state: std::marker::PhantomData,
        })
    }
}

impl<P: Page, FH: FhState, JS: JsState>
    PageWrapper<P, FH, WithCss, JS, PagePreparing>
{
    /// Returns the attached [CssRegister].
    pub fn css_register(&self) -> Arc<CssRegister> {
        // state enforces a Some to css_register
        Arc::clone(self.css_register.as_ref().unwrap())
    }
}

impl<P: Page, FH: FhState, CSS: CssState>
    PageWrapper<P, FH, CSS, WithJs, PagePreparing>
{
    /// Returns the attached [JsRegister].
    pub fn js_register(&self) -> Arc<JsRegister> {
        // state enforces a Some to js_register
        Arc::clone(self.js_register.as_ref().unwrap())
    }
}

impl<P: Page, FH: FhState, CSS: CssState, JS: JsState>
    PageWrapper<P, FH, CSS, JS, PagePreparing>
{
    /// This is your main entry point to processing your implemented page.
    pub fn main(mut self) -> PageWrapper<P, FH, CSS, JS, PageFinished> {
        self.model.main(&mut self.view);

        PageWrapper {
            model: self.model,
            view: self.view,
            fh: self.fh,
            css_register: self.css_register,
            js_register: self.js_register,
            fs_state: std::marker::PhantomData,
            css_state: std::marker::PhantomData,
            js_state: std::marker::PhantomData,
            execution_state: std::marker::PhantomData,
        }
    }
}

impl<P: Page, FH: FhState, CSS: CssState, JS: JsState>
    PageWrapper<P, FH, CSS, JS, PageFinished>
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
        let mut page_head = self.model.head();
        prelude.append(&mut page_head);

        // add viewport if available
        if let Some(v) = self.model.viewport() {
            prelude.push(v);
        }

        head.append(&mut prelude);

        let mut inline_css = match self.get_page_css() {
            Some(css) => format!("{css}"),
            None => "".into(),
        };
        if let Some(css) = self.get_component_css() {
            inline_css += &css;
        };
        if !inline_css.is_empty() {
            log::debug!("Adding inline <style> element with page and all components to <head>");
            head.push(style(text(&inline_css)));
        }

        for c in self.get_component_js() {
            let script = script(Script::Inline(&c));
            script.borrow_attrs(vec![("type", "module"), ("defer", "defer")]);
            head.push(script);
        }

        head.append(&mut self.view.head());

        head
    }

    fn get_page_css(&self) -> Option<Arc<String>> {
        let comp_css = Arc::new(ComponentInformation {
            id: self.model.id(),
            level: Level::Page,
            kind: ComponentType::Css,
        });

        match &self.css_register {
            Some(r) => r.query(comp_css, Entireness::Full),
            None => None,
        }
    }

    fn get_component_css(&self) -> Option<String> {
        let css_register = match &self.css_register {
            Some(r) => r,
            None => return None,
        };
        let mut collected_css = String::new();
        for component in self.view.dependency_list().list() {
            if let Some(css) = css_register.query(
                Arc::new(ComponentInformation {
                    id: component.into(),
                    level: Level::Component,
                    kind: ComponentType::Css,
                }),
                Entireness::Full,
            ) {
                collected_css += &css;
            };
        }

        if collected_css.is_empty() {
            return None;
        }
        Some(collected_css)
    }

    fn get_component_js(&self) -> Vec<String> {
        let mut collected_js = vec![];
        let js_register = match &self.js_register {
            Some(r) => r,
            None => return collected_js,
        };
        for component in self.view.dependency_list().list() {
            if let Some(js) =
                js_register.query(Arc::new(ComponentInformation {
                    id: component.into(),
                    level: Level::Component,
                    kind: ComponentType::JavaScript,
                }))
            {
                collected_js.push((*js).clone());
            };
        }

        collected_js
    }
}

impl<P: Page> From<P>
    for PageWrapper<
        P,
        WithoutFileHierarchy,
        WithoutCss,
        WithoutJs,
        PagePreparing,
    >
{
    fn from(model: P) -> Self {
        Self {
            model,
            view: PageView::default(),
            fh: None,
            css_register: None,
            js_register: None,
            fs_state: std::marker::PhantomData,
            css_state: std::marker::PhantomData,
            js_state: std::marker::PhantomData,
            execution_state: std::marker::PhantomData,
        }
    }
}
