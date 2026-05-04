---
title: ingest_real_trace.rs
path: kei-memory/tests/ingest_real_trace.rs
dna_hash: sha256:e03f6448b172daf9
language: rust
size_loc: 128
generated: by-keidocs
---

# kei-memory/tests/ingest_real_trace.rs

Integration tests for ingest of REAL Claude Code trace shape.

Wave A (2026-05-01) — verifies the schema-mismatch fix: nested
`tool_use` blocks inside `message.content[]` are extracted, one row
per block lands in `events` with `tool` populated, `file_path`
pulled from `input.file_path`, `cwd` from the top-level field.

## Public API

- Build a 5-line JSONL fixture mirroring real Claude Code trace shape:

## Related

- parent: `kei-memory/tests`
- imports: kei_memory, rusqlite, std, tempfile

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
