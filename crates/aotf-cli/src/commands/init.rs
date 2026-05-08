// Copyright 2026 Hieu Trung Dao
// SPDX-License-Identifier: Apache-2.0

//! `aotf init` — create the config file and a default watch directory.

use std::path::Path;

use anyhow::{Context, Result};

use crate::paths;

/// Default `config.toml` body. Walking-skeleton minimal config.
pub const DEFAULT_CONFIG: &str = r#"# AOTF configuration (v0.0.2-alpha)
# This file is read by the CLI and the daemon.

[watch]
# Directory to monitor for file events. Relative paths resolve against the
# directory that runs `aotf watch`.
dir = ".aotf-watch"

[notifications]
# Walking skeleton ships without notification adapters. Telegram + GitHub
# Actions adapters land in v0.0.3.
enabled = false
"#;

pub fn run(runtime_dir: &Path) -> Result<()> {
    // Ensure runtime + parent directories.
    std::fs::create_dir_all(runtime_dir)
        .with_context(|| format!("create runtime dir {runtime_dir:?}"))?;

    let config_path = paths::default_config_path();
    if let Some(parent) = config_path.parent() {
        std::fs::create_dir_all(parent).with_context(|| format!("create config dir {parent:?}"))?;
    }
    if !config_path.exists() {
        std::fs::write(&config_path, DEFAULT_CONFIG)
            .with_context(|| format!("write {config_path:?}"))?;
        println!("wrote {}", config_path.display());
    } else {
        println!("kept existing {}", config_path.display());
    }

    // Watch dir in CWD.
    let watch_dir = std::env::current_dir()
        .context("get current_dir")?
        .join(".aotf-watch");
    std::fs::create_dir_all(&watch_dir)
        .with_context(|| format!("create watch dir {watch_dir:?}"))?;
    // Drop a README so users know what the directory is for.
    let readme = watch_dir.join("README.md");
    if !readme.exists() {
        std::fs::write(
            &readme,
            "This directory is monitored by aotfd. \
             Files dropped here generate audit entries via the gatekeeper.\n",
        )
        .with_context(|| format!("write {readme:?}"))?;
    }
    println!("watch dir ready: {}", watch_dir.display());

    println!("runtime dir: {}", runtime_dir.display());
    println!("\nnext: run `aotf watch` to start the daemon.");
    Ok(())
}
