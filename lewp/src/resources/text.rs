use {
    crate::{
        component::ComponentId,
        resources::ResourceType,
        storage::{Level, Storage, StorageComponent},
    },
    mime::Mime,
    rust_embed::RustEmbed,
};

/// Enables loading text files from a [Storage].
pub struct Text {
    id: ComponentId,
    level: Level,
}

impl StorageComponent for Text {
    /// The actual content of the text file.
    type Content = String;
    /// The file name relative to the level's "text" folder without file extension.
    type ContentParameter = String;

    fn content<T: Storage>(
        &self,
        params: Self::ContentParameter,
    ) -> anyhow::Result<Self::Content> {
        let mut filename = T::folder_path(self);
        filename.push(params);
        let extension = match ResourceType::Text.extension() {
            Some(e) => e,
            None => {
                return Err(
                    anyhow::anyhow!(
                        "The extension for a text file could not be found! This error should never occur!"
                    )
                );
            }
        };
        filename.set_extension(extension);
        log::trace!("filename: {:#?}", filename);
        let filename = match filename.to_str() {
            Some(s) => s,
            None => {
                return Err(anyhow::anyhow!(
                    "Could not convert {} to str!",
                    filename.display()
                ))
            }
        };
        let text = match <T as RustEmbed>::get(&filename) {
            Some(r) => r,
            None => {
                return Err(anyhow::anyhow!(
                    "Error reading text file from file hierarchy!",
                ));
            }
        };
        Ok(String::from(std::str::from_utf8(&text.data)?))
    }

    fn id(&self) -> ComponentId {
        self.id.clone()
    }

    fn level(&self) -> Level {
        self.level
    }

    fn kind(&self) -> ResourceType {
        ResourceType::Text
    }

    fn mime_type() -> Mime {
        mime::TEXT_STAR
    }
}

impl Text {
    /// Creates a new Text component.
    pub fn new(id: ComponentId, level: Level) -> Self {
        Self { id, level }
    }
}
