---
title: _registry.rs
path: keisei/src/adapters/_registry.rs
dna_hash: sha256:f10b0a6b95c9486b
language: rust
size_loc: 32
generated: by-keidocs
---

# keisei/src/adapters/_registry.rs

Adapter registry — the ONE place to add a new `ClientAdapter`.

Constructor Pattern: single responsibility — own the concrete list of
adapters the binary knows about, in priority order. `adapter::all()`
delegates here so callers never have to edit two files when a fifth
adapter ships.

Adding a 5th adapter: create its file under `adapters/<name>.rs`,
register the module in `adapters/mod.rs`, and add one `Box::new(...)`
line below. That's it — `detect_active`, `by_name`, `list-adapters`,
`mount`, and `detach` all pick it up automatically.

Rationale for NOT using the `inventory` crate yet: at the 4→5 scale we
don't pay the dependency cost; a plain function is cheaper and easier
to audit.

## Public API

- `pub fn all_adapters` — Enumerate every adapter the binary knows about, in priority order.

## Related

- parent: `keisei/Cargo.toml`
- imports: crate

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
