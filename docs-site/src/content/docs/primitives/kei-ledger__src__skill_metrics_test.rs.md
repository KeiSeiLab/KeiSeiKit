---
title: skill_metrics_test.rs
path: kei-ledger/src/skill_metrics_test.rs
dna_hash: sha256:ac394fc2ab1aa749
language: rust
size_loc: 174
generated: by-keidocs
---

# kei-ledger/src/skill_metrics_test.rs

Inline tests for `skill_metrics`. Constructor Pattern:
sibling test file via `#[path]` from `skill_metrics.rs`.

Strategy: open a fresh tempdir-backed ledger (so v8 migration runs),
insert 30 fixture rows spanning two skills × successes/failures ×
recent/stale timestamps, then assert each public helper.

## Public API

- End-to-end dispatch path: record_invocation via the same call the CLI

## Related

- parent: `kei-ledger/Cargo.toml`
- imports: crate, rusqlite, tempfile

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
