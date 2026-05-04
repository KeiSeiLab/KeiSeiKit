---
title: invoke_io.rs
path: kei-runtime/src/invoke_io.rs
dna_hash: sha256:7b4033cf861551f0
language: rust
size_loc: 146
generated: by-keidocs
---

# kei-runtime/src/invoke_io.rs

Bounded-read child-process IO for `invoke.rs`.

Wave 44d resource-cap hardening: the previous `wait_with_output()`
buffered ALL stdout/stderr before any size check ran — a malicious
atom writing 10 GiB of zeros would OOM the runtime before truncation.
This module replaces that with size-tracked stream readers that KILL
the child the moment a cap is exceeded — the kill is issued from
INSIDE the reader thread so an unbounded writer cannot deadlock the
parent (post-hoc kill would never happen because the reader would
never return on infinite stdout).

Constructor Pattern: extracted as a sibling module so `invoke.rs`
stays under 200 LOC and per-function under 30 LOC.

## Public API

- Hard cap on stdout/stderr each. Mirrors the public `OUTPUT_CAP`
- Per-read chunk size. 8 KiB is the typical pipe buffer granularity
- Captured child output. `truncated` is true when EITHER stream hit
- Shared kill-handle: the reader thread that trips the cap takes the
- Read both streams concurrently with size caps. If either reader
- Reap the child (waiting on it) and return its exit code. If a reader
- Spawn the stdout reader thread. Returns `(buffer, truncated)`.
- Spawn the stderr reader thread. Returns `(buffer, truncated)`.
- Read until EOF or cap exceeded. On cap exceedance, kill the child
- Append-or-mark logic, extracted so `read_capped` stays under the 30-LOC
- Kill the child via the shared handle. Best-effort: if another

## Related

- parent: `kei-runtime/Cargo.toml`
- imports: std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
