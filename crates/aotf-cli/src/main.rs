// Copyright 2026 Hieu Trung Dao
// SPDX-License-Identifier: Apache-2.0

//! `aotf` — the user-facing CLI. Walking-skeleton commands:
//! - `aotf init` — create `~/.aotf/config.toml` and `.aotf-watch/` in CWD.
//! - `aotf watch` — spawn `aotfd` and stream its logs until ctrl-c.
//! - `aotf audit ls` — open the audit DB read-only and print recent entries.
//! - `aotf doctor` — check toolchain prerequisites and runtime paths.

use std::path::PathBuf;

use anyhow::Result;
use clap::{Parser, Subcommand};

mod commands;
mod paths;

#[derive(Parser, Debug)]
#[command(
    name = "aotf",
    version,
    about = "Agent on the Fly — proactive multi-agent SDLC companion"
)]
struct Cli {
    /// Override the runtime directory (defaults to `~/.aotf/run`).
    #[arg(long, global = true)]
    runtime_dir: Option<PathBuf>,

    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// Initialize an AOTF workspace in the current directory.
    Init,
    /// Spawn the daemon and stream its logs to this terminal until ctrl-c.
    Watch {
        /// Directory the daemon should watch (defaults to `./.aotf-watch`).
        #[arg(long, default_value = ".aotf-watch")]
        dir: PathBuf,
    },
    /// Audit log inspection.
    Audit {
        #[command(subcommand)]
        command: AuditCommand,
    },
    /// Check toolchain prerequisites and runtime paths.
    Doctor,
}

#[derive(Subcommand, Debug)]
enum AuditCommand {
    /// List recent audit entries (newest first).
    Ls {
        /// Maximum number of entries to print (default: 20).
        #[arg(long, default_value_t = 20)]
        limit: usize,
        /// Only show entries with id > since.
        #[arg(long)]
        since: Option<u64>,
    },
}

fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "warn,aotf=info".into()),
        )
        .with_target(false)
        .compact()
        .init();

    let cli = Cli::parse();
    let runtime = cli.runtime_dir.unwrap_or_else(paths::default_runtime_dir);

    match cli.command {
        Command::Init => commands::init::run(&runtime),
        Command::Watch { dir } => {
            // Watch is foreground & blocking; run inside a tokio runtime.
            let rt = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()?;
            rt.block_on(commands::watch::run(&runtime, &dir))
        }
        Command::Audit {
            command: AuditCommand::Ls { limit, since },
        } => commands::audit::ls(&runtime, limit, since),
        Command::Doctor => commands::doctor::run(&runtime),
    }
}
