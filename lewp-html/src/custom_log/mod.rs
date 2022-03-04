//! Contains modified versions of the log crate.

/// Error log with added prefix for this crate.
macro_rules! error {
    ($($arg:tt)*) => (
        log::error!("(LEWP-DOM) {}", format!($($arg)*));
    )
}

/// Warning log with added prefix for this crate.
macro_rules! warn {
    ($($arg:tt)*) => (
        log::warn!("(LEWP-DOM) {}", format!($($arg)*));
    )
}

/// Info log with added prefix for this crate.
macro_rules! info {
    ($($arg:tt)*) => (
        log::info!("(LEWP-DOM) {}", format!($($arg)*));
    )
}

/// Debug log with added prefix for this crate.
macro_rules! debug {
    ($($arg:tt)*) => (
        log::debug!("(LEWP-DOM) {}", format!($($arg)*));
    )
}

/// Trace log with added prefix for this crate.
macro_rules! trace {
    ($($arg:tt)*) => (
        log::trace!("(LEWP-DOM) {}", format!($($arg)*));
    )
}
