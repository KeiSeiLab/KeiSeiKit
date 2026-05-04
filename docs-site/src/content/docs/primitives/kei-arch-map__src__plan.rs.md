---
title: plan.rs
path: kei-arch-map/src/plan.rs
dna_hash: sha256:1aa252c007d21ff4
language: rust
size_loc: 160
generated: by-keidocs
---

# kei-arch-map/src/plan.rs

## Public API

- `pub fn add_claim` — Append a claim to PLAN.toml. Holds an exclusive advisory file lock on a
- Open companion `<plan>.lock` and acquire exclusive advisory lock.
- Verify (module_id, claim_id) is not already declared.
- Render a `[[module.claim]]` block as TOML text.

## Related

- parent: `kei-arch-map/Cargo.toml`
- imports: anyhow, crate, fs2, kei_arch_map, std, toml_edit

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
