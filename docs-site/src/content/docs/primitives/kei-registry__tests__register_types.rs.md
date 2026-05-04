---
title: register_types.rs
path: kei-registry/tests/register_types.rs
dna_hash: sha256:1b1357259f016ff0
language: rust
size_loc: 134
generated: by-keidocs
---

# kei-registry/tests/register_types.rs

Unit tests for per-type DNA computation and idempotency.

Covers: BlockMdScanner, CapabilityScanner, RoleScanner — each scanner
returns a Found with BlockType::Atom. Idempotency: re-register → no-op.

## Related

- parent: `kei-registry/tests`
- imports: kei_registry, std, tempfile

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
