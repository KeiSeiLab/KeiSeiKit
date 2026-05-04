---
title: analyze.rs
path: kei-memory/src/analyze.rs
dna_hash: sha256:107b7fc5525d63b1
language: rust
size_loc: 121
generated: by-keidocs
---

# kei-memory/src/analyze.rs

Session retrospective — duration, tool counts, files, errors, time-wasters.

Constructor Pattern: one cube, one read-only responsibility.
Output is plain-text (stdout). Callers can `--summary` for a one-liner
suitable for appending to audit-backlog.md, or full report for review.

## Public API

- `pub struct SessionHeader` — Minimal session-header info returned as tuple for downstream formatters.
- `pub fn session_header` — Load the `sessions` row for an id.
- `pub fn recent_session_ids` — Return the last `n` session ids (most recent first).
- `pub fn top_tools` — Return (tool, count) pairs ordered by invocation count DESC.
- `pub fn top_files` — Return (file_path, count) for the most-touched files in a session.
- `pub fn render_report` — Render a full retrospective for one session to stdout.
- `pub fn render_recent` — Aggregate analyze across recent N sessions — concat render_report each.

## Related

- parent: `kei-memory/Cargo.toml`
- imports: rusqlite

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
