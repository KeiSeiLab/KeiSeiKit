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

<script src="https://giscus.app/client.js"
        data-repo="KeiSei84/KeiSeiKit-1.0"
        data-repo-id="PLACEHOLDER_REPO_ID"
        data-category="wiki-comments"
        data-category-id="PLACEHOLDER_CATEGORY_ID"
        data-mapping="pathname"
        data-strict="0"
        data-reactions-enabled="1"
        data-emit-metadata="0"
        data-input-position="bottom"
        data-theme="preferred_color_scheme"
        data-lang="en"
        data-loading="lazy"
        crossorigin="anonymous"
        async></script>
