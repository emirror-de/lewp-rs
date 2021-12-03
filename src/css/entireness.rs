/// Defines the level of completeness.
pub enum Entireness {
    /// The entire CSS.
    Full,
    /// Only render critical parts, at least everything that affects
    /// [cumulative layout shift](https://web.dev/cls/).
    RenderCritical,
    /// Only non-render critical parts.
    NonRenderCritical,
}
