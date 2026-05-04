---
title: schema.rs
path: kei-pet/src/schema.rs
dna_hash: sha256:9069e6e8e3cb138b
language: rust
size_loc: 317
generated: by-keidocs
---

# kei-pet/src/schema.rs

Schema types for pet.toml.

Enums use `#[serde(rename_all = "kebab-case")]` to match the TOML wire
format (e.g. `"mirror-user"`). Optional fields use `Option<T>` and are
omitted on serialize when `None`. Arrays default to `Vec::new()`.

## Public API

- Schema version. Must be `1` for this crate.

## Related

- parent: `kei-pet/Cargo.toml`
- imports: serde

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
