use anyhow::{Context, Result};
use chrono::Utc;
use log::{debug, warn};
use std::fs;
use std::path::{Path, PathBuf};

const SNAPSHOT_LIMIT_PER_PROFILE: usize = 20;

#[derive(Clone, Copy)]
pub enum SnapshotKind {
    Device,
    Microphone,
}

impl SnapshotKind {
    fn directory(self) -> &'static str {
        match self {
            Self::Device => "profiles",
            Self::Microphone => "mic-profiles",
        }
    }

    fn extension(self) -> &'static str {
        match self {
            Self::Device => "goxlr",
            Self::Microphone => "goxlrMicProfile",
        }
    }
}

/// Preserve the on-disk profile before a destructive operation.
///
/// Snapshots live below the configured backup directory and are deliberately separate from the
/// legacy single-file recovery backup. The latest 20 snapshots are retained for each profile.
pub fn create_profile_snapshot(
    source_directory: &Path,
    backup_directory: &Path,
    profile_name: &str,
    kind: SnapshotKind,
    reason: &str,
) -> Result<Option<PathBuf>> {
    let source = source_directory.join(format!("{}.{}", profile_name, kind.extension()));
    if !source.is_file() {
        return Ok(None);
    }

    let safe_name = safe_component(profile_name);
    let snapshot_directory = backup_directory
        .join("snapshots")
        .join(kind.directory())
        .join(&safe_name);
    fs::create_dir_all(&snapshot_directory).with_context(|| {
        format!(
            "Unable to create snapshot directory {}",
            snapshot_directory.display()
        )
    })?;

    let timestamp = Utc::now().format("%Y%m%dT%H%M%S%6fZ");
    let destination = snapshot_directory.join(format!(
        "{}-{}-{}.{}",
        timestamp,
        safe_component(reason),
        safe_name,
        kind.extension()
    ));
    fs::copy(&source, &destination).with_context(|| {
        format!(
            "Unable to snapshot {} to {}",
            source.display(),
            destination.display()
        )
    })?;

    if let Err(error) = prune_snapshots(&snapshot_directory) {
        warn!("Unable to prune old profile snapshots: {error:#}");
    }
    debug!("Profile snapshot created at {}", destination.display());
    Ok(Some(destination))
}

fn safe_component(value: &str) -> String {
    let safe: String = value
        .chars()
        .map(|character| {
            if character.is_ascii_alphanumeric() || matches!(character, '-' | '_' | '.') {
                character
            } else {
                '_'
            }
        })
        .collect();
    if safe.is_empty() || safe.chars().all(|character| character == '.') {
        "unnamed".to_string()
    } else {
        safe
    }
}

fn prune_snapshots(directory: &Path) -> Result<()> {
    let mut snapshots: Vec<_> = fs::read_dir(directory)?
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.path().is_file())
        .collect();
    snapshots.sort_by_key(|entry| entry.file_name());

    let remove_count = snapshots.len().saturating_sub(SNAPSHOT_LIMIT_PER_PROFILE);
    for snapshot in snapshots.into_iter().take(remove_count) {
        fs::remove_file(snapshot.path())?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn test_directory() -> PathBuf {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        std::env::temp_dir().join(format!("goxlr-snapshot-test-{unique}"))
    }

    #[test]
    fn creates_a_versioned_snapshot() {
        let root = test_directory();
        let profiles = root.join("profiles");
        let backups = root.join("backups");
        fs::create_dir_all(&profiles).unwrap();
        fs::write(profiles.join("Streaming.goxlr"), b"profile-data").unwrap();

        let snapshot = create_profile_snapshot(
            &profiles,
            &backups,
            "Streaming",
            SnapshotKind::Device,
            "before-save",
        )
        .unwrap()
        .unwrap();

        assert_eq!(fs::read(snapshot).unwrap(), b"profile-data");
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn missing_profiles_are_not_errors() {
        let root = test_directory();
        let snapshot = create_profile_snapshot(
            &root.join("profiles"),
            &root.join("backups"),
            "Missing",
            SnapshotKind::Microphone,
            "before-save",
        )
        .unwrap();
        assert!(snapshot.is_none());
    }

    #[test]
    fn dot_only_names_cannot_escape_the_snapshot_directory() {
        assert_eq!(safe_component(".."), "unnamed");
        assert_eq!(safe_component("../Streaming"), ".._Streaming");
    }
}
