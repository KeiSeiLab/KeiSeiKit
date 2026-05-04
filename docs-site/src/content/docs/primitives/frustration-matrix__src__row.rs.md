---
title: row.rs
path: frustration-matrix/src/row.rs
dna_hash: sha256:d3780f0dd3a954a4
language: rust
size_loc: 111
generated: by-keidocs
---

# frustration-matrix/src/row.rs

Output row — one hit per (category, chatlog file, line_no).

Constructor Pattern: one struct, two serializers. CSV is emitted by hand
(no `csv` crate in the dependency list); JSONL uses `serde_json`.

Fields are public and stable — this is the wire format the `report`
sub-command reads back from disk.

## Public API

- `pub const CSV_HEADER` — CSV header — kept as a const so tests + report agree.
- CSV-escape per RFC 4180 + single-line enforcement. We replace newlines
- `pub fn to_csv` — Serialize one row to a single CSV line (no trailing newline).
- `pub fn to_jsonl` — Serialize one row to JSONL (ends with newline inside `to_string`).
- `pub fn parse_csv` — Parse a CSV body (header + rows) back into `Vec<Row>`.
- Split a CSV line with RFC-4180 quote handling (single-line only).

## Related

- parent: `frustration-matrix/Cargo.toml`
- imports: anyhow, serde

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
