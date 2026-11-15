// Copyright (C) 2026 Trailoupe contributors
// SPDX-License-Identifier: GPL-3.0-or-later

//! Trailoupe command-line interface.

#![forbid(unsafe_code)]

use std::path::{Path, PathBuf};

use anyhow::Context;
use clap::{Parser, Subcommand};
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

fn main() -> anyhow::Result<()> {
    match Cli::parse().command {
        Command::ScanBackup { path } => run_scan(&path, backup::scan),
        Command::ScanSysdiagnose { path } => run_scan(&path, sysdiagnose::scan),
        Command::UpdateIocs => match ioc::update_indicators() {
            Ok(()) => Ok(()),
            Err(Error::NotImplemented(_)) => {
                println!("尚未實作");
                Ok(())
            }
            Err(e) => Err(e).context("failed to update indicators"),
        },
    }
}

fn run_scan(
    path: &Path,
    scan: fn(&Path) -> trailoupe_core::Result<Vec<TimelineEvent>>,
) -> anyhow::Result<()> {
    let events = match scan(path) {
        Ok(events) => events,
        Err(Error::NotImplemented(_)) => {
            println!("尚未實作");
            return Ok(());
        }
        Err(e) => return Err(e).with_context(|| format!("failed to scan {}", path.display())),
    };

    // TODO: load indicators and run `IocSet::check` to obtain detections.
    let report = Report::new(path.display().to_string(), events, Vec::new());
    report.write_json(std::io::stdout().lock())?;
    println!();
    Ok(())
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
}
