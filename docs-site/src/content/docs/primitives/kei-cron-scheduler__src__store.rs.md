---
title: store.rs
path: kei-cron-scheduler/src/store.rs
dna_hash: sha256:26cce4861ae9758e
language: rust
size_loc: 147
generated: by-keidocs
---

# kei-cron-scheduler/src/store.rs

JSON-on-disk job store.

Hermes equivalent: `cron/jobs.py` (load/save). Uses `fs2` advisory file
locking so parallel processes can safely share the same `jobs.json`.
Writes are atomic via temp+rename.

## Public API

- All store errors.
- `pub struct JobStore` — Opens / creates `jobs.json` at the configured path.
- `pub fn default_path` — Convenience helper: `~/.keiseikit/scheduler/jobs.json`.
- `pub fn load_all` — Read all jobs (consumes a shared lock for the duration of the read).
- `pub fn modify` — Atomic read-modify-write under exclusive lock.
- `pub fn upsert` — Insert or overwrite one job.
- `pub fn remove` — Remove a job by ID. Errors if missing.
- `pub fn get` — Single-job lookup (no lock — best-effort eventual consistency).

## Related

- parent: `kei-cron-scheduler/Cargo.toml`
- imports: crate, fs2, std, thiserror

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
