use crate::settings::SettingsHandle;
use anyhow::{Error, Result};
use chrono::Utc;
use std::fs;
use std::path::PathBuf;

/// Write a local, human-readable report for startup failures.
///
/// The report contains no profile contents, samples, credentials, or network tokens. It is kept
/// beside the regular logs so a disappearing UI is still diagnosable after the process exits.
pub async fn write_startup_report(settings: &SettingsHandle, error: &Error) -> Result<PathBuf> {
    let log_directory = settings.get_log_directory().await;
    fs::create_dir_all(&log_directory)?;

    let timestamp = Utc::now();
    let report_path = log_directory.join(format!(
        "startup-diagnostic-{}.txt",
        timestamp.format("%Y%m%dT%H%M%SZ")
    ));
    let profile_directory = settings.get_profile_directory().await;
    let mic_profile_directory = settings.get_mic_profile_directory().await;
    let backup_directory = settings.get_backup_directory().await;

    let report = format!(
        "GoXLR Utility startup diagnostic\n\
         Generated: {}\n\
         Utility version: {}\n\
         Operating system: {}\n\
         Architecture: {}\n\
         Profile directory: {}\n\
         Mic profile directory: {}\n\
         Backup directory: {}\n\
         Log directory: {}\n\
         \nError chain:\n{:#}\n",
        timestamp.to_rfc3339(),
        env!("CARGO_PKG_VERSION"),
        std::env::consts::OS,
        std::env::consts::ARCH,
        profile_directory.display(),
        mic_profile_directory.display(),
        backup_directory.display(),
        log_directory.display(),
        error,
    );
    fs::write(&report_path, report)?;
    Ok(report_path)
}
