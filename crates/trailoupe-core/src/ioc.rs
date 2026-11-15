//! Indicators of compromise loaded from STIX2 bundles.
//!
//! Indicator files are never shipped in this repository; they are downloaded
//! from their original publishers at runtime.

use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::error::{Error, Result};
use crate::report::Detection;
use crate::timeline::TimelineEvent;

/// The kind of observable an indicator describes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IndicatorKind {
    /// A domain name.
    Domain,
    /// A full URL.
    Url,
    /// An IPv4 or IPv6 address.
    IpAddress,
    /// A process name.
    ProcessName,
    /// A file system path.
    FilePath,
    /// An e-mail address.
    Email,
    /// An app bundle identifier.
    AppId,
}

/// A single indicator.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Indicator {
    /// What kind of observable `value` is.
    pub kind: IndicatorKind,
    /// The observable to look for.
    pub value: String,
    /// Name of the campaign / malware this indicator belongs to.
    pub name: Option<String>,
    /// The file or URL the indicator was loaded from.
    pub origin: String,
}

/// A collection of loaded indicators.
#[derive(Debug, Clone, Default)]
pub struct IocSet {
    indicators: Vec<Indicator>,
}

impl IocSet {
    /// Creates an empty set.
    pub fn new() -> Self {
        Self::default()
    }

    /// Number of loaded indicators.
    pub fn len(&self) -> usize {
        self.indicators.len()
    }

    /// Returns `true` if no indicators are loaded.
    pub fn is_empty(&self) -> bool {
        self.indicators.is_empty()
    }

    /// The loaded indicators.
    pub fn indicators(&self) -> &[Indicator] {
        &self.indicators
    }

    /// Loads a STIX2 bundle from a file and adds its indicators.
    ///
    /// TODO: read with a size cap, then delegate to [`IocSet::load_stix2_str`].
    pub fn load_stix2_file(&mut self, path: &Path) -> Result<()> {
        let _ = path;
        Err(Error::NotImplemented("ioc::IocSet::load_stix2_file"))
    }

    /// Parses a STIX2 bundle (JSON) and adds its indicators.
    ///
    /// TODO: deserialize the bundle, keep `indicator` objects and translate
    /// their `pattern` (e.g. `[domain-name:value = '...']`) into
    /// [`Indicator`]s. Unknown or malformed patterns are skipped, not fatal.
    pub fn load_stix2_str(&mut self, json: &str, origin: &str) -> Result<()> {
        let _ = (json, origin);
        Err(Error::NotImplemented("ioc::IocSet::load_stix2_str"))
    }

    /// Checks timeline events against the loaded indicators.
    ///
    /// Deliberately returns an error rather than an empty list until it is
    /// implemented, so a placeholder can never produce a "no indicators
    /// found" result.
    pub fn check(&self, events: &[TimelineEvent]) -> Result<Vec<Detection>> {
        let _ = events;
        Err(Error::NotImplemented("ioc::IocSet::check"))
    }
}

/// Downloads the latest indicator files from their original sources.
///
/// TODO: decide the list of sources and the local storage directory, download
/// over HTTPS, and verify the files parse before replacing existing ones.
pub fn update_indicators() -> Result<()> {
    Err(Error::NotImplemented("ioc::update_indicators"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_set_is_empty() {
        let set = IocSet::new();
        assert!(set.is_empty());
        assert_eq!(set.len(), 0);
    }

    #[test]
    fn check_never_silently_reports_nothing() {
        let set = IocSet::new();
        assert!(matches!(set.check(&[]), Err(Error::NotImplemented(_))));
    }
}
