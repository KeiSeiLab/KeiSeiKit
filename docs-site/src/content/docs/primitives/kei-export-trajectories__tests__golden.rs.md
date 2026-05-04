---
title: golden.rs
path: kei-export-trajectories/tests/golden.rs
dna_hash: sha256:6983f8890e67c511
language: rust
size_loc: 80
generated: by-keidocs
---

# kei-export-trajectories/tests/golden.rs

Golden-file test: insert two synthetic agents into an in-memory
ledger + memory database, run the export library through its public
API, compare emitted JSONL line-by-line against a checked-in
fixture.

Constructor Pattern: fixture builders live in `tests/common/`.

## Related

- parent: `kei-export-trajectories/tests`
- imports: kei_export_trajectories, std, tempfile

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
