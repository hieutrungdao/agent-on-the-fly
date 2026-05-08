#!/usr/bin/env bash
# Copyright 2026 Hieu Trung Dao
# SPDX-License-Identifier: Apache-2.0
#
# End-to-end demo for the AOTF walking skeleton.
#
# 1. Build release binaries.
# 2. `aotf init` in a tempdir (HOME-isolated).
# 3. Spawn `aotfd` in the background.
# 4. Touch a file in the watched directory.
# 5. Poll `aotf audit ls` for ≤5s and require a row with `DRAFT` + `ALLOW`.
# 6. Send SIGTERM and wait for the daemon to exit.
# 7. Tempdir cleanup happens via trap.
#
# Exits 0 on success, non-zero with a diagnostic message otherwise.

set -euo pipefail

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$REPO_ROOT"

DAEMON_PID=""
TMPDIR_PATH=""

cleanup() {
  local code=$?
  if [[ -n "$DAEMON_PID" ]] && kill -0 "$DAEMON_PID" 2>/dev/null; then
    kill -TERM "$DAEMON_PID" 2>/dev/null || true
    # Give it 2s; force-kill if needed.
    for _ in 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20; do
      kill -0 "$DAEMON_PID" 2>/dev/null || break
      sleep 0.1
    done
    kill -KILL "$DAEMON_PID" 2>/dev/null || true
  fi
  if [[ -n "$TMPDIR_PATH" && -d "$TMPDIR_PATH" ]]; then
    rm -rf "$TMPDIR_PATH"
  fi
  exit "$code"
}
trap cleanup EXIT INT TERM

step() { printf '\033[1;34m[e2e] %s\033[0m\n' "$*"; }
fail() {
  printf '\033[1;31m[e2e FAIL] %s\033[0m\n' "$*" >&2
  exit 1
}

step "build release binaries"
cargo build --release \
  --bin aotf --bin aotfd --bin aotf-gatekeeper >/dev/null

AOTF="$REPO_ROOT/target/release/aotf"
AOTFD="$REPO_ROOT/target/release/aotfd"

[[ -x "$AOTF" ]] || fail "aotf binary not found at $AOTF"
[[ -x "$AOTFD" ]] || fail "aotfd binary not found at $AOTFD"

TMPDIR_PATH="$(mktemp -d -t aotf-e2e-XXXXXX)"
export HOME="$TMPDIR_PATH"
RUNTIME_DIR="$TMPDIR_PATH/.aotf/run"
WATCH_DIR="$TMPDIR_PATH/.aotf-watch"
LOG_FILE="$TMPDIR_PATH/aotfd.log"

step "tmp HOME=$TMPDIR_PATH"
cd "$TMPDIR_PATH"

step "aotf init"
"$AOTF" init >/dev/null
[[ -f "$TMPDIR_PATH/.aotf/config.toml" ]] || fail "aotf init did not write config.toml"
[[ -d "$WATCH_DIR" ]] || fail "aotf init did not create $WATCH_DIR"

step "spawn aotfd"
"$AOTFD" --watch-dir "$WATCH_DIR" --runtime-dir "$RUNTIME_DIR" \
  >"$LOG_FILE" 2>&1 &
DAEMON_PID=$!

step "wait for cli socket"
for _ in 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20 \
         21 22 23 24 25 26 27 28 29 30 31 32 33 34 35 36 37 38 39 40 \
         41 42 43 44 45 46 47 48 49 50; do
  if [[ -S "$RUNTIME_DIR/cli.sock" ]]; then
    break
  fi
  sleep 0.1
done
if [[ ! -S "$RUNTIME_DIR/cli.sock" ]]; then
  cat "$LOG_FILE" >&2
  fail "cli socket did not appear within 5s"
fi

step "touch watched file"
touch "$WATCH_DIR/hello.txt"

step "wait for DRAFT/ALLOW audit entry"
saw=""
for _ in 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20 \
         21 22 23 24 25 26 27 28 29 30 31 32 33 34 35 36 37 38 39 40 \
         41 42 43 44 45 46 47 48 49 50; do
  if out="$("$AOTF" --runtime-dir "$RUNTIME_DIR" audit ls 2>/dev/null)" \
     && grep -E 'DRAFT.*ALLOW' <<<"$out" >/dev/null; then
    saw="$out"
    break
  fi
  sleep 0.1
done
if [[ -z "$saw" ]]; then
  echo "--- daemon log ---" >&2
  cat "$LOG_FILE" >&2
  echo "--- last audit output ---" >&2
  echo "${out:-<nothing>}" >&2
  fail "no DRAFT/ALLOW audit entry within 5s"
fi
step "audit row seen:"
grep -E 'DRAFT.*ALLOW' <<<"$saw" | head -1

step "SIGTERM the daemon"
kill -TERM "$DAEMON_PID"
wait_count=0
while kill -0 "$DAEMON_PID" 2>/dev/null; do
  wait_count=$((wait_count + 1))
  if (( wait_count > 30 )); then
    fail "daemon did not exit within 3s after SIGTERM"
  fi
  sleep 0.1
done
DAEMON_PID=""

step "PASS"
