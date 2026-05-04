---
title: orphans.rs
path: kei-conflict-scan/src/scanners/orphans.rs
dna_hash: sha256:168c32d0ac4c77f1
language: rust
size_loc: 74
generated: by-keidocs
---

# kei-conflict-scan/src/scanners/orphans.rs

Orphan-reference detector.

Finds `[[wikilink]]` and `handoffs: - name` references whose targets
do not exist anywhere under the root. Case-insensitive basename match.

## Related

- parent: `kei-conflict-scan/Cargo.toml`
- imports: crate, regex, std, walkdir

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
