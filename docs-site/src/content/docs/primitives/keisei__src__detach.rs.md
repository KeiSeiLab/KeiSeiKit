---
title: detach.rs
path: keisei/src/detach.rs
dna_hash: sha256:b9f6582d08a5c7fb
language: rust
size_loc: 95
generated: by-keidocs
---

# keisei/src/detach.rs

`keisei detach` implementation.

Constructor Pattern: single responsibility — read the marker,
iterate recorded attachments (each carrying its own `brain_name`,
`scope`, `client_type`), call `adapter.detach(brain_name, scope)` on
each, delete the marker file after all adapters succeed. Per-adapter
failures are collected and reported but do NOT abort the other
detaches — partial detach is better than stuck state.

v0.22: marker is now v4 (per-attachment `brain_path` + `brain_name`);
detach iterates each `Attachment` directly rather than reading a
single top-level `brain_name`. Multi-brain markers detach ALL
attachments by default.

## Public API

- For each attachment in the marker, run `adapter.detach(brain_name, scope)`.

## Related

- parent: `keisei/Cargo.toml`
- imports: crate

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
