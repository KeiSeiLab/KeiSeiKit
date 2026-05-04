---
title: scope.rs
path: keisei/src/scope.rs
dna_hash: sha256:f37f68e3ada4394d
language: rust
size_loc: 62
generated: by-keidocs
---

# keisei/src/scope.rs

`Scope` — whether an attach targets the host-wide (User) config or the
project-local (Project) config for an AI client.

Constructor Pattern: single responsibility — a plain enum + its (de)serde
projection. No I/O, no adapter knowledge. Lives in its own file to keep
`adapter.rs` at one-concept (the trait itself).

v0.22: added `Scope::Auto` as the CLI default so
`cd team-repo; keisei attach <brain>` detects project-scope
automatically (if `./.claude/` or `./.cursor/` exists) without the user
having to type `--scope=project`. `Auto` is a CLI-level intent, never
persisted — `attach.rs` resolves it to concrete `User` / `Project` via
`adapter.auto_scope()` before writing the marker.

Default on deserialization is `Scope::User` so v0.20 markers (written
before this field existed) round-trip transparently.

## Public API

- Host-wide config — e.g. `~/.claude/settings.json`, `~/.cursor/mcp.json`.
- Project-local config — e.g. `./.claude/settings.json`, `./.cursor/mcp.json`.
- Ask the adapter to pick based on CWD heuristics. CLI-only intent —

## Related

- parent: `keisei/Cargo.toml`
- imports: serde, std

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
