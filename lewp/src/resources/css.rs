use {
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
    rust_embed::RustEmbed,
    selectors::parser::Selector,
    std::{path::PathBuf, rc::Rc, sync::Arc},
};

mod entireness;
mod processed_component;
mod property_classification;
mod register;
#[cfg(test)]
mod test;

pub use {
    entireness::Entireness,
    processed_component::ProcessedComponent,
    property_classification::PropertyClassification,
    register::{
        Register as CssRegister,
        RegisterOptions as CssRegisterOptions,
    },
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
pub struct Css {
    component_information: Arc<FHComponentInformation>,
}

impl FHComponent for Css {
    /// The actual content is parsed and provided as [Stylesheet].
    type Content = Stylesheet;
    type ContentParameter = ();

    fn component_information(&self) -> Arc<FHComponentInformation> {
        self.component_information.clone()
    }

    fn content<T: FileHierarchy>(
        &self,
        _params: Self::ContentParameter,
    ) -> anyhow::Result<Self::Content> {
        let files = T::get_file_list(self);
        let css_raw = self.combine_files::<T>(files)?;
        let stylesheet = match Stylesheet::parse(&css_raw) {
            Ok(s) => s,
            Err(msg) => {
                return Err(anyhow::anyhow!(
                    "{}",
                    LewpError::new(
                        LewpErrorKind::Css,
                        &format!("{msg:#?}"),
                        self.component_information.clone(),
                    )
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
}

impl Css {
    /// Creates a new CSS component
    pub fn new(component_information: Arc<FHComponentInformation>) -> Self {
        Self {
            component_information,
        }
    }

    fn combine_files<T: FileHierarchy>(
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
                        "{}",
                        LewpError::new(
                            LewpErrorKind::Css,
                            &format!("Stylesheet file not found."),
                            self.component_information.clone(),
                        )
                    ));
                }
            };
            let css = std::str::from_utf8(&css.data)?;
            css_combined.push_str(css);
        }
        Ok(css_combined)
    }

    fn isolate_stylesheet(
        &self,
        stylesheet: Stylesheet,
    ) -> anyhow::Result<<Self as FHComponent>::Content> {
        let mut stylesheet = stylesheet;
        self.isolate_rules(&mut stylesheet.rules, true)?;
        Ok(stylesheet)
    }

    fn isolate_rules(
        &self,
        rules: &mut CssRules,
        recursive: bool,
    ) -> anyhow::Result<()> {
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
    ) -> anyhow::Result<()> {
        let mut old = String::new();
        if let Err(e) = selector.to_css(&mut old) {
            return Err(anyhow::anyhow!(
                "{}",
                LewpError::new(
                    LewpErrorKind::Css,
                    &format!("{e:#?}"),
                    self.component_information(),
                )
            ));
        };
        let new = match lewp_css::parse_css_selector(&format!(
            ".{} {}",
            self.id(),
            old
        )) {
            Err(e) => {
                return Err(anyhow::anyhow!(
                    "{}",
                    LewpError::new(
                        LewpErrorKind::Css,
                        &format!("{e:#?}"),
                        self.component_information(),
                    )
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
    ) -> anyhow::Result<()> {
        let mut old = String::new();
        if let Err(e) = selector.to_css(&mut old) {
            return Err(anyhow::anyhow!(
                "{}",
                LewpError::new(
                    LewpErrorKind::Css,
                    &format!("{e:#?}"),
                    self.component_information(),
                )
            ));
        };
        let new = match lewp_css::parse_css_selector(&format!(
            "{}.{}",
            old.replace(CSS_COMPONENT_IDENTIFIER, ""),
            self.id()
        )) {
            Err(e) => {
                return Err(anyhow::anyhow!(
                    "{}",
                    LewpError::new(
                        LewpErrorKind::Css,
                        &format!("{e:#?}"),
                        self.component_information(),
                    )
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
    ) -> anyhow::Result<<Self as FHComponent>::Content> {
        self.filter_stylesheet_properties(
            stylesheet,
            Rc::new(Box::new(|x| x.is_render_critical())),
        )
    }

    /// Creates a new stylesheet that contains only NON render critical properties.
    pub fn extract_non_render_critical_stylesheet(
        &self,
        stylesheet: Stylesheet,
    ) -> anyhow::Result<<Self as FHComponent>::Content> {
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
    ) -> anyhow::Result<<Self as FHComponent>::Content> {
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
    ) -> anyhow::Result<()> {
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
