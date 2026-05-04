---
title: archive_smoke.rs
path: kei-entity-store/tests/archive_smoke.rs
dna_hash: sha256:97f84c398b045638
language: rust
size_loc: 99
generated: by-keidocs
---

# kei-entity-store/tests/archive_smoke.rs

Archive-verb smoke tests.

Covers kei-chat-store migration: schemas opt-in to soft-delete via
`archived_field: Some("archived")`. The verb flips the column + an
optional `<field>_at` sibling timestamp.

## Related

- parent: `kei-entity-store/tests`
- imports: kei_entity_store, serde_json

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
