/// Used to reset a module to the initial state.
pub trait Reset {
    /// Can be used to reset the implementing module to the initial state.
    fn reset(&mut self);
}
