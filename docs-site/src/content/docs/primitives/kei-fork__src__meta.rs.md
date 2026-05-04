---
title: meta.rs
path: kei-fork/src/meta.rs
dna_hash: sha256:8123114e68e8041a
language: rust
size_loc: 49
generated: by-keidocs
---

# kei-fork/src/meta.rs

`.KEI_FORK_META.toml` — on-disk metadata written once by `create()`
and read by `list()` / `collect()` / `rescue()` / `gc()`.

Layout is stable: `agent_id`, `started_ts`, `base_branch`, `ledger_id`.
Never add fields without bumping a schema version.

## Related

- parent: `kei-fork/Cargo.toml`
- imports: crate, serde, std

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
