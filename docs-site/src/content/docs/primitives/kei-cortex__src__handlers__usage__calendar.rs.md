---
title: calendar.rs
path: kei-cortex/src/handlers/usage/calendar.rs
dna_hash: sha256:9efca18390eabc10
language: rust
size_loc: 140
generated: by-keidocs
---

# kei-cortex/src/handlers/usage/calendar.rs

Calendar-day boundary computation in local time (F-MED-3).

Three anchors:
- `today_start_ts`  — 00:00 of the current local day
- `week_start_ts`   — 00:00 Monday of the current ISO week
- `month_start_ts`  — 00:00 of the 1st of the current month

All exposed as unix seconds (UTC), suitable for `WHERE started_ts >= ?`
against `started_ts INTEGER` in the ledger DB.

The pure helper `boundaries_for(local_dt)` is split out so unit tests
can drive a deterministic `NaiveDateTime` instead of `Local::now()`.

## Public API

- Three calendar anchors as unix seconds.
- `pub fn for_now_local` — Compute the three boundaries from the system clock in local time.
- `pub fn boundaries_for` — Pure boundary computation given a naive local datetime. Folds three
- 00:00 local of `date` → unix seconds (UTC). DST-ambiguous instant
- Monday of the ISO week containing `date`. ISO weeks are Monday-anchored.
- 1st-of-month for `date`.
- Wednesday 2026-04-22 14:30 → today=22, week=20 (Mon), month=01.
- Sunday 2026-04-26 23:59 → today=26, week=20 (Mon prior), month=01.
- 23:59 today and 00:01 tomorrow give DIFFERENT today_start_ts —

## Related

- parent: `kei-cortex/Cargo.toml`
- imports: chrono

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
