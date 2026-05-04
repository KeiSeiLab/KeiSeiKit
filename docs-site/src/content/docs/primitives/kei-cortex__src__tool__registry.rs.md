---
title: registry.rs
path: kei-cortex/src/tool/registry.rs
dna_hash: sha256:eb14ddeb3e773f2f
language: rust
size_loc: 178
generated: by-keidocs
---

# kei-cortex/src/tool/registry.rs

`ToolRegistry` — name → executor dispatch table.

Each entry is a boxed async fn that takes the parsed `ToolCall.input`
JSON and returns either `String` (success content) or `ToolError`. The
registry is built once at daemon startup via `with_project_root(...)`;
the agentic loop borrows it for each turn.

Project-root chroot: tools that touch the filesystem (`read`, `write`,
`edit`, `glob`, `grep`) all enforce that requested paths resolve
INSIDE the configured `project_root`. The registry is constructed
once with the `project_root` captured into each tool's closure so
the path is immutable for the daemon's lifetime.

Constructor Pattern: this module owns ONLY the dispatch table. Each
tool's logic lives in its own sibling cube (`read.rs`, `write.rs`, …).

## Public API

- `pub type Executor` — Async executor signature. Each tool's implementation module exports a
- `pub struct ToolRegistry` — Registry of tools, keyed by Anthropic `tool_use.name`.
- `pub fn empty` — Empty registry with a project root. Useful for tests.
- `pub fn register` — Register a tool by name. Overwrites any existing entry for the
- `pub fn has` — True iff the registry knows this name.
- `pub fn names` — Names currently registered, sorted for deterministic listings.
- `pub fn project_root` — The project root captured at construction.
- Dispatch one call. Unknown names produce a `ToolResult` with
- `pub fn with_project_root` — Build the production registry with all 8 tools wired in,
- Default builds with `project_root = "."` (cwd). Production code

## Related

- parent: `kei-cortex/Cargo.toml`
- imports: futures, serde_json, std

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
