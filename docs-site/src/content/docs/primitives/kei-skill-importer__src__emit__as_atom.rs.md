---
title: as_atom.rs
path: kei-skill-importer/src/emit/as_atom.rs
dna_hash: sha256:eae0c66cf9459970
language: rust
size_loc: 128
generated: by-keidocs
---

# kei-skill-importer/src/emit/as_atom.rs

Emit `ImportedSkill` as a KeiSeiKit atom markdown.

Produces a YAML-frontmatter `.md` file matching the shape of
`_primitives/_rust/kei-task/atoms/search.md`.

Provenance: an HTML-comment line is injected immediately after the
frontmatter delimiter recording the source path and import time.

## Public API

- `pub fn render` — Render `skill` as atom markdown text. Side-effect-free.
- `pub fn write` — Render + write to `<output_dir>/atoms/<verb>.md`. Returns the

## Related

- parent: `kei-skill-importer/Cargo.toml`
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
