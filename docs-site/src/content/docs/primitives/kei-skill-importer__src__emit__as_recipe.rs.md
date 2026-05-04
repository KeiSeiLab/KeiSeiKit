---
title: as_recipe.rs
path: kei-skill-importer/src/emit/as_recipe.rs
dna_hash: sha256:7f19343366da17c1
language: rust
size_loc: 145
generated: by-keidocs
---

# kei-skill-importer/src/emit/as_recipe.rs

Emit `ImportedSkill` as a KeiSeiKit recipe TOML DAG.

Recipe schema (defined in this wave — see `recipes/<name>.toml`):

```toml
[recipe]
name = "..."
description = "..."
imported_from = "openclaw://create-npm-package"
imported_at = "2026-04-25T..."

[[steps]]
id = "step-1"
atom = "kei-cortex::chat"
input = { ... }
depends_on = []
```

Steps are derived by walking each phase in order and emitting one
`[[steps]]` block per detected `atom_call` whose `atom_id` resolves.

## Public API

- `pub fn render` — Render `skill` as recipe TOML text. Side-effect-free.
- `pub fn write` — Render + write to `<output_dir>/recipes/<name>.toml`.

## Related

- parent: `kei-skill-importer/Cargo.toml`
- imports: anyhow, crate, serde, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
