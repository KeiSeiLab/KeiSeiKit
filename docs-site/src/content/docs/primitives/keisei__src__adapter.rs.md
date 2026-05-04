---
title: adapter.rs
path: keisei/src/adapter.rs
dna_hash: sha256:2a02ee9c0e8d2aaf
language: rust
size_loc: 99
generated: by-keidocs
---

# keisei/src/adapter.rs

Adapter trait + registry — the pluggable surface for AI clients.

Constructor Pattern: this file owns the trait + the "enumerate all
adapters" function + lookup-by-name helper. Each concrete adapter
lives in its own file under `adapters/`. `Scope` itself lives in
`scope.rs` (own file, own concept). The adapter list lives in
`adapters/_registry.rs` — this file delegates via `all()` so the
public API stays stable when adapters are added.

v0.21: trait gained `Scope` parameter — adapters with both host-wide
and per-project config surfaces (claude-code, cursor) can be driven
to either location from one code path. Adapters that only expose a
global config (continue, zed) declare `supported_scopes() = [User]`.

v0.22:
* `auto_scope()` — adapter-driven CWD heuristic that turns
`Scope::Auto` into a concrete `User` / `Project`. Default is
`Scope::User` (safe fallback); Claude Code + Cursor override.
* `post_attach_hint(brain, scope)` — templated hint so the CLI can
interpolate the brain's name and the resolved scope into the
client-specific reload instruction. Returns `String` (not
`&'static str`) to accommodate `format!(...)`.

## Public API

- Which scopes this adapter can write into.
- Resolve `Scope::Auto` into a concrete scope via adapter-specific CWD
- One-line instruction the CLI prints after a successful attach so
- Helper: does this adapter support the given scope?
- `pub fn all` — Enumerate all adapters the binary knows about, in priority order.
- `pub fn detect_active` — Return the first adapter whose `detect()` fires. `NoClientDetected`
- `pub fn by_name` — Look up an adapter by its `name()`. Used by the detach flow which

## Related

- parent: `keisei/Cargo.toml`
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
