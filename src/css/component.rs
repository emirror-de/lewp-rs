use {
    crate::{
        fh::{Component, ComponentType, FileHierarchy},
        LewpError,
        LewpErrorKind,
    },
    css_next::{
        cssparser::ToCss,
        domain::{
            at_rules::{document::DocumentAtRule, media::MediaAtRule},
            CssRule,
            CssRules,
            StyleRule,
        },
        Stylesheet,
    },
    std::{io::Read, path::PathBuf},
};

/// Helps creating a Css instance.
pub struct CssComponentBuilder {
    fh: FileHierarchy,
    exclude_files: Vec<PathBuf>,
    component: Component,
}

impl CssComponentBuilder {
    /// Creates a new instance.
    pub fn new(component: Component) -> Self {
        Self {
            component,
            fh: FileHierarchy::new(),
            exclude_files: vec![],
        }
    }

    /// Sets the file hierarchy.
    pub fn file_hierarchy(mut self, fh: FileHierarchy) -> Self {
        self.fh = fh;
        self
    }

    /// Sets the excluded files.
    pub fn exclude_files(mut self, files: Vec<PathBuf>) -> Self {
        self.exclude_files = files;
        self
    }

    /// Creates the Css instance.
    pub fn build(self) -> CssComponent {
        CssComponent {
            fh: self.fh,
            exclude_files: self.exclude_files,
            component: self.component,
        }
    }
}

/// Responsible for CSS that is stored for a given [Component].
pub struct CssComponent {
    fh: FileHierarchy,
    exclude_files: Vec<PathBuf>,
    component: Component,
}

impl CssComponent {
    fn combine_files(
        &self,
        css_files: Vec<PathBuf>,
    ) -> Result<String, LewpError> {
        let mut css_combined = String::new();
        for css_file_name in css_files {
            let mut css = String::new();
            let file_path =
                self.fh.folder(&self.component).join(&css_file_name);
            let mut css_file = match std::fs::File::open(&file_path) {
                Ok(c) => c,
                Err(msg) => {
                    return Err(LewpError {
                        kind: LewpErrorKind::Css,
                        message: format!(
                            "Error opening file {}: {}",
                            css_file_name.to_str().unwrap(),
                            msg
                        ),
                        source_component: self.component.clone(),
                    });
                }
            };
            match css_file.read_to_string(&mut css) {
                Ok(_) => (),
                Err(msg) => {
                    return Err(LewpError {
                        kind: LewpErrorKind::Css,
                        message: format!("Error loading stylesheet: {}", msg),
                        source_component: self.component.clone(),
                    });
                }
            };
            css_combined.push_str(&css);
        }
        Ok(css_combined)
    }

    fn isolate_module_css(&self, css_raw: &str) -> Result<String, LewpError> {
        let css_raw = css_raw.to_owned();
        let stylesheet = match Stylesheet::parse(&css_raw) {
            Ok(s) => s,
            Err(msg) => {
                return Err(LewpError {
                    kind: LewpErrorKind::Css,
                    message: format!("{:#?}", msg),
                    source_component: self.component.clone(),
                });
            }
        };
        Ok(String::new())
    }

    /// Prepares and processes CSS files for given component. Returns the
    /// processed CSS as String.
    pub fn process(&self) -> Result<String, LewpError> {
        let files = self.fh.collect_filenames(&self.component);
        let css_raw = self.combine_files(files?)?;
        Ok(css_raw)
    }
}

#[test]
fn collect_css_files() {
    use crate::fh::Level;
    let id = "hello-world";

    // get temporary directory
    let dir = tempfile::tempdir().unwrap();
    // base the file hierarchy to this directory
    let fh = crate::fh::FileHierarchyBuilder::new()
        .base_directory(dir.path().to_path_buf())
        .build();

    // create path where the testfiles should be copied
    let testfiles_destination = dir.path().join("modules");
    let testfiles_source = "testfiles/modules";
    let mut copy_options = fs_extra::dir::CopyOptions::new();
    copy_options.copy_inside = true;
    match fs_extra::dir::copy(
        testfiles_source,
        testfiles_destination,
        &copy_options,
    ) {
        Err(msg) => panic!("{}", msg.to_string()),
        Ok(_) => (),
    };

    let css = CssComponent {
        fh,
        exclude_files: vec![],
        component: Component::new(
            "hello-world",
            Level::Module,
            ComponentType::CSS,
        ),
    };
    let css = match css.process() {
        Ok(c) => c,
        Err(e) => panic!("{}", e),
    };
    assert_eq!(
        css,
        String::from("h2 {\n    font-style: italic;\n}\nh1 {\n    font-style: bold;\n}\n")
        );

    match dir.close() {
        Err(msg) => panic!("{}", msg.to_string()),
        Ok(_) => (),
    }
}
