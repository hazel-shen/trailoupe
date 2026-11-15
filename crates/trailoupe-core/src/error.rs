//! Unified error type for the core library.

use std::path::PathBuf;

/// Errors produced by `trailoupe-core`.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Reading or opening a file failed.
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    /// An SQLite database (e.g. `Manifest.db`) could not be read.
    #[error("SQLite error: {0}")]
    Sqlite(#[from] rusqlite::Error),

    /// A property list could not be parsed.
    #[error("plist error: {0}")]
    Plist(#[from] plist::Error),

    /// JSON could not be parsed or serialized.
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    /// A required file is missing from the input.
    #[error("file not found: {}", .0.display())]
    MissingFile(PathBuf),

    /// The input is malformed or not what was expected.
    #[error("invalid input: {0}")]
    InvalidInput(String),

    /// The backup is encrypted and decryption is not available yet.
    #[error("encrypted backups are not supported yet")]
    EncryptedBackup,

    /// No indicators are loaded, so a scan result would be meaningless.
    #[error("no indicators loaded; cannot check for known indicators")]
    NoIndicatorsLoaded,

    /// The requested functionality is a placeholder.
    #[error("not implemented: {0}")]
    NotImplemented(&'static str),
}

/// Convenience alias used throughout the crate.
pub type Result<T> = std::result::Result<T, Error>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn io_error_converts_and_displays() {
        let io = std::io::Error::new(std::io::ErrorKind::NotFound, "gone");
        let err: Error = io.into();
        assert!(matches!(err, Error::Io(_)));
        assert_eq!(err.to_string(), "I/O error: gone");
    }

    #[test]
    fn not_implemented_displays_feature() {
        let err = Error::NotImplemented("backup::read_manifest");
        assert_eq!(err.to_string(), "not implemented: backup::read_manifest");
    }
}
