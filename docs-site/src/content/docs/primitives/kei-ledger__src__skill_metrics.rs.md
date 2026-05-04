---
title: skill_metrics.rs
path: kei-ledger/src/skill_metrics.rs
dna_hash: sha256:2a240b268199963b
language: rust
size_loc: 140
generated: by-keidocs
---

# kei-ledger/src/skill_metrics.rs

Phase D skill-invocation metrics (HERMES-MIGRATION-PLAN P3.4).

Constructor Pattern: one cube = one read/write surface for the
`skill_invocations` table introduced by schema v8. Phase D's nightly
self-improvement loop uses these helpers to decide
- archive (skill not used in N days)
- re-extract (success_rate < 60% over last M days)
- mark validated (usage_count > 20 AND success_rate > 90%)

Times are unix-seconds (i64), matching the rest of the ledger
(`agents.started_ts`, etc.). The task spec calls for unix-ms; we keep
seconds for ledger-wide consistency — Phase D thresholds are "days",
and millisecond resolution buys nothing while breaking SUM/MAX
comparisons against existing columns.

## Public API

- One invocation record. `success` is the agent's review.md verdict
- `pub fn record_invocation` — Append a row. Returns rows-inserted (always 1 on success).
- `pub fn success_rate` — Success rate over `lookback_days` for `skill_name`. Returns NaN when
- `pub fn usage_count` — Count invocations of `skill_name` in the last `lookback_days`.
- `pub fn last_used` — Most-recent `ts` for `skill_name`. Returns `None` if never invoked.
- `pub fn unused_skills` — Distinct skill names whose most-recent invocation is older than
- `pub fn unused_skills_at` — Test-injectable variant of [`unused_skills`].
- `now - days*86400`. Centralised so tests and helpers agree on the
- Test-injectable: `cutoff_ts(days)` reduces to this with `now` from clock.

## Related

- parent: `kei-ledger/Cargo.toml`
- imports: rusqlite, serde

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
