// Copyright 2026 Hieu Trung Dao
// SPDX-License-Identifier: Apache-2.0

//! `aotfd` — the AOTF daemon. Walking-skeleton orchestration:
//! 1. Open audit DB.
//! 2. Spawn `aotf-gatekeeper` as a child process.
//! 3. Watch a directory; on each file event, ask the gatekeeper to evaluate
//!    a `DRAFT`-tier action and let the gatekeeper write the audit entry.
//! 4. Serve `daemon.ping` and `audit.list` over a CLI socket.

use std::path::PathBuf;
use std::sync::mpsc::{Receiver, channel};
use std::time::{Duration, Instant};

use anyhow::{Context, Result, bail};
use aotf_core_types::{ActionTier, Finding};
use aotf_ipc_protocol::{JsonRpcRequest, methods};
use clap::Parser;
use serde_json::json;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::UnixStream;

#[derive(Parser, Debug)]
#[command(version, about = "AOTF daemon")]
struct Cli {
    /// Directory to watch for file-system events.
    #[arg(long, default_value = ".aotf-watch")]
    watch_dir: PathBuf,
    /// Runtime directory for sockets, audit db, and lock files.
    /// Defaults to `~/.aotf/run`.
    #[arg(long)]
    runtime_dir: Option<PathBuf>,
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "info,aotfd=debug".into()),
        )
        .json()
        .init();

    let cli = Cli::parse();
    let runtime_dir = cli.runtime_dir.unwrap_or_else(default_runtime_dir);
    std::fs::create_dir_all(&runtime_dir)
        .with_context(|| format!("create runtime dir {runtime_dir:?}"))?;
    std::fs::create_dir_all(&cli.watch_dir)
        .with_context(|| format!("create watch dir {:?}", cli.watch_dir))?;

    let gate_socket = runtime_dir.join("gate.sock");
    let cli_socket = runtime_dir.join("cli.sock");
    let audit_db = runtime_dir.join("audit.db");

    // Spawn the gatekeeper.
    let gatekeeper_path = locate_gatekeeper()?;
    tracing::info!(path = %gatekeeper_path.display(), "spawning gatekeeper");
    let mut gatekeeper = tokio::process::Command::new(&gatekeeper_path)
        .arg("--socket")
        .arg(&gate_socket)
        .arg("--audit-db")
        .arg(&audit_db)
        .kill_on_drop(true)
        .spawn()
        .with_context(|| format!("spawn {gatekeeper_path:?}"))?;

    // Wait for the gatekeeper socket to appear.
    wait_for_socket(&gate_socket, Duration::from_secs(5)).await?;

    // Set up the watcher → mpsc channel.
    let (findings_tx, findings_rx) = channel::<Finding>();
    let _watcher = aotfd::watcher::spawn_watcher(&cli.watch_dir, findings_tx)
        .with_context(|| format!("watch {:?}", cli.watch_dir))?;
    tracing::info!(dir = %cli.watch_dir.display(), "watching");

    // Drain findings into gate.evaluate calls in a blocking task.
    let gate_socket_for_task = gate_socket.clone();
    let gate_task =
        tokio::task::spawn(async move { drain_findings(findings_rx, gate_socket_for_task).await });

    // Serve the CLI socket.
    let cli_task = tokio::spawn(aotfd::ipc_server::serve(cli_socket, audit_db));

    tracing::info!("aotfd ready");

    let mut sigterm = tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
        .context("install SIGTERM handler")?;

    tokio::select! {
        _ = tokio::signal::ctrl_c() => {
            tracing::info!("SIGINT received, shutting down");
        }
        _ = sigterm.recv() => {
            tracing::info!("SIGTERM received, shutting down");
        }
        r = gatekeeper.wait() => {
            tracing::warn!(status = ?r, "gatekeeper exited");
        }
        r = gate_task => {
            tracing::warn!(result = ?r, "gate-drain task ended");
        }
        r = cli_task => {
            tracing::warn!(result = ?r, "cli task ended");
        }
    }

    let _ = gatekeeper.start_kill();
    Ok(())
}

fn default_runtime_dir() -> PathBuf {
    if let Ok(home) = std::env::var("HOME") {
        PathBuf::from(home).join(".aotf").join("run")
    } else {
        PathBuf::from(".aotf/run")
    }
}

/// Find the `aotf-gatekeeper` binary. We expect it to live next to this
/// daemon binary (cargo target dir, `~/.cargo/bin`, or the user install dir).
fn locate_gatekeeper() -> Result<PathBuf> {
    let here = std::env::current_exe().context("locate current_exe")?;
    let dir = here
        .parent()
        .ok_or_else(|| anyhow::anyhow!("current_exe has no parent"))?;
    let candidate = dir.join(if cfg!(windows) {
        "aotf-gatekeeper.exe"
    } else {
        "aotf-gatekeeper"
    });
    if candidate.is_file() {
        Ok(candidate)
    } else {
        bail!("could not find aotf-gatekeeper next to {}", here.display())
    }
}

async fn wait_for_socket(path: &std::path::Path, timeout: Duration) -> Result<()> {
    let started = Instant::now();
    while started.elapsed() < timeout {
        if path.exists() {
            // Try a quick connect to confirm the listener is bound.
            if UnixStream::connect(path).await.is_ok() {
                return Ok(());
            }
        }
        tokio::time::sleep(Duration::from_millis(20)).await;
    }
    bail!(
        "socket {} did not appear within {:?}",
        path.display(),
        timeout
    )
}

async fn drain_findings(rx: Receiver<Finding>, gate_socket: PathBuf) -> Result<()> {
    // The receiver is std::sync::mpsc::Receiver, so we recv on a blocking
    // thread to keep the async runtime free.
    let (async_tx, mut async_rx) = tokio::sync::mpsc::unbounded_channel::<Finding>();
    std::thread::spawn(move || {
        while let Ok(f) = rx.recv() {
            if async_tx.send(f).is_err() {
                break;
            }
        }
    });

    while let Some(finding) = async_rx.recv().await {
        if let Err(e) = call_gate_evaluate(&gate_socket, &finding).await {
            tracing::warn!(error = %e, finding = %finding.id, "gate.evaluate failed");
        }
    }
    Ok(())
}

async fn call_gate_evaluate(socket: &std::path::Path, finding: &Finding) -> Result<()> {
    let stream = UnixStream::connect(socket).await?;
    let (read_half, mut write_half) = stream.into_split();
    let mut lines = BufReader::new(read_half).lines();

    let req = JsonRpcRequest::new(
        methods::GATE_EVALUATE,
        json!({
            "tier": tier_wire(finding.tier),
            "findingId": finding.id.0,
            "reasonHint": finding.summary,
        }),
        json!(1),
    );
    let mut bytes = serde_json::to_vec(&req)?;
    bytes.push(b'\n');
    write_half.write_all(&bytes).await?;

    // Single-response per connection for now.
    if let Some(line) = lines.next_line().await? {
        tracing::debug!(response = %line, "gatekeeper response");
    }
    Ok(())
}

fn tier_wire(t: ActionTier) -> &'static str {
    t.as_wire_str()
}
