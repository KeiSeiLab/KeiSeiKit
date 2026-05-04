---
title: mod.rs
path: keisei/src/adapters/mod.rs
dna_hash: sha256:1eddf6d80b80f435
language: rust
size_loc: 14
generated: by-keidocs
---

# keisei/src/adapters/mod.rs

Concrete `ClientAdapter` implementations, one file per client.

Constructor Pattern: this file is the module declaration hub only —
no logic lives here. `jsonmcp` owns the shared JSON merge helpers
used by every JSON-keyed adapter (claude-code, cursor, zed).
`_registry` is the single canonical adapter list (v0.22).

## Related

- parent: `keisei/Cargo.toml`

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
