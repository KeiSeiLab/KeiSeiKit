---
title: rollback.rs
path: kei-forge/src/generate/rollback.rs
dna_hash: sha256:92ef0e3440fce393
language: rust
size_loc: 100
generated: by-keidocs
---

# kei-forge/src/generate/rollback.rs

Rollback accumulator for atom scaffolding writes.

Keeps the list of successfully-written paths. On `finish()` the list
is returned (success). On `Drop` without `finish()` — i.e. an early
return from the caller due to an error — every recorded path is
deleted best-effort. Mirrors `trap rollback ERR` in new-atom.sh.

Deletion is best-effort: we ignore `std::fs::remove_file` errors
because the caller already has a more-specific error to return.

## Public API

- `pub fn record` — Register a successful write so the rollback can undo it on drop.
- `pub fn finish` — Consume the rollback — mark complete and return the recorded

## Related

- parent: `kei-forge/Cargo.toml`
- imports: std

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
