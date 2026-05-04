---
title: parser.rs
path: kei-cron-scheduler/src/parser.rs
dna_hash: sha256:e6442601a282aa7a
language: rust
size_loc: 158
generated: by-keidocs
---

# kei-cron-scheduler/src/parser.rs

Schedule parser.

Port of Hermes `cron/jobs.py:parse_schedule` (102-209). Four input modes:

1. Bare duration  — `30m`, `2h`, `1d`             → [`Schedule::AfterDuration`]
2. Recurring      — `every 30m`, `every 2h`       → [`Schedule::Interval`]
3. Cron expr      — `0 9 * * *`                   → [`Schedule::Cron`]
4. ISO timestamp  — `2026-05-01T14:00:00Z`        → [`Schedule::Once`]

## Public API

- All parser errors.
- `pub fn parse_schedule` — Top-level entry point.
- `pub fn parse_duration` — Parse `30m`, `2h`, `1d` (and verbose variants).
- Heuristic: ≥5 whitespace-separated tokens, each containing only digits or

## Related

- parent: `kei-cron-scheduler/Cargo.toml`
- imports: chrono, crate, std, thiserror

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
