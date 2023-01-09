use {
    super::Component,
    crate::{fh::Component as FHComponent, LewpError},
    std::sync::Arc,
};

/// Container of a processed CSS component.
pub struct ProcessedComponent {
    render_critical: Arc<String>,
    non_render_critical: Arc<String>,
    full: Arc<String>,
}

impl ProcessedComponent {
    /// Returns the render critical part of the processed [css_next::Stylesheet] [Component].
    pub fn render_critical(&self) -> Arc<String> {
        Arc::clone(&self.render_critical)
    }
    /// Returns the NON render critical part of the processed [css_next::Stylesheet] [Component].
    pub fn non_render_critical(&self) -> Arc<String> {
        Arc::clone(&self.non_render_critical)
    }
    /// Returns the complete processed [css_next::Stylesheet] [Component].
    pub fn full(&self) -> Arc<String> {
        Arc::clone(&self.full)
    }
}

impl TryFrom<&Component> for ProcessedComponent {
    type Error = LewpError;
    fn try_from(value: &Component) -> Result<Self, Self::Error> {
        let origin = value.content(())?;
        let render_critical =
            value.extract_render_critical_stylesheet(origin.clone())?;
        let non_render_critical =
            value.extract_non_render_critical_stylesheet(origin.clone())?;
        Ok(ProcessedComponent {
            render_critical: Arc::new(render_critical.to_css_string(false)),
            non_render_critical: Arc::new(
                non_render_critical.to_css_string(false),
            ),
            full: Arc::new(origin.to_css_string(false)),
        })
    }
}
