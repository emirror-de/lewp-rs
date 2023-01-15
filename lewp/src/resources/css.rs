use {
    crate::{
        component::ComponentId,
        storage::{Level, ResourceType, Storage, StorageComponent},
    },
    lewp_css::{
        cssparser::ToCss,
        domain::{
            at_rules::{document::DocumentAtRule, media::MediaAtRule},
            selectors::OurSelectorImpl,
            CssRule,
            CssRules,
            StyleRule,
        },
        Stylesheet,
    },
    rust_embed::RustEmbed,
    selectors::parser::Selector,
    std::path::PathBuf,
};

mod entireness;
mod processed_component;
mod property_classification;
#[cfg(test)]
mod test;

pub(crate) use {
    entireness::Entireness,
    processed_component::ProcessedComponent,
    property_classification::PropertyClassification,
};

/// This keyword is intentionally defined with a whitespace at the end.
const CSS_COMPONENT_IDENTIFIER: &str = "#component ";

/// CSS resources available in a [Storage].
///
/// Processes all files in the components directory and combines them into one
/// CSS [Stylesheet]. The resulting stylesheet is isolated to the scope of the
/// module it belongs to. If the stylesheet's [level](Level) is [Page](Level::Page),
/// then the resulting stylesheet is *NOT* isolated as there is no reason for
/// isolating a page wide CSS rule.
pub struct Css {
    id: ComponentId,
    level: Level,
}

impl StorageComponent for Css {
    type Content = ProcessedComponent;
    type ContentParameter = ();

    fn content<T: Storage>(
        &self,
        _params: Self::ContentParameter,
    ) -> anyhow::Result<Self::Content> {
        let files = T::get_file_list(self);
        let css_raw = self.combine_files::<T>(files)?;
        let stylesheet = match Stylesheet::parse(&css_raw) {
            Ok(s) => s,
            Err(msg) => {
                return Err(anyhow::anyhow!("{msg:#?}",));
            }
        };
        match &self.level {
            Level::Page => return Ok(ProcessedComponent::new(stylesheet)?), // there is no reason for pages to be isolated
            _ => (),
        }
        let stylesheet = self.isolate_stylesheet(stylesheet)?;
        Ok(ProcessedComponent::new(stylesheet)?)
    }

    fn id(&self) -> ComponentId {
        self.id.clone()
    }

    fn level(&self) -> Level {
        self.level
    }

    fn kind(&self) -> ResourceType {
        ResourceType::Css
    }
}

impl Css {
    /// Creates a new CSS component
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
                    return Err(anyhow::anyhow!("Stylesheet file not found.",));
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
    ) -> anyhow::Result<Stylesheet> {
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
            return Err(anyhow::anyhow!("{e:#?}",));
        };
        let new = match lewp_css::parse_css_selector(&format!(
            ".{} {}",
            self.id(),
            old
        )) {
            Err(e) => {
                return Err(anyhow::anyhow!("{e:#?}",));
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
            return Err(anyhow::anyhow!("{e:#?}",));
        };
        let new = match lewp_css::parse_css_selector(&format!(
            "{}.{}",
            old.replace(CSS_COMPONENT_IDENTIFIER, ""),
            self.id()
        )) {
            Err(e) => {
                return Err(anyhow::anyhow!("{e:#?}",));
            }
            Ok(s) => s,
        };
        *selector = new;
        Ok(())
    }
}
