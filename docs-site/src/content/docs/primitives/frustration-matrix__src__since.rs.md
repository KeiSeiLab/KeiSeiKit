---
title: since.rs
path: frustration-matrix/src/since.rs
dna_hash: sha256:63867fe02ebfbcb9
language: rust
size_loc: 71
generated: by-keidocs
---

# frustration-matrix/src/since.rs

`--since` parser + mtime filter.

Accepts `30d`, `7d`, `1d`, `all`, or any `<N>d` (positive integer days).
Returns a `SystemTime` cut-off; files strictly older than the cut-off
are excluded from the scan.

Rationale: we use filesystem mtime rather than in-document timestamps
because chatlogs have heterogeneous timestamp formats (ISO, human,
none). mtime is reliable, cheap, and matches user intent of "files I
edited in the last 30 days".

## Public API

- `pub fn parse` — Cut-off time from a `--since` string. `None` means "scan everything".
- `pub fn passes` — True iff `path`'s mtime is at or after `cutoff`. Missing mtime → true

## Related

- parent: `frustration-matrix/Cargo.toml`
- imports: anyhow, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
