use {
    crate::resources::css::PropertyClassification,
    lewp_css::{
        domain::{
            at_rules::{document::DocumentAtRule, media::MediaAtRule},
            properties::{Importance, PropertyDeclaration},
            CssRule,
            CssRules,
            StyleRule,
        },
        Stylesheet,
    },
    std::{rc::Rc, sync::Arc},
};

/// Container of a processed CSS component.
#[derive(Debug)]
pub struct ProcessedComponent {
    pub render_critical: Arc<String>,
    pub non_render_critical: Arc<String>,
    pub full: Arc<String>,
}

impl ProcessedComponent {
    /// Creates a new processed component from the given [Css].
    pub fn new(stylesheet: Stylesheet) -> anyhow::Result<Self> {
        let render_critical =
            Self::extract_render_critical_stylesheet(stylesheet.clone())?;
        let non_render_critical =
            Self::extract_non_render_critical_stylesheet(stylesheet.clone())?;
        Ok(ProcessedComponent {
            render_critical: Arc::new(render_critical.to_css_string(false)),
            non_render_critical: Arc::new(
                non_render_critical.to_css_string(false),
            ),
            full: Arc::new(stylesheet.to_css_string(false)),
        })
    }

    /// Creates a new stylesheet that contains only render critical properties.
    pub fn extract_render_critical_stylesheet(
        stylesheet: Stylesheet,
    ) -> anyhow::Result<Stylesheet> {
        Self::filter_stylesheet_properties(
            stylesheet,
            Rc::new(Box::new(|x| x.is_render_critical())),
        )
    }

    /// Creates a new stylesheet that contains only NON render critical properties.
    pub fn extract_non_render_critical_stylesheet(
        stylesheet: Stylesheet,
    ) -> anyhow::Result<Stylesheet> {
        Self::filter_stylesheet_properties(
            stylesheet,
            Rc::new(Box::new(|x| !x.is_render_critical())),
        )
    }

    /// Creates a new stylesheet and filters the properties by the given closure.
    ///
    /// It automatically cleans up empty rules.
    pub fn filter_stylesheet_properties(
        stylesheet: Stylesheet,
        filter: Rc<Box<dyn Fn(&PropertyDeclaration<Importance>) -> bool>>,
    ) -> anyhow::Result<Stylesheet> {
        let mut stylesheet = stylesheet;

        Self::filter_rules(&mut stylesheet.rules, filter, true)?;

        Self::remove_empty_rules(&mut stylesheet.rules);

        Ok(stylesheet)
    }

    fn filter_rules(
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
                    Self::filter_rules(rules, iteration_filter, true)?
                }
                _ => {}
            }
        }

        Ok(())
    }

    fn remove_empty_rules(rules: &mut CssRules) {
        rules.0.retain(|r| match r {
            CssRule::Style(StyleRule {
                property_declarations,
                ..
            }) => !property_declarations.is_empty(),
            _ => false,
        });
    }
}
