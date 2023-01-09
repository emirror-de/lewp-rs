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

impl TryFrom<&Component> for ProcessedComponent {
    type Error = LewpError;
    fn try_from(value: &Component) -> Result<Self, Self::Error> {
        let origin = value.content(())?;
        let render_critical =
            value.extract_render_critical_stylesheet(origin)?;
        Ok(ProcessedComponent {
            render_critical: Arc::new(content.to_css_string(false)),
        })
    }
}
