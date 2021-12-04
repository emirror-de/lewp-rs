use {
    crate::{
        fh::{
            Component as FHComponent,
            ComponentInformation as FHComponentInformation,
            ComponentType,
            FileHierarchy,
        },
        LewpError,
        LewpErrorKind,
    },
    css_next::{
        cssparser::ToCss,
        domain::{
            at_rules::{document::DocumentAtRule, media::MediaAtRule},
            selectors::OurSelectorImpl,
            CssRule,
            CssRules,
            StyleRule,
        },
        selectors::parser::Selector,
        Stylesheet,
    },
    std::{io::Read, path::PathBuf, rc::Rc},
};

/// Responsible for CSS that is stored for a given [FHComponent].
///
/// Processes all files in the components directory and combines them into one
/// CSS [Stylesheet]. The stylesheet is already isolated to the scope of the
/// desired module.
pub struct Component {
    fh: Rc<FileHierarchy>,
    component_information: Rc<FHComponentInformation>,
}

impl FHComponent for Component {
    type Content = Stylesheet;
    type ContentParameter = ();

    fn component_information(&self) -> Rc<FHComponentInformation> {
        self.component_information.clone()
    }

    fn content(
        &self,
        params: Self::ContentParameter,
    ) -> Result<Self::Content, LewpError> {
        let files = self.fh.get_file_list(self)?;
        let css_raw = self.combine_files(files)?;
        let stylesheet = match Stylesheet::parse(&css_raw) {
            Ok(s) => s,
            Err(msg) => {
                return Err(LewpError::new(
                    LewpErrorKind::Css,
                    &format!("{:#?}", msg),
                    self.component_information.clone(),
                ));
            }
        };
        let stylesheet = self.isolate_stylesheet(stylesheet)?;
        Ok(stylesheet)
    }

    fn file_hierarchy(&self) -> Rc<FileHierarchy> {
        self.fh.clone()
    }
}

impl Component {
    fn combine_files(
        &self,
        css_files: Vec<PathBuf>,
    ) -> Result<String, LewpError> {
        let mut css_combined = String::new();
        for css_file_name in css_files {
            let file_path = self.fh.folder(self).join(&css_file_name);
            let css = match std::fs::read_to_string(&file_path) {
                Ok(r) => r,
                Err(msg) => {
                    return Err(LewpError::new(
                        LewpErrorKind::Css,
                        &format!("Error reading stylesheet file: {}", msg),
                        self.component_information.clone(),
                    ));
                }
            };
            css_combined.push_str(&css);
        }
        Ok(css_combined)
    }

    fn isolate_stylesheet(
        &self,
        stylesheet: Stylesheet,
    ) -> Result<<Self as FHComponent>::Content, LewpError> {
        let mut stylesheet = stylesheet;
        self.isolate_rules(&mut stylesheet.rules, true)?;
        Ok(stylesheet)
    }

    fn isolate_rules(
        &self,
        rules: &mut CssRules,
        recursive: bool,
    ) -> Result<(), LewpError> {
        for rule in &mut rules.0 {
            match rule {
                CssRule::Style(StyleRule { selectors, .. }) => {
                    for s in &mut selectors.0 {
                        self.add_module_prefix(s)?;
                    }
                }
                CssRule::Media(MediaAtRule { rules, .. })
                | CssRule::Document(DocumentAtRule { rules, .. }) => {
                    if !recursive {
                        continue;
                    }
                    self.isolate_rules(rules, true)?
                }
                _ => {}
            }
        }
        Ok(())
    }

    fn add_module_prefix(
        &self,
        selector: &mut Selector<OurSelectorImpl>,
    ) -> Result<(), LewpError> {
        let mut old = String::new();
        selector.to_css(&mut old);
        let new = match css_next::parse_css_selector(&format!(
            ".{} {}",
            self.id(),
            old
        )) {
            Err(e) => {
                return Err(LewpError::new(
                    LewpErrorKind::Css,
                    &format!("{:#?}", e),
                    self.component_information().clone(),
                ));
            }
            Ok(s) => s,
        };
        *selector = new;
        Ok(())
    }
}

#[test]
fn isolate_css_module() {
    use {crate::fh::Level, css_next::cssparser::ToCss};

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

    let css = Component {
        fh: Rc::new(fh),
        component_information: Rc::new(FHComponentInformation {
            id: String::from("hello-world"),
            level: Level::Module,
            kind: ComponentType::Css,
        }),
    };
    let stylesheet = match css.content(()) {
        Ok(c) => c,
        Err(e) => panic!("{}", e),
    };
    assert_eq!(
        stylesheet.to_css_string(true),
        String::from(".hello-world h1{font-style: bold}.hello-world h2{font-style: italic}")
        );

    match dir.close() {
        Err(msg) => panic!("{}", msg.to_string()),
        Ok(_) => (),
    }
}
