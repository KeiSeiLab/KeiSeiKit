---
title: index_substrate.rs
path: kei-registry/src/index_substrate.rs
dna_hash: sha256:ca109a809b010838
language: rust
size_loc: 129
generated: by-keidocs
---

# kei-registry/src/index_substrate.rs

`index-substrate` subcommand — bulk registration of all kit substrate dirs.

Constructor Pattern: this cube owns the multi-type substrate walk. It
delegates per-type scanning to existing scanner adapters (Primitive,
Skill, Hook inside kit/hooks, Atom, plus BlockMd, Capability, Role).
Each found artefact is forwarded to `registry::register` with idempotency.
Output: per-type count table printed as JSON.

## Public API

- Per-type counts for `index-substrate` output.
- `pub fn handle_index_substrate` — Top-level handler for `index-substrate`.

## Related

- parent: `kei-registry/Cargo.toml`
- imports: anyhow, crate, rusqlite, serde, serde_json, std

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
