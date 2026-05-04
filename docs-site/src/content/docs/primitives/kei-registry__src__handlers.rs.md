---
title: handlers.rs
path: kei-registry/src/handlers.rs
dna_hash: sha256:2ec1ffea74512db7
language: rust
size_loc: 322
generated: by-keidocs
---

# kei-registry/src/handlers.rs

CLI command handlers.

Constructor Pattern: this cube wires CLI args to library calls. Each
handler is a thin adapter. The common parts (db path resolve,
id-or-DNA lookup) live in sibling cubes (`paths.rs`, `lookup.rs`).
Schema-version mismatch surfaces as exit 3, not-found 2, IO 1.

## Public API

- Exit-code outcome. `Ok` for success, plus typed not-found variant.
- `pub fn dispatch` — Dispatch one parsed Command. Returns Outcome → main maps to exit code.

## Related

- parent: `kei-registry/Cargo.toml`
- imports: anyhow, crate, serde_json, std

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
