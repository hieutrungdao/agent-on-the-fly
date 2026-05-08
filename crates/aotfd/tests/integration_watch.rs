// Copyright 2026 Hieu Trung Dao
// SPDX-License-Identifier: Apache-2.0

//! In-process integration test for the watcher → finding pipeline.
//!
//! Story 9 covers the full subprocess-driven end-to-end demo via
//! `scripts/e2e_demo.sh`. Here we test only the watcher + Finding shape so
//! `cargo test` stays fast and deterministic.

use std::sync::mpsc::channel;
use std::time::Duration;

use aotf_core_types::{ActionTier, FindingSource};
use aotfd::watcher;
use tempfile::tempdir;

fn drain_until<F>(rx: &std::sync::mpsc::Receiver<aotf_core_types::Finding>, predicate: F) -> bool
where
    F: Fn(&aotf_core_types::Finding) -> bool,
{
    let deadline = std::time::Instant::now() + Duration::from_secs(3);
    while std::time::Instant::now() < deadline {
        match rx.recv_timeout(Duration::from_millis(200)) {
            Ok(f) => {
                if predicate(&f) {
                    return true;
                }
            }
            Err(std::sync::mpsc::RecvTimeoutError::Timeout) => continue,
            Err(std::sync::mpsc::RecvTimeoutError::Disconnected) => return false,
        }
    }
    false
}

#[test]
fn watcher_emits_finding_on_file_create() {
    let dir = tempdir().unwrap();
    let (tx, rx) = channel();
    let _watcher = watcher::spawn_watcher(dir.path(), tx).expect("spawn watcher");

    // Some platforms emit pre-existing-state events; let the watcher settle.
    std::thread::sleep(Duration::from_millis(100));

    let target = dir.path().join("hello.txt");
    std::fs::write(&target, b"hi").unwrap();

    let saw = drain_until(&rx, |f| {
        f.tier == ActionTier::Draft
            && f.source == FindingSource::FileTail
            && f.summary.contains("hello.txt")
    });
    assert!(saw, "expected a Finding for hello.txt within 3s");
}

#[test]
fn watcher_emits_finding_on_file_modify() {
    let dir = tempdir().unwrap();
    let (tx, rx) = channel();
    let _watcher = watcher::spawn_watcher(dir.path(), tx).expect("spawn watcher");
    std::thread::sleep(Duration::from_millis(100));

    let target = dir.path().join("touched.txt");
    std::fs::write(&target, b"v1").unwrap();
    std::thread::sleep(Duration::from_millis(50));
    std::fs::write(&target, b"v2-changed").unwrap();

    let saw = drain_until(&rx, |f| f.summary.contains("touched.txt"));
    assert!(saw, "expected a Finding for touched.txt within 3s");
}

#[test]
fn make_finding_has_uuid_v7_id_and_tier_draft() {
    let f = watcher::make_finding(std::path::Path::new("a/b.txt"));
    assert_eq!(f.tier, ActionTier::Draft);
    assert_eq!(f.id.0.len(), 36);
    assert_eq!(f.id.0.as_bytes()[14], b'7');
}
