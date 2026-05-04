---
title: registry.rs
path: kei-model/src/registry.rs
dna_hash: sha256:d897b93cc8deb33c
language: rust
size_loc: 83
generated: by-keidocs
---

# kei-model/src/registry.rs

`Registry` — load `models.toml`, expose query helpers.

Resolution order for the catalog path:
1. Explicit `--models-toml <path>` flag (caller-supplied)
2. `KEI_MODEL_REGISTRY` env var
3. `<CARGO_MANIFEST_DIR>/data/models.toml` (compiled-in default)

The crate ships `data/models.toml` next to its `Cargo.toml`, so the
compiled-in default works for both `cargo run` and a copied release binary
provided the binary is invoked from inside the repo. Downstream callers
should pass `--models-toml` or set the env var explicitly.

## Public API

- `pub fn load` — Load the catalog from a path, returning a parsed registry.
- `pub fn resolve_path` — Resolve catalog path: arg → env → compiled-in default.

## Related

- parent: `kei-model/Cargo.toml`
- imports: anyhow, crate, serde, std

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
