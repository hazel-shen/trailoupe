//! Trailoupe core library.
//!
//! Analyses iOS backups and sysdiagnose archives and compares the extracted
//! artefacts against public indicators of compromise (STIX2).
//!
//! All input (backups, archives, indicator files) is treated as untrusted:
//! parsing code must never panic and never use `unsafe`.
//!
//! A scan can only ever conclude that *no known indicators were found*; it
//! can never conclude that a device is safe. See [`report::Verdict`].

#![forbid(unsafe_code)]

pub mod backup;
pub mod error;
pub mod ioc;
pub mod report;
pub mod sysdiagnose;
pub mod timeline;

pub use error::{Error, Result};
