---
title: pet.rs
path: kei-cortex/src/handlers/pet.rs
dna_hash: sha256:7611c8236d6a56e5
language: rust
size_loc: 117
generated: by-keidocs
---

# kei-cortex/src/handlers/pet.rs

Pet endpoints — read a persona manifest + record an interaction.

- `GET  /api/v1/cortex/pet/:user_id`
- `POST /api/v1/cortex/pet/:user_id/interaction`

The manifest lives on disk at `<pet_root>/<user_id>.toml`. Interactions
are written to the kei-pet SQLite memory store.

## Public API

- Response body for `GET /pet/:user_id`.
- Request body for `POST /pet/:user_id/interaction`.
- Response body for `POST /pet/:user_id/interaction`.
- Handler — load `<pet_root>/<user_id>.toml` into a `PetManifest`.
- Handler — append a single interaction row to the kei-pet memory DB.

## Related

- parent: `kei-cortex/Cargo.toml`
- imports: axum, crate, kei_pet, rusqlite, serde, std

## Discussion

<script src="https://giscus.app/client.js"
        data-repo="KeiSei84/KeiSeiKit-1.0"
        data-repo-id="PLACEHOLDER_REPO_ID"
        data-category="wiki-comments"
        data-category-id="PLACEHOLDER_CATEGORY_ID"
        data-mapping="pathname"
        data-strict="0"
        data-reactions-enabled="1"
        data-emit-metadata="0"
        data-input-position="bottom"
        data-theme="preferred_color_scheme"
        data-lang="en"
        data-loading="lazy"
        crossorigin="anonymous"
        async></script>
