//! Integration of JavaScript for Lewp.
use {
    crate::{
        archive::{Archive, ArchiveComponent},
        component::{ComponentDetails, ComponentId},
        resources::{ResourceLevel, ResourceType},
    },
    mime::Mime,
    minify_js::{minify, TopLevelMode},
    rust_embed::RustEmbed,
    std::{path::PathBuf, sync::Arc},
};

/// The options to be passed when loading a [Js] component.
#[derive(Debug)]
pub struct JsOptions {
    /// The component id.
    pub id: ComponentId,
    /// The resource level.
    pub level: ResourceLevel,
}

/// Responsible for JS that is stored for a given [Archive].
///
/// Processes all files in the components directory and combines them into one
/// JavaScript file. The resulting file is used to initialize your component on
/// the client side.
pub struct Js {
    details: ComponentDetails,
    /// The JavaScript content.
    pub content: Arc<String>,
}

impl ArchiveComponent for Js {
    type Options = JsOptions;

    fn load<A: Archive>(options: Self::Options) -> anyhow::Result<Self> {
        let details = ComponentDetails::new(
            options.id.clone(),
            ResourceType::JavaScript,
            options.level.clone(),
        );
        log::debug!("Created ComponentDetails for {options:?}:\n{details:#?}");

        let files = A::get_file_list(&details);
        log::debug!("Found {} JavaScript files.", files.len());
        log::debug!("Combining the JavaScript files for component {details:?}",);
        let js = Self::combine_files::<A>(files)?;

        let mut result = Vec::new();
        log::debug!(
            "Minifying combined JavaScript files for component {details:?}",
        );
        match minify(
            TopLevelMode::Module,
            js.into_bytes().to_vec(),
            &mut result,
        ) {
            Ok(j) => j,
            Err(e) => {
                return Err(anyhow::anyhow!(
                    "Could not minify JavaScript: {e}",
                ));
            }
        };
        let content = match String::from_utf8(result) {
            Ok(r) => Arc::new(r),
            Err(e) => {
                return Err(anyhow::anyhow!(
                    "Could not create String from minified JavaScript: {e}",
                ));
            }
        };
        Ok(Self { details, content })
    }

    fn mime_type() -> Mime {
        mime::APPLICATION_JAVASCRIPT
    }

    fn details(&self) -> &ComponentDetails {
        &self.details
    }
}

impl Js {
    fn combine_files<A: Archive>(
        js_files: Vec<PathBuf>,
    ) -> anyhow::Result<String> {
        let mut js_combined = String::new();
        for js_file_name in js_files {
            let js_file_name = match js_file_name.to_str() {
                Some(s) => s,
                None => {
                    return Err(anyhow::anyhow!(
                        "Could not convert {} to str!",
                        js_file_name.display()
                    ))
                }
            };
            let js = match <A as RustEmbed>::get(&js_file_name) {
                Some(r) => r,
                None => {
                    return Err(anyhow::anyhow!(
                        "Could not get JavaScript file {js_file_name}",
                    ));
                }
            };
            let js = std::str::from_utf8(&js.data)?;
            js_combined.push_str(&js);
        }
        Ok(js_combined)
    }
}
