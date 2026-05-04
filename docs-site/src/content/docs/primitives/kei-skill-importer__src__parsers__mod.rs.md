---
title: mod.rs
path: kei-skill-importer/src/parsers/mod.rs
dna_hash: sha256:579026deac79a13d
language: rust
size_loc: 106
generated: by-keidocs
---

# kei-skill-importer/src/parsers/mod.rs

Format parsers — one module per source dialect.

Each parser exposes `pub fn parse(path: &Path) -> Result<ImportedSkill>`
and is side-effect-free (read-only file access).

## Public API

- `pub fn detect_format` — Detect format from extension + content sniffing.
- Shared utility: split a markdown file into `(frontmatter_yaml, body)`.
- Heuristic language detector: counts Cyrillic codepoints in the

## Related

- parent: `kei-skill-importer/Cargo.toml`
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
