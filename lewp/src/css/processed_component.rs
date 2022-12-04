use {
    super::Component,
    crate::{fh::Component as FHComponent, LewpError},
    std::sync::Arc,
};

/// Container of a processed CSS component.
pub struct ProcessedComponent {
    render_critical: Arc<String>,
}

impl ProcessedComponent {
    /// Returns the render critical part of the processed [css_next::Stylesheet] [Component].
    pub fn render_critical(&self) -> Arc<String> {
        Arc::clone(&self.render_critical)
    }

    /// Takes the content of the given component and processes it for the usage
    /// within a website.
    pub fn from(
        component: &Component,
    ) -> Result<ProcessedComponent, LewpError> {
        let content = component.content(())?;
        Ok(ProcessedComponent {
            render_critical: Arc::new(content.to_css_string(false)),
        })
    }
}
