// Copyright (C) 2026 Trailoupe contributors
// SPDX-License-Identifier: GPL-3.0-or-later

//! Trailoupe command-line interface.

#![forbid(unsafe_code)]

use std::path::{Path, PathBuf};
use std::process::ExitCode;

use anyhow::Context;
use clap::{Parser, Subcommand};
use trailoupe_core::ioc::IocSet;
use trailoupe_core::report::Report;
use trailoupe_core::timeline::TimelineEvent;
use trailoupe_core::{Error, backup, ioc, sysdiagnose};

/// Check your own iPhone backups and sysdiagnose archives for known spyware
/// indicators. Finding nothing only means no *known* indicators were found.
#[derive(Debug, Parser)]
#[command(name = "trailoupe", version, about)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Scan an iOS backup directory
    ScanBackup {
        /// Path to the backup directory (containing Manifest.db)
        path: PathBuf,
    },
    /// Scan a sysdiagnose archive (.tar.gz)
    ScanSysdiagnose {
        /// Path to the sysdiagnose .tar.gz archive
        path: PathBuf,
    },
    /// Download the latest indicator files from their original sources
    UpdateIocs,
}

fn main() -> ExitCode {
    match run(Cli::parse().command) {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            match not_implemented(&err) {
                Some(feature) => eprintln!("尚未實作：{feature}"),
                None => eprintln!("錯誤：{err:#}"),
            }
            ExitCode::FAILURE
        }
    }
}

fn run(command: Command) -> anyhow::Result<()> {
    match command {
        Command::ScanBackup { path } => run_scan(&path, backup::scan),
        Command::ScanSysdiagnose { path } => run_scan(&path, sysdiagnose::scan),
        Command::UpdateIocs => ioc::update_indicators().context("failed to update indicators"),
    }
}

fn run_scan(
    path: &Path,
    scan: fn(&Path) -> trailoupe_core::Result<Vec<TimelineEvent>>,
) -> anyhow::Result<()> {
    let report =
        build_report(path, scan).with_context(|| format!("failed to scan {}", path.display()))?;
    report.write_json(std::io::stdout().lock())?;
    println!();
    Ok(())
}

/// Returns the feature name if `err` was caused by a placeholder.
fn not_implemented(err: &anyhow::Error) -> Option<&'static str> {
    err.chain()
        .find_map(|cause| match cause.downcast_ref::<Error>() {
            Some(Error::NotImplemented(feature)) => Some(*feature),
            _ => None,
        })
}

/// Loads the indicators, scans `path` and checks the events against them.
fn build_report(
    path: &Path,
    scan: fn(&Path) -> trailoupe_core::Result<Vec<TimelineEvent>>,
) -> trailoupe_core::Result<Report> {
    // Load first so a missing indicator set fails before a long scan.
    let iocs = IocSet::load_installed()?;
    let events = scan(path)?;
    let outcome = iocs.check(events)?;
    Ok(Report::new(path.display().to_string(), outcome))
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::CommandFactory;

    #[test]
    fn cli_definition_is_valid() {
        Cli::command().debug_assert();
    }

    #[test]
    fn parses_scan_backup() {
        let cli = Cli::try_parse_from(["trailoupe", "scan-backup", "/tmp/backup"]).unwrap();
        assert!(
            matches!(cli.command, Command::ScanBackup { path } if path == Path::new("/tmp/backup"))
        );
    }

    #[test]
    fn parses_update_iocs() {
        let cli = Cli::try_parse_from(["trailoupe", "update-iocs"]).unwrap();
        assert!(matches!(cli.command, Command::UpdateIocs));
    }

    #[test]
    fn detects_not_implemented_through_context() {
        let err = anyhow::Error::from(Error::NotImplemented("backup::scan")).context("failed");
        assert_eq!(not_implemented(&err), Some("backup::scan"));
    }

    #[test]
    fn other_errors_are_not_reported_as_not_implemented() {
        let err = anyhow::Error::from(Error::NoIndicatorsLoaded).context("failed");
        assert_eq!(not_implemented(&err), None);
    }
}
