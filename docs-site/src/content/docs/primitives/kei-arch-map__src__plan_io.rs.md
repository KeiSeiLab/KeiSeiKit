---
title: plan_io.rs
path: kei-arch-map/src/plan_io.rs
dna_hash: sha256:f4270f645a0c3c36
language: rust
size_loc: 53
generated: by-keidocs
---

# kei-arch-map/src/plan_io.rs

Filesystem and string helpers for `plan.rs` — atomic write + TOML
quoting + inline-table render. Split out to keep `plan.rs` under the
200-LOC ceiling (RULE ZERO).

## Public API

- `pub fn inline_evidence` — Convert serialized evidence (multi-line `key = value`) into a single

## Related

- parent: `kei-arch-map/Cargo.toml`
- imports: anyhow, std

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
