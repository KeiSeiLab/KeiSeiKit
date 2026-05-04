---
title: nightly.rs
path: kei-frustration-loop/src/nightly.rs
dna_hash: sha256:1e897da285dd9bdb
language: rust
size_loc: 199
generated: by-keidocs
---

# kei-frustration-loop/src/nightly.rs

Phase-0 nightly scan — walks new chatlogs since timestamp, classifies
each user line via regex categories (the regex SSoT in `categories.rs`),
and appends every hit to the shared queue for morning review.

The per-user `Firmware` argument is the user's baseline language model;
it is a passthrough today and reserved for future likelihood-ratio
filtering once enough confirmed feedback exists. Carrying it in the
signature now keeps the call sites stable across the v2 → v3 cutover.

Constructor Pattern: this cube only orchestrates existing primitives
(jsonl parser + categories regex). Hit emission goes through the
`feedback`/`persistence` cubes; this file owns no IO format.

## Public API

- `pub const MIN_HIT_LEN` — Default minimum char length applied to user messages before any regex
- One queued hit awaiting morning review.
- Stable identifier — `<file_basename>:<line_no>:<ts>`.
- User identifier under which this hit was produced.
- Predicted category id (matches `categories.rs::Category.id`).
- Original message text.
- Source file (absolute path).
- 1-based line number inside the source file.
- Unix-seconds timestamp when this hit was queued.
- Aggregate scan result returned to the CLI / sleep-layer Phase 0.
- Total trace files visited (after `since_ts` filter).
- Total hits emitted across all files.
- Per-category hit counts.
- `pub fn nightly_scan` — Walk `traces_dir`, classify each user line in files modified strictly
- Walk `traces_dir` and return every `.jsonl` file with mtime strictly
- Parse one trace, run every user line through the regex categories,
- Run regex categories against `text`. Returns the first category id whose
- Construct a `QueuedHit` from one parsed user line + classifier verdict.
- Append one hit as JSONL on `queue` with O_APPEND.
- File mtime in Unix seconds. 0 on any FS error (caller treats 0 as
- Wall-clock now in Unix seconds. 0 if the system clock is broken.

## Related

- parent: `kei-frustration-loop/Cargo.toml`
- imports: anyhow, frustration_matrix, serde, std, walkdir

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
