//! Android stub for the `trash` crate.
//! Android has no desktop trash spec; this provides a no-op API that
//! matches the upstream surface so dependent crates compile.

use std::path::PathBuf;

#[derive(Debug)]
pub struct Error(pub std::io::Error);

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "trash unsupported on Android: {}", self.0)
    }
}

impl std::error::Error for Error {}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error(e)
    }
}

pub struct TrashContext;

impl TrashContext {
    pub fn new() -> Self {
        TrashContext
    }

    pub fn delete_all<P: Into<PathBuf>>(&self, _paths: Vec<P>) -> Result<(), Error> {
        Err(Error(std::io::Error::new(
            std::io::ErrorKind::Unsupported,
            "trash is not supported on Android",
        )))
    }
}

impl Default for TrashContext {
    fn default() -> Self {
        Self::new()
    }
}

/// Free-function form used by Lapce (`trash::delete(path)`).
/// Android has no desktop trash, so this always errors.
pub fn delete<P: Into<PathBuf>>(_path: P) -> Result<(), Error> {
    Err(Error(std::io::Error::new(
        std::io::ErrorKind::Unsupported,
        "trash is not supported on Android",
    )))
}
