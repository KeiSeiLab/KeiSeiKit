---
title: export.rs
path: kei-artifact/src/export.rs
dna_hash: sha256:de45dcb1c9b195d7
language: rust
size_loc: 82
generated: by-keidocs
---

# kei-artifact/src/export.rs

v0.16: schema-registry export.

Writes the current list of registered schema names as JSON at a path the
assembler's manifest validator reads to accept custom-registered schemas
without a rebuild.

Format: `{"schemas": ["spec", "plan", ...]}` with a trailing newline.

Constructor Pattern: one cube, one responsibility. Tests live inline —
`render()` is pure, so we exercise it without a Store.

## Public API

- `pub fn write` — Write the current schemas registry to `override_path` or the default
- `pub fn render` — Serialize `names` as `{"schemas": ["a", "b"]}\n`.
- `pub fn default_path` — `~/.claude/agents/artifacts/schemas.json` (consumed by the assembler).

## Related

- parent: `kei-artifact/Cargo.toml`
- imports: anyhow, crate, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
