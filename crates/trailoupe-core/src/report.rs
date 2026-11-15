//! JSON scan reports.
//!
//! A report can only state that known indicators were or were not found.
//! There is intentionally no verdict meaning "the device is safe": absence of
//! known indicators does not prove absence of spyware.

use std::io::Write;

use serde::{Deserialize, Serialize};

use crate::error::Result;
use crate::ioc::Indicator;
use crate::timeline::TimelineEvent;

/// A timeline event that matched an indicator.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Detection {
    pub indicator: Indicator,
    pub event: TimelineEvent,
}

/// The outcome of a scan.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Verdict {
    NoKnownIndicatorsFound,
    KnownIndicatorsFound,
}

impl Verdict {
    /// Human-readable summary of the verdict.
    pub fn summary(self) -> &'static str {
        match self {
            Verdict::NoKnownIndicatorsFound => {
                "No known indicators were found. This does not mean the device is free of spyware."
            }
            Verdict::KnownIndicatorsFound => {
                "Matches with known indicators were found. Review the detections below."
            }
        }
    }
}

/// A complete scan report.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Report {
    pub tool_version: String,
    /// Description of what was scanned (e.g. the input path).
    pub input: String,
    pub verdict: Verdict,
    pub summary: String,
    pub detections: Vec<Detection>,
    pub events: Vec<TimelineEvent>,
}

impl Report {
    /// Builds a report; the verdict is derived from `detections`.
    pub fn new(
        input: impl Into<String>,
        events: Vec<TimelineEvent>,
        detections: Vec<Detection>,
    ) -> Self {
        let verdict = if detections.is_empty() {
            Verdict::NoKnownIndicatorsFound
        } else {
            Verdict::KnownIndicatorsFound
        };
        Self {
            tool_version: env!("CARGO_PKG_VERSION").to_owned(),
            input: input.into(),
            verdict,
            summary: verdict.summary().to_owned(),
            detections,
            events,
        }
    }

    pub fn to_json(&self) -> Result<String> {
        Ok(serde_json::to_string_pretty(self)?)
    }

    pub fn write_json<W: Write>(&self, writer: W) -> Result<()> {
        serde_json::to_writer_pretty(writer, self)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ioc::IndicatorKind;
    use crate::timeline::EventSource;

    #[test]
    fn empty_report_never_claims_safety() {
        let report = Report::new("backup", Vec::new(), Vec::new());
        assert_eq!(report.verdict, Verdict::NoKnownIndicatorsFound);
        let json = report.to_json().unwrap().to_lowercase();
        assert!(json.contains("no_known_indicators_found"));
        for word in ["\"safe\"", "is safe", "clean"] {
            assert!(!json.contains(word), "report must not contain {word:?}");
        }
    }

    #[test]
    fn detections_set_verdict() {
        let detection = Detection {
            indicator: Indicator {
                kind: IndicatorKind::Domain,
                value: "example.invalid".into(),
                name: None,
                origin: "test".into(),
            },
            event: TimelineEvent::new(None, EventSource::Backup, "visited example.invalid"),
        };
        let report = Report::new("backup", Vec::new(), vec![detection]);
        assert_eq!(report.verdict, Verdict::KnownIndicatorsFound);
    }
}
