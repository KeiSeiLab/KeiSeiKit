---
title: register_types.rs
path: kei-registry/tests/register_types.rs
dna_hash: sha256:1b1357259f016ff0
language: rust
size_loc: 134
generated: by-keidocs
---

# kei-registry/tests/register_types.rs

Unit tests for per-type DNA computation and idempotency.

Covers: BlockMdScanner, CapabilityScanner, RoleScanner — each scanner
returns a Found with BlockType::Atom. Idempotency: re-register → no-op.

## Related

- parent: `kei-registry/tests`
- imports: kei_registry, std, tempfile

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
