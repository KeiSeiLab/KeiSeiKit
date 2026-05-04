---
title: create_defaults.rs
path: kei-entity-store/src/verbs/create_defaults.rs
dna_hash: sha256:146744d032194973
language: rust
size_loc: 109
generated: by-keidocs
---

# kei-entity-store/src/verbs/create_defaults.rs

Per-kind value-for-insert helpers split out of `create.rs` to keep
that file under the Constructor-Pattern 200-LOC cap. Each function
handles one FieldKind's default / coerce logic.

## Related

- parent: `kei-entity-store/Cargo.toml`
- imports: crate, rusqlite, serde_json

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
