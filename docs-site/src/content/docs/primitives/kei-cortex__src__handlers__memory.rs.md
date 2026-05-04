---
title: memory.rs
path: kei-cortex/src/handlers/memory.rs
dna_hash: sha256:c696786f833b24ae
language: rust
size_loc: 103
generated: by-keidocs
---

# kei-cortex/src/handlers/memory.rs

`GET /api/v1/cortex/memory/search` — substring scan over pet memory.

Delegates to `kei_pet::memory::search` which implements a LIKE-scoped
query keyed by `(user_id, pet_name)`.

## Public API

- `pub const MAX_LIMIT` — Maximum allowed `limit`.
- `pub const DEFAULT_LIMIT` — Default `limit` when absent.
- Handler entry point.

## Related

- parent: `kei-cortex/Cargo.toml`
- imports: axum, crate, kei_pet, rusqlite, serde

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
