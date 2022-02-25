mod nodes;
/// Checks if appended nodes are allowed inside their parents.
pub mod validator;

#[cfg(test)]
mod test;

pub use nodes::*;
