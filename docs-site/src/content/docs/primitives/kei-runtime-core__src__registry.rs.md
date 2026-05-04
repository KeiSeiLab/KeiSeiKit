---
title: registry.rs
path: kei-runtime-core/src/registry.rs
dna_hash: sha256:0904e3bb8166be65
language: rust
size_loc: 155
generated: by-keidocs
---

# kei-runtime-core/src/registry.rs

[`Registry`] — in-memory DNA-keyed registry for trait impls.

Each `kei-{compute,llm,git,...}-*` impl registers itself at startup
by handing the orchestrator a [`RegistryEntry`]. The registry refuses
anonymous impls (impl must satisfy [`HasDna`] via its `RegistryEntry`).

## Public API

- `pub struct RegistryEntry` — What every registered impl carries.
- The 12 trait surfaces. Each registry slot is keyed by `(kind, dna)`.
- Registry of all loaded impls. Cheap to clone (Arc-internal).

## Related

- parent: `kei-runtime-core/Cargo.toml`
- imports: crate, std

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
