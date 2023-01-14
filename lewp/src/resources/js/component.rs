use {
    crate::{
        component::ComponentId,
        storage::{Level, ResourceType, Storage, StorageComponent},
    },
    minify_js::{minify, TopLevelMode},
    rust_embed::RustEmbed,
    std::{path::PathBuf, sync::Arc},
};

/// Responsible for JS that is stored for a given [FHComponent].
///
/// Processes all files in the components directory and combines them into one
/// JavaScript file. The resulting file is used to initialize your component on
/// the client side.
pub struct Js {
    id: ComponentId,
    level: Level,
}

impl StorageComponent for Js {
    /// The actual content is parsed and provided as String.
    type Content = String;
    type ContentParameter = ();

    fn content<T: Storage>(
        &self,
        _params: Self::ContentParameter,
    ) -> anyhow::Result<Self::Content> {
        let files = T::get_file_list(self);
        let js = self.combine_files::<T>(files)?;
        let mut result = Vec::new();
        let js = match minify(
            TopLevelMode::Global,
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
        match String::from_utf8(result) {
            Ok(r) => Ok(r),
            Err(e) => {
                return Err(anyhow::anyhow!(
                    "Could not create String from minified JavaScript: {e}",
                ));
            }
        }
    }

    fn id(&self) -> ComponentId {
        self.id.clone()
    }

    fn level(&self) -> Level {
        self.level
    }

    fn kind(&self) -> ResourceType {
        ResourceType::JavaScript
    }
}

impl Js {
    /// Creates a new JS component
    pub fn new(id: ComponentId, level: Level) -> Self {
        Self { id, level }
    }

    fn combine_files<T: Storage>(
        &self,
        css_files: Vec<PathBuf>,
    ) -> anyhow::Result<String> {
        let mut css_combined = String::new();
        for css_file_name in css_files {
            let css_file_name = match css_file_name.to_str() {
                Some(s) => s,
                None => {
                    return Err(anyhow::anyhow!(
                        "Could not convert {} to str!",
                        css_file_name.display()
                    ))
                }
            };
            let css = match <T as RustEmbed>::get(&css_file_name) {
                Some(r) => r,
                None => {
                    return Err(anyhow::anyhow!(
                        "Could not get JavaScript file {css_file_name}",
                    ));
                }
            };
            let css = std::str::from_utf8(&css.data)?;
            css_combined.push_str(&css);
        }
        Ok(css_combined)
    }
}
