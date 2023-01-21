//! Traits and data structures to create, run, assemble and render a web page.
//!
//! # Hello World! example
//! ```rust
//! # use lewp::{
//! #     component::{Component, ComponentId},
//! #     html::{
//! #         api::{h1, text},
//! #         Node,
//! #     },
//! #     page::{PageModel, PageId, Page},
//! #     view::PageView,
//! # };
//! #
//! # // Your hello world component.
//! # struct HelloWorld {
//! #     data: String,
//! # }
//! #
//! # impl HelloWorld {
//! #     pub fn new() -> Self {
//! #         Self {
//! #             data: String::from("Hello World!"),
//! #         }
//! #     }
//! # }
//! #
//! # // Implement the [Component] trait to define the behavior and view.
//! # impl Component for HelloWorld {
//! #     // No message required for a simple component.
//! #     type Message = ();
//! #
//! #     // The unique ID of your component is used to identify and process further
//! #     // resources, as well as isolation in the world of JavaScript on client side.
//! #     // It is best practice to use the lowercase kebab-case of your structs name
//! #     // to have a clear identification of the components resources in the file
//! #     // hierarchy and your code base.
//! #     fn id(&self) -> ComponentId {
//! #         "hello-world".into()
//! #     }
//! #
//! #     // There is no reason for your page to fail. Errors during processing should
//! #     // result in a different view that you define below.
//! #     fn main(&mut self) {}
//! #
//! #     // This is the view of your component.
//! #     fn view(&self) -> Option<Node> {
//! #         Some(h1(vec![text(&self.data)]))
//! #     }
//! # }
//! #
//! // Define your page. This simple example page uses the example from the component
//! // module documentation.
//! struct HelloWorldPage;
//!
//! impl PageModel for HelloWorldPage {
//!     // Throughout your site, the page id should be unique for the same reason as
//!     // the component id. Use lower kebab-case here as convention.
//!     fn id(&self) -> PageId {
//!         "hello-world-page".into()
//!     }
//!
//!     // The main method of the page. In here you can add your components to the
//!     // page and do whatever processing is required for your page to be rendered.
//!     fn main(&self, view: &mut PageView) {
//!         let mut comp = Component::new(HelloWorld::new());
//!         // The component is only borrowed, to enable the possibility of adding
//!         // it twice to your page. You can use the state of your component to
//!         // define the behavior when adding it multiple times.
//!         // However, the required head nodes for example CSS and JS is being added
//!         // only once, so you can be sure that there is no overhead when adding
//!         // the component multiple times.
//!         view.push(&mut comp);
//!     }
//! }
//!
//! fn main() {
//!     simple_logger::init().unwrap();
//!
//!     // Create an instance of your page
//!     let page = Page::from(HelloWorldPage {});
//!
//!     // You have full control when you want to run and render your page.
//!     // Because the internal state of the page changes when running the main
//!     // method, you need to get the result in order to be able to render the
//!     // resulting page.
//!     let executed_page = page.main();
//!
//!     println!("{}", executed_page.render());
//! }
//! ```

use {
    crate::{
        archive::ArchiveCache,
        component::{ComponentDetails, ComponentId},
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
        lewp_archive,
        resources::{Css, Js, Resource, ResourceLevel, ResourceType},
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

lewp_archive!(LewpJavaScript, "js");

/// Defines the unique page ID. This ID is used to identify eg. the page resources
/// on the file system.
pub type PageId = String;

/// Defines your web page model with sane defaults.
pub trait PageModel
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
    /*
    /// Creates a new [Page] which is able to attach different
    /// resources and rendering the page.
    fn new(
        model: Self,
    ) -> Page<Self, WithoutCss, WithoutJs, PagePreparing, CSTO, JSTO> {
        Page::from(model)
    }
    */
}

/// A wrapper around the implemented [Page] trait. Contains all necessary code
/// to execute the behavior and assemble the view of your page. Please do always
/// use [Page::new] for creating an instance.
pub struct Page<P: PageModel, E: ExecutionState> {
    model: P,
    view: PageView,
    archive_cache: Option<Arc<ArchiveCache>>,
    execution_state: std::marker::PhantomData<E>,
}

impl<P: PageModel> Page<P, PagePreparing> {
    /// Attaches the given [ArchiveCache] instance to the page.
    pub fn with_archive_cache(
        self,
        archive_cache: Arc<ArchiveCache>,
    ) -> Page<P, PagePreparing> {
        Page {
            model: self.model,
            view: self.view,
            archive_cache: Some(archive_cache),
            execution_state: std::marker::PhantomData,
        }
    }
}

impl<P: PageModel> Page<P, PagePreparing> {
    /// This is your main entry point to processing your implemented page.
    pub fn main(mut self) -> Page<P, PageFinished> {
        self.model.main(&mut self.view);

        Page {
            model: self.model,
            view: self.view,
            archive_cache: self.archive_cache,
            execution_state: std::marker::PhantomData,
        }
    }
}

impl<P: PageModel> Page<P, PageFinished> {
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
            None => {
                log::debug!("No page CSS has been found!");
                "".into()
            }
        };
        if let Some(css) = self.get_component_css() {
            inline_css += &css;
        };
        if !inline_css.is_empty() {
            log::debug!("Adding inline <style> element with page and all components to <head>");
            head.push(style(text(&inline_css)));
        }

        for (id, c) in self.get_component_js() {
            let script = script(Script::Inline(&c)).attrs(vec![
                ("type", "module"),
                //("defer", "defer"), // can only be used when not inlined
                ("data-lewp-id", &id),
                ("data-lewp-type", "component"),
            ]);
            head.push(script);
        }

        // add lewp javascript code
        match LewpJavaScript::get("lewp.js") {
            None => log::error!(
                "ALERT!! Could not get lewp.js! This should never occur!"
            ),
            Some(js) => match String::from_utf8(js.data.to_vec()) {
                Ok(s) => head.push(script(Script::Inline(&s))),
                Err(e) => log::error!(
                    "ALERT!! Converting lewp.js to UTF8 failed: {e}"
                ),
            },
        };

        head.append(&mut self.view.head());

        head
    }

    fn get_page_css(&self) -> Option<Arc<String>> {
        match self.archive_cache.as_ref() {
            Some(a) => {
                let details = ComponentDetails::new(
                    self.model.id(),
                    ResourceType::Css,
                    ResourceLevel::Page,
                );
                a.query(&details)
                    .map(|c: Arc<&Resource<Css>>| Arc::clone(&c.content.full))
            }
            None => None,
        }
    }

    fn get_component_css(&self) -> Option<String> {
        let mut collected_css = String::new();

        for component_id in self.view.dependency_list().list() {
            if let Some(a) = self.archive_cache.as_ref() {
                let details = ComponentDetails::new(
                    component_id.into(),
                    ResourceType::Css,
                    ResourceLevel::Component,
                );
                a.query(&details).map(|c: Arc<&Resource<Css>>| {
                    log::debug!("Adding CSS for {:?}", details);
                    collected_css += &(*c).content.full;
                });
            };
        }

        if collected_css.is_empty() {
            return None;
        }
        Some(collected_css)
    }

    fn get_component_js(&self) -> Vec<(ComponentId, Arc<String>)> {
        let mut collected_js = vec![];
        for component_id in self.view.dependency_list().list() {
            if let Some(a) = self.archive_cache.as_ref() {
                let details = ComponentDetails::new(
                    component_id.into(),
                    ResourceType::JavaScript,
                    ResourceLevel::Component,
                );
                a.query(&details).map(|c: Arc<&Resource<Js>>| {
                    log::debug!("Adding JavaScript for {:?}", details);
                    collected_js
                        .push((component_id.into(), Arc::clone(&(*c).content)));
                });
            };
        }

        collected_js
    }
}

impl<P: PageModel> From<P> for Page<P, PagePreparing> {
    fn from(model: P) -> Self {
        Self {
            model,
            view: PageView::default(),
            archive_cache: None,
            execution_state: std::marker::PhantomData,
        }
    }
}
