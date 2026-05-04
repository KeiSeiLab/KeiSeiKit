---
title: ledger_tests.rs
path: kei-cortex/tests/ledger_tests.rs
dna_hash: sha256:8bc87921e9a3abf6
language: rust
size_loc: 84
generated: by-keidocs
---

# kei-cortex/tests/ledger_tests.rs

`GET /api/v1/cortex/ledger/recent` — integration test with a seeded DB.

Uses the minimal v1 schema subset that the cortex query depends on.
Not linked against `kei-ledger` to keep the test hermetic.

## Public API

- Create the subset of the kei-ledger v1 `agents` table we need + seed rows.

## Related

- parent: `kei-cortex/tests`
- imports: common, reqwest, rusqlite, serde_json

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
