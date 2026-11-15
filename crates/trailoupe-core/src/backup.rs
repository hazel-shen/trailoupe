//! iOS backup (iTunes / Finder) analysis.
//!
//! A backup directory contains `Manifest.plist` (metadata, including whether
//! the backup is encrypted) and `Manifest.db` (an SQLite index mapping each
//! backed-up file to its hashed file ID).

use std::path::{Path, PathBuf};

use crate::error::{Error, Result};
use crate::timeline::TimelineEvent;

/// File name of the SQLite index of backed-up files.
pub const MANIFEST_DB: &str = "Manifest.db";
/// File name of the backup metadata property list.
pub const MANIFEST_PLIST: &str = "Manifest.plist";

/// One row of the `Files` table in `Manifest.db`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BackupFile {
    /// SHA-1 based file ID; the file is stored at `<root>/<id[0..2]>/<id>`.
    pub file_id: String,
    /// Backup domain, e.g. `HomeDomain` or `AppDomain-<bundle id>`.
    pub domain: String,
    /// Path of the file relative to its domain. Untrusted.
    pub relative_path: String,
    /// Entry type flags (file, directory, symlink).
    pub flags: i64,
}

/// An opened backup directory.
#[derive(Debug)]
pub struct Backup {
    root: PathBuf,
}

impl Backup {
    /// Opens a backup directory.
    ///
    /// TODO: read `Manifest.plist`, validate it and detect encryption
    /// (`IsEncrypted`), returning [`Error::EncryptedBackup`] until decryption
    /// exists.
    pub fn open(root: impl Into<PathBuf>) -> Result<Self> {
        let root = root.into();
        let manifest = root.join(MANIFEST_DB);
        if !manifest.is_file() {
            return Err(Error::MissingFile(manifest));
        }
        Ok(Self { root })
    }

    /// The backup directory.
    pub fn root(&self) -> &Path {
        &self.root
    }

    /// Path of `Manifest.db` inside the backup directory.
    pub fn manifest_db_path(&self) -> PathBuf {
        self.root.join(MANIFEST_DB)
    }

    /// Lists the files recorded in `Manifest.db`.
    ///
    /// TODO: open the database read-only (`SQLITE_OPEN_READ_ONLY`) and query
    /// the `Files` table. Treat every column as untrusted (wrong types, NULLs,
    /// path traversal in `relativePath`).
    pub fn read_manifest(&self) -> Result<Vec<BackupFile>> {
        Err(Error::NotImplemented("backup::Backup::read_manifest"))
    }

    /// Decrypts an encrypted backup's keybag and `Manifest.db`.
    ///
    /// TODO: implement; add the cryptography dependencies only at that point.
    /// The password must never be logged or written to disk.
    pub fn decrypt(&mut self, password: &str) -> Result<()> {
        let _ = password;
        Err(Error::NotImplemented("backup::Backup::decrypt"))
    }
}

/// Scans a backup directory and returns the extracted timeline events.
///
/// TODO: open the backup, read the manifest and run the individual artefact
/// extractors.
pub fn scan(path: &Path) -> Result<Vec<TimelineEvent>> {
    let _ = path;
    Err(Error::NotImplemented("backup::scan"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn open_rejects_directory_without_manifest() {
        let dir = std::env::temp_dir().join("trailoupe-test-no-manifest-does-not-exist");
        let err = Backup::open(&dir).unwrap_err();
        assert!(matches!(err, Error::MissingFile(p) if p == dir.join(MANIFEST_DB)));
    }

    #[test]
    fn scan_is_not_implemented_yet() {
        let err = scan(Path::new("whatever")).unwrap_err();
        assert!(matches!(err, Error::NotImplemented(_)));
    }
}
