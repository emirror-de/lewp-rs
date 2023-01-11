use {
    crate::{
        fh::{
            Component as FHComponent,
            ComponentInformation as FHComponentInformation,
            FileHierarchy,
        },
        LewpError,
        LewpErrorKind,
    },
    minify_js::{minify, TopLevelMode},
    std::{path::PathBuf, rc::Rc, sync::Arc},
};

/// Responsible for JS that is stored for a given [FHComponent].
///
/// Processes all files in the components directory and combines them into one
/// JavaScript file. The resulting file is used to initialize your component on
/// the client side.
pub struct Component {
    fh: Arc<FileHierarchy>,
    component_information: Arc<FHComponentInformation>,
}

impl FHComponent for Component {
    /// The actual content is parsed and provided as String.
    type Content = String;
    type ContentParameter = ();

    fn component_information(&self) -> Arc<FHComponentInformation> {
        self.component_information.clone()
    }

    fn content(
        &self,
        _params: Self::ContentParameter,
    ) -> Result<Self::Content, LewpError> {
        let files = self.fh.get_file_list(self)?;
        let js = self.combine_files(files)?;
        let mut result = Vec::new();
        let js = match minify(
            TopLevelMode::Global,
            js.into_bytes().to_vec(),
            &mut result,
        ) {
            Ok(j) => j,
            Err(e) => {
                return Err(LewpError {
                    kind: LewpErrorKind::JavaScript,
                    message: format!("Could not minify JavaScript: {e}"),
                    source_component: self.component_information.clone(),
                })
            }
        };
        match String::from_utf8(result) {
            Ok(r) => Ok(r),
            Err(e) => {
                return Err(LewpError {
                    kind: LewpErrorKind::JavaScript,
                    message: format!(
                        "Could not create String from minified JavaScript: {e}",
                    ),
                    source_component: self.component_information.clone(),
                })
            }
        }
    }

    fn file_hierarchy(&self) -> Arc<FileHierarchy> {
        self.fh.clone()
    }
}

impl Component {
    /// Creates a new JS component
    pub fn new(
        component_information: Arc<FHComponentInformation>,
        fh: Arc<FileHierarchy>,
    ) -> Self {
        Self {
            fh,
            component_information,
        }
    }

    fn combine_files(
        &self,
        css_files: Vec<PathBuf>,
    ) -> Result<String, LewpError> {
        let mut css_combined = String::new();
        for css_file_name in css_files {
            let css = match std::fs::read_to_string(&css_file_name) {
                Ok(r) => r,
                Err(msg) => {
                    return Err(LewpError::new(
                        LewpErrorKind::Css,
                        &format!("Error reading stylesheet file: {msg}"),
                        self.component_information.clone(),
                    ));
                }
            };
            css_combined.push_str(&css);
        }
        Ok(css_combined)
    }
}
