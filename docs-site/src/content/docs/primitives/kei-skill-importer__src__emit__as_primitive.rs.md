---
title: as_primitive.rs
path: kei-skill-importer/src/emit/as_primitive.rs
dna_hash: sha256:9078ed93d43c117b
language: rust
size_loc: 171
generated: by-keidocs
---

# kei-skill-importer/src/emit/as_primitive.rs

Emit `ImportedSkill` as a proposed-primitive markdown.

This output is INTENDED for human review — it is NOT a real
primitive. The file lives at `_primitives/proposed/<name>.md`
and contains:
- classification report (atoms-found / unresolved / why-emerge)
- suggested `MANIFEST.toml` stanza in a fenced TOML block
- suggested `Cargo.toml` skeleton in a fenced TOML block
- body of the original skill quoted verbatim

The reviewer decides: accept (turn into real primitive) / reject /
defer / split-into-recipe.

## Public API

- `pub fn render` — Render `skill` as proposed-primitive markdown text. Side-effect-free.
- `pub fn write` — Render + write to `<output_dir>/proposed/<crate>.md`.

## Related

- parent: `kei-skill-importer/Cargo.toml`
- imports: anyhow, crate, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
