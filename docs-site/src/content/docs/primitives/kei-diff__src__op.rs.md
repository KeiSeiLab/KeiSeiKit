---
title: op.rs
path: kei-diff/src/op.rs
dna_hash: sha256:911b1db1b92790d9
language: rust
size_loc: 134
generated: by-keidocs
---

# kei-diff/src/op.rs

Patch operation types + RFC 6902 JSON serialization.

We emit only the minimal trio (`add`, `remove`, `replace`). Custom Serialize
keeps the wire format stable and self-documenting (no need for serde tag
gymnastics).

## Public API

- A single RFC 6902 patch operation (subset).
- An ordered list of `Op`s. Serializes as a JSON array per RFC 6902.

## Related

- parent: `kei-diff/Cargo.toml`
- imports: serde, serde_json, std

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
