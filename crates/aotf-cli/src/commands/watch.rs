// Copyright 2026 Hieu Trung Dao
// SPDX-License-Identifier: Apache-2.0

//! `aotf watch` — foreground-spawn the daemon and stream its logs until
//! ctrl-c.

use std::path::Path;
use std::process::Stdio;

use anyhow::{Context, Result, bail};
use tokio::io::{AsyncBufReadExt, BufReader};

pub async fn run(runtime_dir: &Path, watch_dir: &Path) -> Result<()> {
    let aotfd = locate_sibling("aotfd")
        .ok_or_else(|| anyhow::anyhow!("could not find aotfd next to this binary"))?;

    std::fs::create_dir_all(watch_dir)
        .with_context(|| format!("create watch dir {watch_dir:?}"))?;
    std::fs::create_dir_all(runtime_dir)
        .with_context(|| format!("create runtime dir {runtime_dir:?}"))?;

    let mut child = tokio::process::Command::new(&aotfd)
        .arg("--watch-dir")
        .arg(watch_dir)
        .arg("--runtime-dir")
        .arg(runtime_dir)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .kill_on_drop(true)
        .spawn()
        .with_context(|| format!("spawn {}", aotfd.display()))?;

    let pid = child.id().unwrap_or(0);
    println!(
        "aotfd started (pid {}). watching {}. ctrl-c to stop.",
        pid,
        watch_dir.display()
    );

    let stdout = child.stdout.take().context("daemon stdout")?;
    let stderr = child.stderr.take().context("daemon stderr")?;
    let stdout_task = tokio::spawn(forward_lines(stdout, "aotfd"));
    let stderr_task = tokio::spawn(forward_lines(stderr, "aotfd!"));

    let exit = tokio::select! {
        _ = tokio::signal::ctrl_c() => {
            println!("\nctrl-c — stopping daemon.");
            let _ = child.start_kill();
            child.wait().await
        }
        r = child.wait() => r,
    };

    let _ = stdout_task.await;
    let _ = stderr_task.await;

    let status = exit.context("await daemon exit")?;
    if !status.success() && !matches!(status.code(), None | Some(0)) {
        bail!("daemon exited with {status}");
    }
    Ok(())
}

async fn forward_lines<R: tokio::io::AsyncRead + Unpin>(reader: R, tag: &'static str) {
    let mut lines = BufReader::new(reader).lines();
    while let Ok(Some(line)) = lines.next_line().await {
        println!("[{tag}] {line}");
    }
}

fn locate_sibling(name: &str) -> Option<std::path::PathBuf> {
    let here = std::env::current_exe().ok()?;
    let dir = here.parent()?;
    let candidate = dir.join(if cfg!(windows) {
        format!("{name}.exe")
    } else {
        name.to_string()
    });
    candidate.is_file().then_some(candidate)
}
