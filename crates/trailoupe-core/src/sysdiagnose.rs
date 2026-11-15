//! sysdiagnose archive analysis.
//!
//! A sysdiagnose is a `.tar.gz` archive. Of interest first is
//! `system_logs.logarchive/Extra/shutdown.log`, which records processes that
//! were still running when the device was shut down.

use std::fs::File;
use std::path::Path;

use flate2::read::GzDecoder;
use tar::Archive;

use crate::error::{Error, Result};
use crate::timeline::{TimelineEvent, Timestamp};

/// One process entry from `shutdown.log`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShutdownLogEntry {
    /// Time of the shutdown the entry belongs to, if known.
    pub timestamp: Option<Timestamp>,
    /// Process ID, if present on the line.
    pub pid: Option<u32>,
    /// Executable path of the process that was still running.
    pub process_path: String,
}

/// Opens a `.tar.gz` sysdiagnose archive for streaming.
pub fn open_archive(path: &Path) -> Result<Archive<GzDecoder<File>>> {
    let file = File::open(path)?;
    Ok(Archive::new(GzDecoder::new(file)))
}

/// Returns `true` if an archive entry path names the shutdown log.
pub fn is_shutdown_log(entry_path: &Path) -> bool {
    entry_path
        .file_name()
        .and_then(|n| n.to_str())
        .is_some_and(|n| n.eq_ignore_ascii_case("shutdown.log"))
}

/// Extracts the raw contents of `shutdown.log` from the archive.
///
/// TODO: iterate entries with [`open_archive`], match with
/// [`is_shutdown_log`], and read the entry into memory with a size cap.
/// Never extract to disk using archive-supplied paths (path traversal) and
/// guard against decompression bombs.
pub fn find_shutdown_log(archive_path: &Path) -> Result<Vec<u8>> {
    let _ = archive_path;
    Err(Error::NotImplemented("sysdiagnose::find_shutdown_log"))
}

/// Parses the text of `shutdown.log`.
///
/// TODO: implement a tolerant line parser; malformed lines are skipped or
/// reported, never panicked on.
pub fn parse_shutdown_log(content: &str) -> Result<Vec<ShutdownLogEntry>> {
    let _ = content;
    Err(Error::NotImplemented("sysdiagnose::parse_shutdown_log"))
}

/// Scans a sysdiagnose archive and returns the extracted timeline events.
///
/// TODO: combine [`find_shutdown_log`] and [`parse_shutdown_log`] and convert
/// the entries to [`TimelineEvent`]s.
pub fn scan(path: &Path) -> Result<Vec<TimelineEvent>> {
    let _ = path;
    Err(Error::NotImplemented("sysdiagnose::scan"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn recognises_shutdown_log_paths() {
        assert!(is_shutdown_log(Path::new(
            "sysdiagnose_2026/system_logs.logarchive/Extra/shutdown.log"
        )));
        assert!(!is_shutdown_log(Path::new("sysdiagnose_2026/ps.txt")));
        assert!(!is_shutdown_log(Path::new("")));
    }

    #[test]
    fn open_archive_reports_missing_file() {
        let result = open_archive(Path::new("does-not-exist.tar.gz"));
        assert!(matches!(result, Err(Error::Io(_))));
    }
}
