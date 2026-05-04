---
title: validate.rs
path: kei-pet/src/validate.rs
dna_hash: sha256:5afde07b0aba693f
language: rust
size_loc: 239
generated: by-keidocs
---

# kei-pet/src/validate.rs

Validation rules R1–R19.

`validate()` returns `Err(Vec<ValidationError>)` accumulating ALL errors,
not just the first. This lets `/pet-setup` and `kei-pet validate` surface
the full diagnostic in one pass.

## Public API

- `pub fn validate` — Run R1–R19. Returns `Err(Vec<ValidationError>)` on any failure.
- Recognised schedule grammar — strings that the runtime scheduler can act on.

## Related

- parent: `kei-pet/Cargo.toml`
- imports: crate, thiserror

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
