---
title: usage.rs
path: kei-cortex/src/handlers/usage.rs
dna_hash: sha256:bec9b74e790a45d4
language: rust
size_loc: 136
generated: by-keidocs
---

# kei-cortex/src/handlers/usage.rs

`GET /api/v1/cortex/usage` — cost rollup over the kei-ledger SQLite.

Read-only aggregation over today / week / month time windows plus
the top (provider, model) pair by total cost. The handler returns
404 if the ledger file is missing or its `agents` table lacks a
`cost_cents` column — schema migration is a separate concern.

F-MED-3 fix: today / week / month boundaries are CALENDAR DAYS in the
local timezone (Mon-anchored ISO weeks, 1st-of-month). The pre-fix
sliding-window rollup (`now - 24*3600`, etc.) drifted across midnight
and contradicted the UI labels. See `usage_calendar.rs` for the
boundary helpers.

## Public API

- JSON body returned to the UI. Cents are unsigned i64 — the handler
- Handler entry point. Off-loads the SQLite read to a blocking task
- Top-level loader. Returns `None` (→ 404) when the ledger file is
- Probe `pragma table_info(agents)` for the three cost-tracking columns
- Sum `cost_cents` since each calendar boundary (today / Monday / 1st of
- Top (provider, model) pair by total `cost_cents` across all rows.

## Related

- parent: `kei-cortex/Cargo.toml`
- imports: axum, calendar, crate, rusqlite, serde, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
