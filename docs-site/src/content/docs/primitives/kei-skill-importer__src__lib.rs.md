---
title: lib.rs
path: kei-skill-importer/src/lib.rs
dna_hash: sha256:e4e50f920a22ee4a
language: rust
size_loc: 48
generated: by-keidocs
---

# kei-skill-importer/src/lib.rs

kei-skill-importer — parse external AI-coding-tool skill files and
emit them in KeiSeiKit canonical shapes.

Pipeline: `parse → canonicalize (ImportedSkill) → classify (atom-calls)
→ decide emit-path → emit (atom / recipe / proposed-primitive)`.

Side-effect-free at the library surface: parsers and the classifier
never write to disk. Only `emit::*::write` functions persist files,
and only when handed an explicit `output_dir`.

## Public API

- `pub fn import` — Canonical entry point: parse a skill file at `path` using the format

## Related

- parent: `kei-skill-importer/Cargo.toml`
- imports: anyhow, kei_atom_discovery, std

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
