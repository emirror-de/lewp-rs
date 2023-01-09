use {
    super::property_classification::PropertyClassification,
    crate::{
        fh::{
            Component as FHComponent,
            ComponentInformation as FHComponentInformation,
            FileHierarchy,
            Level,
        },
        LewpError,
        LewpErrorKind,
    },
    lewp_css::{
        cssparser::ToCss,
        domain::{
            at_rules::{document::DocumentAtRule, media::MediaAtRule},
            properties::{Importance, PropertyDeclaration},
            selectors::OurSelectorImpl,
            CssRule,
            CssRules,
            StyleRule,
        },
        Stylesheet,
    },
    selectors::parser::Selector,
    std::{path::PathBuf, rc::Rc, sync::Arc},
};

/// This keyword is intentionally defined with a whitespace at the end.
const CSS_COMPONENT_IDENTIFIER: &str = "#component ";

/// Responsible for CSS that is stored for a given [FHComponent].
///
/// Processes all files in the components directory and combines them into one
/// CSS [Stylesheet]. The resulting stylesheet is isolated to the scope of the
/// module it belongs to. If the stylesheet's [level](Level) is [Page](Level::Page),
/// then the resulting stylesheet is *NOT* isolated as there is no reason for
/// isolating a page wide CSS rule.
pub struct Component {
    fh: Arc<FileHierarchy>,
    component_information: Arc<FHComponentInformation>,
}

impl FHComponent for Component {
    /// The actual content is parsed and provided as [Stylesheet].
    type Content = Stylesheet;
    type ContentParameter = ();

    fn component_information(&self) -> Arc<FHComponentInformation> {
        self.component_information.clone()
    }

    fn content(
        &self,
        _params: Self::ContentParameter,
    ) -> Result<Self::Content, LewpError> {
        let files = self.fh.get_file_list(self)?;
        let css_raw = self.combine_files(files)?;
        let stylesheet = match Stylesheet::parse(&css_raw) {
            Ok(s) => s,
            Err(msg) => {
                return Err(LewpError::new(
                    LewpErrorKind::Css,
                    &format!("{msg:#?}"),
                    self.component_information.clone(),
                ));
            }
        };
        match &self.component_information.level {
            Level::Page => return Ok(stylesheet), // there is no reason for pages to be isolated
            _ => (),
        }
        let stylesheet = self.isolate_stylesheet(stylesheet)?;
        Ok(stylesheet)
    }

    fn file_hierarchy(&self) -> Arc<FileHierarchy> {
        self.fh.clone()
    }
}

impl Component {
    /// Creates a new CSS component
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
                        if s.to_css_string()
                            .starts_with(CSS_COMPONENT_IDENTIFIER)
                        {
                            self.replace_identifier_and_append_module_prefix(
                                s,
                            )?;
                            continue;
                        }
                        self.add_component_prefix(s)?;
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

    fn add_component_prefix(
        &self,
        selector: &mut Selector<OurSelectorImpl>,
    ) -> Result<(), LewpError> {
        let mut old = String::new();
        if let Err(e) = selector.to_css(&mut old) {
            return Err(LewpError::new(
                LewpErrorKind::Css,
                &format!("{e:#?}"),
                self.component_information(),
            ));
        };
        let new = match lewp_css::parse_css_selector(&format!(
            ".{} {}",
            self.id(),
            old
        )) {
            Err(e) => {
                return Err(LewpError::new(
                    LewpErrorKind::Css,
                    &format!("{e:#?}"),
                    self.component_information(),
                ));
            }
            Ok(s) => s,
        };
        *selector = new;
        Ok(())
    }

    fn replace_identifier_and_append_module_prefix(
        &self,
        selector: &mut Selector<OurSelectorImpl>,
    ) -> Result<(), LewpError> {
        let mut old = String::new();
        if let Err(e) = selector.to_css(&mut old) {
            return Err(LewpError::new(
                LewpErrorKind::Css,
                &format!("{e:#?}"),
                self.component_information(),
            ));
        };
        let new = match lewp_css::parse_css_selector(&format!(
            "{}.{}",
            old.replace(CSS_COMPONENT_IDENTIFIER, ""),
            self.id()
        )) {
            Err(e) => {
                return Err(LewpError::new(
                    LewpErrorKind::Css,
                    &format!("{e:#?}"),
                    self.component_information(),
                ));
            }
            Ok(s) => s,
        };
        *selector = new;
        Ok(())
    }

    /// Creates a new stylesheet that contains only render critical properties.
    pub fn extract_render_critical_stylesheet(
        &self,
        stylesheet: Stylesheet,
    ) -> Result<<Self as FHComponent>::Content, LewpError> {
        self.filter_stylesheet_properties(
            stylesheet,
            Rc::new(Box::new(|x| x.is_render_critical())),
        )
    }

    /// Creates a new stylesheet that contains only NON render critical properties.
    pub fn extract_non_render_critical_stylesheet(
        &self,
        stylesheet: Stylesheet,
    ) -> Result<<Self as FHComponent>::Content, LewpError> {
        self.filter_stylesheet_properties(
            stylesheet,
            Rc::new(Box::new(|x| !x.is_render_critical())),
        )
    }

    /// Creates a new stylesheet and filters the properties by the given closure.
    ///
    /// It automatically cleans up empty rules.
    pub fn filter_stylesheet_properties(
        &self,
        stylesheet: Stylesheet,
        filter: Rc<Box<dyn Fn(&PropertyDeclaration<Importance>) -> bool>>,
    ) -> Result<<Self as FHComponent>::Content, LewpError> {
        let mut stylesheet = stylesheet;

        self.filter_rules(&mut stylesheet.rules, filter, true)?;

        self.remove_empty_rules(&mut stylesheet.rules);

        Ok(stylesheet)
    }

    fn filter_rules(
        &self,
        rules: &mut CssRules,
        filter: Rc<Box<dyn Fn(&PropertyDeclaration<Importance>) -> bool>>,
        recursive: bool,
    ) -> Result<(), LewpError> {
        for rule in &mut rules.0 {
            let iteration_filter = Rc::clone(&filter);
            match rule {
                CssRule::Style(StyleRule {
                    property_declarations,
                    ..
                }) => {
                    property_declarations.0.retain(|x| iteration_filter(x));
                }
                CssRule::Media(MediaAtRule { rules, .. })
                | CssRule::Document(DocumentAtRule { rules, .. }) => {
                    if !recursive {
                        continue;
                    }
                    self.filter_rules(rules, iteration_filter, true)?
                }
                _ => {}
            }
        }

        Ok(())
    }

    fn remove_empty_rules(&self, rules: &mut CssRules) {
        rules.0.retain(|r| match r {
            CssRule::Style(StyleRule {
                property_declarations,
                ..
            }) => !property_declarations.is_empty(),
            _ => false,
        });
    }
}
