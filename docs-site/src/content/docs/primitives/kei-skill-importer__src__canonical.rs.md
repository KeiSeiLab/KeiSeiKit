---
title: canonical.rs
path: kei-skill-importer/src/canonical.rs
dna_hash: sha256:9879a5b92e068d89
language: rust
size_loc: 127
generated: by-keidocs
---

# kei-skill-importer/src/canonical.rs

Canonical AST for imported skills.

`ImportedSkill` is the lingua franca between parsers and emitters.
Each parser maps its source-specific shape into this struct; each
emitter consumes ONLY this struct (never the source-specific raw).

Privacy note: the raw `yaml_frontmatter` value is exposed publicly so
that emitters can preserve fidelity (round-trip), but it is NOT meant
to be inspected by downstream code in lieu of the parsed fields.

## Public API

- Source format of a skill file. `Auto` triggers detection by extension
- Top-level canonical representation of an imported skill.
- A logical phase / section / step inside a skill. For flat skills
- An invocation detected inside a phase body. `atom_id` is `Some`
- Coarse classification of a detected call site. `Bash` is a generic
- `pub fn total_atom_calls` — Total number of detected atom calls across all phases.
- `pub fn resolved_atom_calls` — Number of atom calls that resolved to a known atom_id.
- `pub fn body_bytes` — Effective body byte length (max of top-level body or sum of

## Related

- parent: `kei-skill-importer/Cargo.toml`
- imports: serde, serde_yaml_ng, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
