---
title: legacy_schema.rs
path: kei-brain-view/tests/legacy_schema.rs
dna_hash: sha256:26f0f0287052f3d2
language: rust
size_loc: 38
generated: by-keidocs
---

# kei-brain-view/tests/legacy_schema.rs

Legacy-schema compat: brain-view must handle a pre-v2 ledger that
lacks the `dna` / `creator_id` / `fork_parent_id` columns. Separate
test file so the main integration suite stays focused.

## Related

- parent: `kei-brain-view/tests`
- imports: kei_brain_view, rusqlite

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
