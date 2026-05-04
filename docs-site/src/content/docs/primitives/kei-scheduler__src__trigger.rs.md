---
title: trigger.rs
path: kei-scheduler/src/trigger.rs
dna_hash: sha256:8d348f924489d799
language: rust
size_loc: 105
generated: by-keidocs
---

# kei-scheduler/src/trigger.rs

Trigger parsing + `compute_next` core.

Three trigger kinds:
- `cron` — 5-field or 6-field cron expression. 5-field inputs get a
leading `"0 "` for seconds (standard cron semantics: trigger at
the start of the minute).
- `at` — one-shot ISO-8601 datetime with trailing `Z` (UTC).
- `interval` — repeat every N seconds (positive integer).

`compute_next(kind, spec, from)` is pure — no DB, no clock read.
Callers pass `from` explicitly so tests are deterministic.

## Public API

- `pub const CRON` — Canonical trigger-kind names. Schema stores these as TEXT; this
- `pub fn validate_kind` — Validate a trigger kind string. Returned as `&'static str` so callers
- `pub fn compute_next` — Compute the next fire time (unix seconds, UTC) for a trigger given a
- Accept both classic 5-field (`min hour dom mon dow`) and the
- Parse `YYYY-MM-DDTHH:MM:SSZ` (no fractional seconds, no offset other

## Related

- parent: `kei-scheduler/Cargo.toml`
- imports: chrono, crate, cron, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
