---
title: intent.rs
path: firewall-diff/src/intent.rs
dna_hash: sha256:cc3c69e643a6c8bc
language: rust
size_loc: 111
generated: by-keidocs
---

# firewall-diff/src/intent.rs

Intent YAML schema + loader. See `_blocks/security-firewall-ufw.md` for
the reference format. Anything missing is treated as "don't care".

## Public API

- `pub fn key` — Canonical key used to match against a live rule: port/proto/from/action.

## Related

- parent: `firewall-diff/Cargo.toml`
- imports: serde, std

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
