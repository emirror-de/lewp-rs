//! Defines the different states of a page.

/// This trait defines the mode of the page.
pub trait FhState {}

/// Indicates the state when a page has a file hierarchy attached.
pub struct WithFileHierarchy;
impl FhState for WithFileHierarchy {}

/// Indicates the state when a page has no file hierarchy attached.
pub struct WithoutFileHierarchy;
impl FhState for WithoutFileHierarchy {}

/// Defines the state if a [CssRegister] has been attached.
pub trait CssState {}

/// Indicates a state with [CssRegister] attached.
pub struct WithCss;
impl CssState for WithCss {}

/// Indicates a state without [CssRegister] attached.
pub struct WithoutCss;
impl CssState for WithoutCss {}

/// Defines the state if a [JsRegister] has been attached.
pub trait JsState {}

/// Indicates a state with [JsRegister] attached.
pub struct WithJs;
impl JsState for WithJs {}

/// Indicates a state without [CssRegister] attached.
pub struct WithoutJs;
impl JsState for WithoutJs {}

/// Defines the execution state of a page.
pub trait ExecutionState {}

/// Indicates a state when the page has been run and is ready to render.
pub struct PageFinished;
impl ExecutionState for PageFinished {}

/// Indicates a state when the page is in a preparing state and ready to be executed.
pub struct PagePreparing;
impl ExecutionState for PagePreparing {}
