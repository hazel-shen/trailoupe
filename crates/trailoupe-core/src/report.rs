//! JSON scan reports.
//!
//! A report can only state that known indicators were or were not found.
//! There is intentionally no verdict meaning "the device is safe": absence of
//! known indicators does not prove absence of spyware.

use std::io::Write;
use std::num::NonZeroUsize;

use serde::{Deserialize, Serialize};

use crate::error::Result;
use crate::ioc::{CheckOutcome, Indicator};
use crate::timeline::TimelineEvent;

/// Statement included in every report, whatever the verdict.
pub const DISCLAIMER: &str = "Public indicators cannot prove that a device has not been \
     compromised. Spyware that is unknown or not covered by the loaded indicators will not \
     be detected.";

/// A timeline event that matched an indicator.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Detection {
    /// The indicator that matched.
    pub indicator: Indicator,
    /// The event it matched.
    pub event: TimelineEvent,
}

/// The outcome of a scan.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Verdict {
    /// None of the loaded indicators matched.
    NoKnownIndicatorsFound,
    /// At least one loaded indicator matched.
    KnownIndicatorsFound,
}

impl Verdict {
    /// Human-readable summary of the verdict.
    pub fn summary(self) -> &'static str {
        match self {
            Verdict::NoKnownIndicatorsFound => "No known indicators were found.",
            Verdict::KnownIndicatorsFound => {
                "Matches with known indicators were found. Review the detections below."
            }
        }
    }
}

/// A complete scan report.
///
/// A report can only be built from a [`CheckOutcome`], so it always reflects
/// a check that actually ran against at least one indicator. Fields are
/// private so that the verdict always matches the detections and the
/// [`DISCLAIMER`] cannot be removed.
#[derive(Debug, Clone, Serialize)]
pub struct Report {
    tool_version: &'static str,
    input: String,
    indicator_count: NonZeroUsize,
    verdict: Verdict,
    summary: &'static str,
    disclaimer: &'static str,
    detections: Vec<Detection>,
    events: Vec<TimelineEvent>,
}

impl Report {
    /// Builds a report from the outcome of
    /// [`IocSet::check`](crate::ioc::IocSet::check); the verdict is derived
    /// from its detections.
    pub fn new(input: impl Into<String>, outcome: CheckOutcome) -> Self {
        let (indicator_count, events, detections) = outcome.into_parts();
        let verdict = if detections.is_empty() {
            Verdict::NoKnownIndicatorsFound
        } else {
            Verdict::KnownIndicatorsFound
        };
        Self {
            tool_version: env!("CARGO_PKG_VERSION"),
            input: input.into(),
            indicator_count,
            verdict,
            summary: verdict.summary(),
            disclaimer: DISCLAIMER,
            detections,
            events,
        }
    }

    /// Description of what was scanned (e.g. the input path).
    pub fn input(&self) -> &str {
        &self.input
    }

    /// Number of indicators the events were checked against.
    pub fn indicator_count(&self) -> NonZeroUsize {
        self.indicator_count
    }

    /// The outcome of the scan.
    pub fn verdict(&self) -> Verdict {
        self.verdict
    }

    /// Events that matched an indicator.
    pub fn detections(&self) -> &[Detection] {
        &self.detections
    }

    /// All extracted events.
    pub fn events(&self) -> &[TimelineEvent] {
        &self.events
    }

    /// Serializes the report as pretty-printed JSON.
    pub fn to_json(&self) -> Result<String> {
        Ok(serde_json::to_string_pretty(self)?)
    }

    /// Writes the report as pretty-printed JSON.
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

    fn detection() -> Detection {
        Detection {
            indicator: Indicator {
                kind: IndicatorKind::Domain,
                value: "example.invalid".into(),
                name: None,
                origin: "test".into(),
            },
            event: TimelineEvent::new(None, EventSource::Backup, "visited example.invalid"),
        }
    }

    fn outcome(detections: Vec<Detection>) -> CheckOutcome {
        let events = vec![TimelineEvent::new(None, EventSource::Backup, "event")];
        CheckOutcome::for_test(NonZeroUsize::MIN, events, detections)
    }

    #[test]
    fn empty_report_never_claims_safety() {
        let report = Report::new("backup", outcome(Vec::new()));
        assert_eq!(report.verdict(), Verdict::NoKnownIndicatorsFound);
        let json = report.to_json().unwrap().to_lowercase();
        assert!(json.contains("no_known_indicators_found"));
        for word in ["\"safe\"", "is safe", "clean"] {
            assert!(!json.contains(word), "report must not contain {word:?}");
        }
    }

    #[test]
    fn detections_set_verdict() {
        let report = Report::new("backup", outcome(vec![detection()]));
        assert_eq!(report.verdict(), Verdict::KnownIndicatorsFound);
        assert_eq!(report.detections().len(), 1);
    }

    #[test]
    fn carries_checked_events_and_indicator_count() {
        let events = vec![TimelineEvent::new(None, EventSource::Backup, "event")];
        let checked = CheckOutcome::for_test(NonZeroUsize::new(3).unwrap(), events, Vec::new());
        let report = Report::new("backup", checked);
        assert_eq!(report.indicator_count().get(), 3);
        assert_eq!(report.events().len(), 1);
        let json: serde_json::Value = serde_json::from_str(&report.to_json().unwrap()).unwrap();
        assert_eq!(json["indicator_count"], 3);
    }

    #[test]
    fn every_report_carries_disclaimer() {
        for detections in [Vec::new(), vec![detection()]] {
            let report = Report::new("backup", outcome(detections));
            let json: serde_json::Value = serde_json::from_str(&report.to_json().unwrap()).unwrap();
            assert_eq!(json["disclaimer"], DISCLAIMER);
        }
    }
}
