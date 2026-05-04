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

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
