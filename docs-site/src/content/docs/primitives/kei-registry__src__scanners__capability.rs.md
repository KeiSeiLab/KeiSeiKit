---
title: capability.rs
path: kei-registry/src/scanners/capability.rs
dna_hash: sha256:5488c5c8a3f48f0e
language: rust
size_loc: 103
generated: by-keidocs
---

# kei-registry/src/scanners/capability.rs

Capability scanner — walks `<kit-root>/_capabilities/<group>/<name>/capability.toml`.

Constructor Pattern: this cube knows the nested `_capabilities/` layout.
Body = raw TOML; name = `[capability].name` from TOML, fallback = dir
stem; maps to BlockType::Atom; caps = category field if present.

## Public API

- `pub struct CapabilityScanner` — `<kit-root>/_capabilities/<group>/<name>/capability.toml` adapter.
- Extract `[capability].name` and `[capability].category` from TOML.

## Related

- parent: `kei-registry/Cargo.toml`
- imports: anyhow, crate, std, walkdir

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
