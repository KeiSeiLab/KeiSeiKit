---
title: README
path: keidna-sign/README.md
dna_hash: sha256:e9bffe18ccb338f1
language: markdown
size_loc: 56
generated: by-keidocs
---

# keidna-sign/README.md

## Public API

- `keidna-sign` — Signed DNA manifest for KeiSeiKit primitives. Phase 1 = sha256 content
- `What it does` — Walks `src/**/*.rs` + `Cargo.toml` of a primitive, hashes each file with
- `CLI` — ```bash
- `emit DNA for one primitive (root = workspace dir containing the primitive)` — keidna-sign emit --primitive kei-cortex --root _primitives/_rust
- `verify .dna.json still matches current source` — keidna-sign verify --primitive kei-cortex --root _primitives/_rust
- `table of all primitives + their stored DNA` — keidna-sign list --root _primitives/_rust
- `Use in CI` — ```yaml
- `Sample `.dna.json`` — ```json

## Related

- parent: `keidna-sign`

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
