// Copyright 2026 Hieu Trung Dao
// SPDX-License-Identifier: Apache-2.0

//! `aotf doctor` — runtime health check.

use std::path::{Path, PathBuf};

use anyhow::Result;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CheckResult {
    Ok(String),
    Warn(String),
    Fail(String),
}

pub fn run(runtime_dir: &Path) -> Result<()> {
    let checks = [
        ("rust toolchain", check_rust()),
        ("bun runtime", check_bun()),
        ("aotf-gatekeeper present", check_gatekeeper_present()),
        ("aotfd present", check_aotfd_present()),
        (
            "runtime dir writable",
            check_runtime_dir_writable(runtime_dir),
        ),
    ];

    let mut had_fail = false;
    for (name, result) in &checks {
        match result {
            CheckResult::Ok(msg) => println!("[ ok  ] {name}: {msg}"),
            CheckResult::Warn(msg) => println!("[warn ] {name}: {msg}"),
            CheckResult::Fail(msg) => {
                println!("[fail ] {name}: {msg}");
                had_fail = true;
            }
        }
    }

    if had_fail {
        anyhow::bail!("doctor reported failures; see above");
    }
    Ok(())
}

pub fn check_rust() -> CheckResult {
    // Walking skeleton: if we're running, we have a Rust binary built with at
    // least the MSRV. Report the compiler version that built this binary.
    let v = option_env!("CARGO_PKG_RUST_VERSION").unwrap_or("(unknown msrv)");
    CheckResult::Ok(format!("compiled against MSRV {v}"))
}

pub fn check_bun() -> CheckResult {
    match which("bun") {
        Some(path) => CheckResult::Ok(format!("found at {}", path.display())),
        None => CheckResult::Warn(
            "not found in PATH (Bun is required for the agent in v0.0.3+; not used in v0.0.2-alpha)"
                .into(),
        ),
    }
}

pub fn check_gatekeeper_present() -> CheckResult {
    match sibling_binary("aotf-gatekeeper") {
        Some(path) => CheckResult::Ok(format!("at {}", path.display())),
        None => CheckResult::Fail("no aotf-gatekeeper next to this binary".into()),
    }
}

pub fn check_aotfd_present() -> CheckResult {
    match sibling_binary("aotfd") {
        Some(path) => CheckResult::Ok(format!("at {}", path.display())),
        None => CheckResult::Fail("no aotfd next to this binary".into()),
    }
}

pub fn check_runtime_dir_writable(runtime_dir: &Path) -> CheckResult {
    if let Err(e) = std::fs::create_dir_all(runtime_dir) {
        return CheckResult::Fail(format!("cannot create {}: {e}", runtime_dir.display()));
    }
    let probe = runtime_dir.join(".doctor-write-probe");
    match std::fs::write(&probe, b"ok") {
        Ok(_) => {
            let _ = std::fs::remove_file(&probe);
            CheckResult::Ok(format!("{} writable", runtime_dir.display()))
        }
        Err(e) => CheckResult::Fail(format!("write probe failed: {e}")),
    }
}

fn which(cmd: &str) -> Option<PathBuf> {
    let path = std::env::var_os("PATH")?;
    for dir in std::env::split_paths(&path) {
        let candidate = dir.join(cmd);
        if candidate.is_file() {
            return Some(candidate);
        }
    }
    None
}

fn sibling_binary(name: &str) -> Option<PathBuf> {
    let here = std::env::current_exe().ok()?;
    let dir = here.parent()?;
    let candidate = dir.join(if cfg!(windows) {
        format!("{name}.exe")
    } else {
        name.to_string()
    });
    candidate.is_file().then_some(candidate)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn check_runtime_dir_writable_ok_for_temp() {
        let dir = tempdir().unwrap();
        match check_runtime_dir_writable(dir.path()) {
            CheckResult::Ok(_) => {}
            other => panic!("expected Ok, got {other:?}"),
        }
    }

    #[test]
    fn check_rust_returns_ok() {
        match check_rust() {
            CheckResult::Ok(_) => {}
            other => panic!("expected Ok, got {other:?}"),
        }
    }
}
