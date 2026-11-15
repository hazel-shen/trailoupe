//! Unified timeline event structure shared by all analysis modules.

use serde::{Deserialize, Serialize};

/// Seconds since the Unix epoch (UTC).
///
/// Kept as a plain integer for now; a richer date-time type can be adopted
/// once the parsers need it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Timestamp(pub i64);

/// Where an event was extracted from.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EventSource {
    /// An iOS backup.
    Backup,
    /// A sysdiagnose archive.
    Sysdiagnose,
}

/// A single event on the timeline.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TimelineEvent {
    /// When the event happened, if the source records it.
    pub timestamp: Option<Timestamp>,
    /// Where the event was extracted from.
    pub source: EventSource,
    /// Human-readable description of the event.
    pub description: String,
}

impl TimelineEvent {
    /// Creates an event.
    pub fn new(
        timestamp: Option<Timestamp>,
        source: EventSource,
        description: impl Into<String>,
    ) -> Self {
        Self {
            timestamp,
            source,
            description: description.into(),
        }
    }
}

/// Sorts events chronologically. Events without a timestamp go last,
/// keeping their relative order.
pub fn sort_events(events: &mut [TimelineEvent]) {
    events.sort_by_key(|e| (e.timestamp.is_none(), e.timestamp));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sorts_chronologically_with_untimed_last() {
        let mut events = vec![
            TimelineEvent::new(None, EventSource::Backup, "untimed"),
            TimelineEvent::new(Some(Timestamp(20)), EventSource::Sysdiagnose, "later"),
            TimelineEvent::new(Some(Timestamp(10)), EventSource::Backup, "earlier"),
        ];
        sort_events(&mut events);
        let order: Vec<_> = events.iter().map(|e| e.description.as_str()).collect();
        assert_eq!(order, ["earlier", "later", "untimed"]);
    }

    #[test]
    fn serializes_to_json() {
        let event = TimelineEvent::new(Some(Timestamp(1)), EventSource::Sysdiagnose, "x");
        let json = serde_json::to_string(&event).unwrap();
        assert_eq!(
            json,
            r#"{"timestamp":1,"source":"sysdiagnose","description":"x"}"#
        );
    }
}
