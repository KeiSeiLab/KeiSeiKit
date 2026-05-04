---
title: sleep_report.rs
path: kei-token-tracker/src/sleep_report.rs
dna_hash: sha256:33ed1b02fc08e4b3
language: rust
size_loc: 93
generated: by-keidocs
---

# kei-token-tracker/src/sleep_report.rs

Phase D nightly markdown sleep-report rendering.

Pure function: takes a sorted `Vec<ModelAggregate>` + a date string
and emits a deterministic markdown payload. No filesystem side-effects
here — the CLI dispatcher writes the bytes.

## Public API

- `pub fn render` — Render the markdown report. `date` is the report header (e.g.

## Related

- parent: `kei-token-tracker/Cargo.toml`
- imports: crate

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
