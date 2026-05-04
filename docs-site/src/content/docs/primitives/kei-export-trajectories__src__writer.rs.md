---
title: writer.rs
path: kei-export-trajectories/src/writer.rs
dna_hash: sha256:51384ce4657bb33f
language: rust
size_loc: 110
generated: by-keidocs
---

# kei-export-trajectories/src/writer.rs

Atomic JSONL writer.

Constructor Pattern: one cube = path → tempfile → fsync → rename. The
caller hands us an iterator of `Trajectory`; we serialize one per line
using `serde_json::to_string` (no pretty-printing — JSONL is
line-delimited and trainers expect compact output).

Atomicity matters because exports often run during a Phase B / Phase D
sleep cycle: a partially written `trajectory_samples.jsonl` would
poison the next training run. We write to `<path>.tmp` in the same
directory (so rename is atomic on POSIX) and only swap on full success.

## Public API

- `pub fn write_jsonl` — Serialize `trajectories` to JSONL at `path`, writing through a sibling
- Compute the sibling `.tmp` path. Putting it in the same directory keeps
- Serialize every trajectory to the temp file, flush, drop. Each line is
- Open the temp file for write, truncating any previous failed run's

## Related

- parent: `kei-export-trajectories/Cargo.toml`
- imports: anyhow, crate, std, tempfile

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
