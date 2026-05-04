---
title: manifest.rs
path: keidna-sign/src/manifest.rs
dna_hash: sha256:029e735712a750bd
language: rust
size_loc: 179
generated: by-keidocs
---

# keidna-sign/src/manifest.rs

DNA manifest schema + compute / read / write / verify.

Aggregate `dna_hash` is sha256 over a canonical concatenation of:
name | version | sorted(file_path:sha256) | sorted(deps)
Order-independent in deps; deterministic across machines.

## Related

- parent: `keidna-sign/Cargo.toml`
- imports: anyhow, serde, sha2, std, walkdir

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
