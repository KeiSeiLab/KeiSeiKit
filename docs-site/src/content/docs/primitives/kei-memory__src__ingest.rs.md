---
title: ingest.rs
path: kei-memory/src/ingest.rs
dna_hash: sha256:186e35679e67d3a9
language: rust
size_loc: 175
generated: by-keidocs
---

# kei-memory/src/ingest.rs

Ingest — read JSONL trace → insert events into DB.

Constructor Pattern: one cube, single responsibility.
Trace-line shape lives in `trace_line.rs`; classification in
`classifier.rs`; tool_use/tool_result extraction in `extract.rs`.
This file owns the persistence + IO loop.

Schema-mismatch fix: Wave A (2026-05-01). Pre-fix, ~50% of real
traces silently dropped via `Err(_) => continue` — root cause was
the old struct mapping `kind` to top-level `kind` field, which the
real format calls `type`, plus tool calls being nested objects.

## Public API

- `pub fn ensure_session` — Ensure the sessions row exists (idempotent). Returns started_ts.
- `pub fn ingest_jsonl` — Read a JSONL transcript line by line and insert events.
- Parse one JSONL line into a TraceLine, surfacing errors to stderr.
- Persist all event rows derivable from one parsed trace line.
- `pub fn insert_event` — Insert a single event row directly (legacy entrypoint kept for tests).
- Single insert path used by `process_line` AND `insert_event`.
- Returns true (and logs) when `message` carries a Block-tier finding.
- `pub fn finalize_session` — Update aggregate counters on the sessions row.

## Related

- parent: `kei-memory/Cargo.toml`
- imports: chrono, crate, rusqlite, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
