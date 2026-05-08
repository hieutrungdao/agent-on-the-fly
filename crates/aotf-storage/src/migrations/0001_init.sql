-- AOTF audit log schema, v1.
-- Append-only by API contract (the Rust surface exposes only `append` and `list`).
-- Storage-layer enforcement (revoke DELETE via SQLite views/triggers) is a
-- v0.0.3 hardening item; the plan explicitly defers it.

CREATE TABLE IF NOT EXISTS audit_entry (
    id              INTEGER PRIMARY KEY AUTOINCREMENT,
    finding_id      TEXT    NOT NULL,
    tier            TEXT    NOT NULL,
    decision        TEXT    NOT NULL CHECK(decision IN ('ALLOW','DENY')),
    reason          TEXT    NOT NULL,
    created_micros  INTEGER NOT NULL,
    payload_json    TEXT    NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_audit_finding ON audit_entry(finding_id);
CREATE INDEX IF NOT EXISTS idx_audit_created ON audit_entry(created_micros);
