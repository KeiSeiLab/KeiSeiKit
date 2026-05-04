---
title: eval_report.rs
path: frustration-matrix/src/eval_report.rs
dna_hash: sha256:65a6908baf965014
language: rust
size_loc: 140
generated: by-keidocs
---

# frustration-matrix/src/eval_report.rs

Eval report — CSV writer + human-readable stdout summary.

Constructor Pattern: IO-only. All math is already done in
`eval_metrics`; this cube just serializes.

CSV schema (one row per model per category):
`model,category,precision,recall,f1,support`

Stdout format matches the layout in the task spec — fixed-width
columns so `grep` / `awk` still work on the summary.

## Public API

- `pub const CSV_HEADER` — CSV header — kept as a const so tests + readers agree.
- `pub fn write_csv` — Write the full report to a CSV file.
- `pub fn print_summary` — Print a human-readable summary. Mirrors the task-spec layout.
- Two-line overall block: accuracy + macro-f1 for both models.
- Union of categories seen in either model's report, alphabetical.

## Related

- parent: `frustration-matrix/Cargo.toml`
- imports: anyhow, crate, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
