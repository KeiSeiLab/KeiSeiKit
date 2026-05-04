---
title: lib.rs
path: kei-diff/src/lib.rs
dna_hash: sha256:fb838001f73f1507
language: rust
size_loc: 22
generated: by-keidocs
---

# kei-diff/src/lib.rs

kei-diff — structural JSON diff (RFC 6902 subset: add/remove/replace).

## Design
* Emits ONLY `add`, `remove`, `replace`. No `copy`/`move`/`test`.
* Arrays diffed by index (not LCS) — matches drift-detection semantics.
* Paths are RFC 6901 JSON Pointers (`~` → `~0`, `/` → `~1`).
* Correctness invariant: `apply(old, diff(old, new)) == new`.

Consumed by `kei-replay` (drift detection between DNA-scoped agent runs)
and `kei-cache` (invalidation signals). Pure compute, zero sibling deps.

## Related

- parent: `kei-diff/Cargo.toml`

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
