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

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
