---
title: import.rs
path: kei-sage/src/import.rs
dna_hash: sha256:03c083d4711eb335
language: rust
size_loc: 76
generated: by-keidocs
---

# kei-sage/src/import.rs

Obsidian-style vault import: walk a directory, ingest .md files.

Minimal subset of LBM internal/sage/import_obsidian.go — we do NOT parse
frontmatter here (the upstream parser used multiple helper files). Port
of frontmatter/wikilinks parsing is a later milestone; this cube honours
the public interface.

## Related

- parent: `kei-sage/Cargo.toml`
- imports: anyhow, crate, std

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
