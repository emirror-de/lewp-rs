use {
    super::Css,
    crate::storage::{Storage, StorageComponent},
    std::sync::Arc,
};

/// Container of a processed CSS component.
#[derive(Debug)]
pub struct ProcessedComponent {
    render_critical: Arc<String>,
    non_render_critical: Arc<String>,
    full: Arc<String>,
}

impl ProcessedComponent {
    /// Creates a new processed component from the given [Css].
    pub fn new<T: Storage>(comp: &Css) -> anyhow::Result<Self> {
        let origin = comp.content::<T>(())?;
        let render_critical =
            comp.extract_render_critical_stylesheet(origin.clone())?;
        let non_render_critical =
            comp.extract_non_render_critical_stylesheet(origin.clone())?;
        Ok(ProcessedComponent {
            render_critical: Arc::new(render_critical.to_css_string(false)),
            non_render_critical: Arc::new(
                non_render_critical.to_css_string(false),
            ),
            full: Arc::new(origin.to_css_string(false)),
        })
    }
    /// Returns the render critical part of the processed [css_next::Stylesheet] [Css].
    pub fn render_critical(&self) -> Arc<String> {
        Arc::clone(&self.render_critical)
    }
    /// Returns the NON render critical part of the processed [css_next::Stylesheet] [Css].
    pub fn non_render_critical(&self) -> Arc<String> {
        Arc::clone(&self.non_render_critical)
    }
    /// Returns the complete processed [css_next::Stylesheet] [Css].
    pub fn full(&self) -> Arc<String> {
        Arc::clone(&self.full)
    }
}
