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

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
