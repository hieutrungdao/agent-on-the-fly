// Copyright 2026 Hieu Trung Dao
// SPDX-License-Identifier: Apache-2.0

//! File-system watcher: emits a [`Finding`] per `Create` / `Modify` event in
//! the watched directory.
//!
//! Walking-skeleton scope: file-tail / Docker-stdout sources only (FR-09).
//! The producer here is the file-tail flavor; Docker stdout watcher is a
//! v0.0.3 addition.

use std::path::{Path, PathBuf};
use std::sync::mpsc::Sender;
use std::time::{SystemTime, UNIX_EPOCH};

use anyhow::Result;
use aotf_core_types::{ActionTier, Finding, FindingId, FindingSource};
use notify::{Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher};

/// Build a [`Finding`] for a file-system event at `path`.
pub fn make_finding(path: &Path) -> Finding {
    let display = path.display().to_string();
    let summary = if display.len() <= 180 {
        format!("file event: {display}")
    } else {
        let head: String = display.chars().take(120).collect();
        format!("file event: {head}…")
    };
    Finding {
        id: FindingId::new(),
        tier: ActionTier::Draft,
        source: FindingSource::FileTail,
        summary,
        payload: serde_json::json!({"path": display}),
        created_at_micros: now_micros(),
        lamport: 0,
        confidence: 0.0,
    }
}

fn now_micros() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_micros() as i64)
        .unwrap_or(0)
}

/// Spawn a recommended-platform watcher on `dir`, sending one [`Finding`]
/// per relevant file-system event to `findings_tx`.
///
/// The returned [`RecommendedWatcher`] **must be kept alive by the caller**
/// — dropping it stops the watch.
pub fn spawn_watcher(dir: &Path, findings_tx: Sender<Finding>) -> Result<RecommendedWatcher> {
    let dir_owned: PathBuf = dir.to_path_buf();
    let mut watcher = notify::recommended_watcher(move |res: notify::Result<Event>| match res {
        Ok(ev) => {
            if !is_relevant_kind(&ev.kind) {
                return;
            }
            for p in ev.paths {
                // Filter out events that are not children of the watched dir
                // (some platforms emit on the dir itself).
                if p == dir_owned {
                    continue;
                }
                let finding = make_finding(&p);
                if findings_tx.send(finding).is_err() {
                    // Receiver gone; nothing else to do, the watcher will be
                    // dropped by its owner shortly.
                    return;
                }
            }
        }
        Err(e) => tracing::warn!(error = %e, "watcher error"),
    })?;
    watcher.watch(dir, RecursiveMode::Recursive)?;
    Ok(watcher)
}

fn is_relevant_kind(k: &EventKind) -> bool {
    matches!(k, EventKind::Create(_) | EventKind::Modify(_))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn make_finding_truncates_long_paths() {
        let long = "a".repeat(500);
        let f = make_finding(Path::new(&long));
        f.validate().expect("validates");
        assert_eq!(f.tier, ActionTier::Draft);
        assert_eq!(f.source, FindingSource::FileTail);
    }

    #[test]
    fn make_finding_short_path_keeps_full_display() {
        let f = make_finding(Path::new("short.txt"));
        assert!(f.summary.contains("short.txt"));
    }
}
