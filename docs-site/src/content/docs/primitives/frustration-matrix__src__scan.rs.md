---
title: scan.rs
path: frustration-matrix/src/scan.rs
dna_hash: sha256:2ee05bc80bdadbf3
language: rust
size_loc: 199
generated: by-keidocs
---

# frustration-matrix/src/scan.rs

Scanner — walk `root`, parse each chatlog, apply compiled categories
(or the firmware `Classifier` when `--model-dir` is set), emit rows.
Handles both curated markdown (`.md`) and raw Claude Code `.jsonl`.
Constructor Pattern: one public entry (`run`); helpers small + private.

## Public API

- Output format accepted by `scan`.
- Source file kind — dispatch target for per-file parser.
- `pub struct ScanArgs` — Inputs from the CLI layer — keep the main.rs dispatch thin.
- When `Some`: bypass regex, classify via firmware. `None`: regex path.
- `pub fn run` — Execute a full scan. Returns number of rows emitted.
- Map a filesystem path to its parser kind, or `None` to skip.
- Dispatch to the parser for `kind` and return `Hit`s. Markdown reads
- Best-effort ISO-ish stamp from mtime. Returns empty on FS errors — row

## Related

- parent: `frustration-matrix/Cargo.toml`
- imports: anyhow, crate, std, walkdir

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
