---
title: backend.rs
path: kei-backend-daytona/src/backend.rs
dna_hash: sha256:6dc3fe7b25205475
language: rust
size_loc: 163
generated: by-keidocs
---

# kei-backend-daytona/src/backend.rs

High-level resume-or-create logic on top of `DaytonaClient`.

Defines a minimal local `Backend` trait. When a shared backend trait
crate lands, this trait can be replaced by a re-export — the method
shapes are intentionally generic.

## Public API

- Opaque handle returned by `Backend::acquire`.
- Optional file-sync configuration plumbed via `DaytonaBackend::with_sync`.
- Minimal sandbox-lifecycle trait, modelled after Hermes' BaseEnvironment
- Resume an existing sandbox keyed by `task_id`, or create a new one.
- `persist=true` → stop (preserve filesystem); `false` → delete.
- Execute a single bash command inside the sandbox.
- Concrete Daytona backend wired to a `DaytonaClient`.
- `pub fn new` — Build with an explicit client + default resources.
- `pub fn with_resources` — Override default resources.
- `pub fn with_sync` — Configure bidirectional file-sync with the sandbox.
- `pub fn client` — Borrow the underlying low-level client (file uploads, etc.).
- Sandbox name from a task id. Mirrors Hermes `f"hermes-{task_id}"`.
- Build a CreateSandboxSpec for first-time creation.
- Resume-or-create state machine. Returns the live sandbox.
- Bring an existing sandbox to Running, regardless of prior state.

## Related

- parent: `kei-backend-daytona/Cargo.toml`
- imports: async_trait, crate, std

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
