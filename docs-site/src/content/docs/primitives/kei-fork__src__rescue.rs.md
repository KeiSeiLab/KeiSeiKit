---
title: rescue.rs
path: kei-fork/src/rescue.rs
dna_hash: sha256:7859c2c1ce6ab8e5
language: rust
size_loc: 54
generated: by-keidocs
---

# kei-fork/src/rescue.rs

`rescue(agent_id, kit_root, out_dir)` — copy a fork's files out of
band.

Resolution order:
1. `_forks/<id>/` (live) → copy to `out_dir`
2. `_archive/forks/<date>/<id>/` (archived) → copy to `out_dir`
3. Neither → `Error::Gone`

Copy is recursive; the destination may pre-exist (we merge on top).
Returns the number of regular files copied.

## Related

- parent: `kei-fork/Cargo.toml`
- imports: crate, std

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
