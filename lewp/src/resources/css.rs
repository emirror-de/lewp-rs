use {
    crate::{
        archive::{Archive, ArchiveComponent},
        component::{ComponentDetails, ComponentId},
        resources::{ResourceLevel, ResourceType},
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
    mime::Mime,
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
    //entireness::Entireness,
    processed_component::ProcessedComponent,
    property_classification::PropertyClassification,
};

/// This keyword is intentionally defined with a whitespace at the end.
const CSS_COMPONENT_IDENTIFIER: &str = "#component ";

/// Options to be passed when loading a [Css] component from disk.
#[derive(Debug)]
pub struct CssOptions {
    /// The unique component id.
    pub id: ComponentId,
    /// The resource level of the component.
    pub level: ResourceLevel,
}

/// CSS resources available in an [Archive].
///
/// Processes all files in the components directory and combines them into one
/// CSS [Stylesheet]. The resulting stylesheet is isolated to the scope of the
/// module it belongs to. If the stylesheet's [level](ResourceLevel) is [Page](ResourceLevel::Page),
/// then the resulting stylesheet is *NOT* isolated as there is no reason for
/// isolating a page wide CSS rule.
#[doc = include_str!(concat!("../../docs/resources/css.md"))]
#[derive(Debug)]
pub struct Css {
    details: ComponentDetails,
    /// The processed content of the [Css] component.
    pub content: ProcessedComponent,
}

impl ArchiveComponent for Css {
    type Options = CssOptions;
    fn load<A: Archive>(options: Self::Options) -> anyhow::Result<Self> {
        let details = ComponentDetails::new(
            options.id.clone(),
            ResourceType::Css,
            options.level,
        );
        log::debug!("Created ComponentDetails for {options:?}:\n{details:#?}");

        let files = A::get_file_list(&details);
        log::debug!("Found {} CSS files.", files.len());
        log::debug!("Combining the CSS files for component {details:?}",);
        let css_raw = Self::combine_files::<A>(files)?;
        log::debug!("Parsing combined stylesheet...",);
        let stylesheet = match Stylesheet::parse(&css_raw) {
            Ok(s) => s,
            Err(msg) => {
                return Err(anyhow::anyhow!("{msg:#?}",));
            }
        };
        log::debug!("Successfully parsed combined stylesheet for {details:?}",);
        match &options.level {
            ResourceLevel::Page => {
                let content = ProcessedComponent::new(stylesheet)?;
                return Ok(Self { details, content });
            } // there is no reason for pages to be isolated
            _ => (),
        }
        let stylesheet = Self::isolate_stylesheet(stylesheet, &options)?;
        let content = ProcessedComponent::new(stylesheet)?;
        Ok(Self { details, content })
    }

    fn mime_type() -> Mime {
        mime::TEXT_CSS
    }

    fn details(&self) -> &ComponentDetails {
        &self.details
    }
}

impl Css {
    fn combine_files<A: Archive>(
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
            let css = match <A as RustEmbed>::get(&css_file_name) {
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
        stylesheet: Stylesheet,
        options: &<Self as ArchiveComponent>::Options,
    ) -> anyhow::Result<Stylesheet> {
        let mut stylesheet = stylesheet;
        Self::isolate_rules(&mut stylesheet.rules, true, options)?;
        Ok(stylesheet)
    }

    fn isolate_rules(
        rules: &mut CssRules,
        recursive: bool,
        options: &<Self as ArchiveComponent>::Options,
    ) -> anyhow::Result<()> {
        for rule in &mut rules.0 {
            match rule {
                CssRule::Style(StyleRule { selectors, .. }) => {
                    for s in &mut selectors.0 {
                        if s.to_css_string()
                            .starts_with(CSS_COMPONENT_IDENTIFIER)
                        {
                            Self::replace_identifier_and_append_module_prefix(
                                s, options,
                            )?;
                            continue;
                        }
                        Self::add_component_prefix(s, options)?;
                    }
                }
                CssRule::Media(MediaAtRule { rules, .. })
                | CssRule::Document(DocumentAtRule { rules, .. }) => {
                    if !recursive {
                        continue;
                    }
                    Self::isolate_rules(rules, true, options)?
                }
                _ => {}
            }
        }
        Ok(())
    }

    fn add_component_prefix(
        selector: &mut Selector<OurSelectorImpl>,
        options: &<Self as ArchiveComponent>::Options,
    ) -> anyhow::Result<()> {
        let mut old = String::new();
        if let Err(e) = selector.to_css(&mut old) {
            return Err(anyhow::anyhow!("{e:#?}",));
        };
        let new = match lewp_css::parse_css_selector(&format!(
            ".{} {}",
            options.id, old
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
        selector: &mut Selector<OurSelectorImpl>,
        options: &<Self as ArchiveComponent>::Options,
    ) -> anyhow::Result<()> {
        let mut old = String::new();
        if let Err(e) = selector.to_css(&mut old) {
            return Err(anyhow::anyhow!("{e:#?}",));
        };
        let new = match lewp_css::parse_css_selector(&format!(
            "{}.{}",
            old.replace(CSS_COMPONENT_IDENTIFIER, ""),
            options.id
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
