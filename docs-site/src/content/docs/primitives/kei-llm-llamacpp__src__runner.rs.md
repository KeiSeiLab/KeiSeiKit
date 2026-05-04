---
title: runner.rs
path: kei-llm-llamacpp/src/runner.rs
dna_hash: sha256:d99d0c8f5afee00d
language: rust
size_loc: 169
generated: by-keidocs
---

# kei-llm-llamacpp/src/runner.rs

Runner trait — the ONLY surface that owns subprocess lifecycle.

All `tokio::process::Command` invocations flow through this trait.
Tests inject `MockRunner` which returns canned `RunOutput` from a
fixture queue; production uses `RealRunner` which spawns the binary.

Mirrors the Wave 56 kei-machine-probe pattern. Uses Rust 1.75+
native `async fn in trait` (no `async-trait` dep) — workspace
`rust-version = "1.75"` permits this.

## Public API

- Result of a one-shot `<bin> <args>` invocation.
- Handle to a spawned `llama-server` (or any long-lived child).
- Mock-mode kill flag — flipped on Drop when no real child held.
- `pub fn from_child` — Construct a real handle backed by a tokio Child.
- `pub fn mock` — Mock-mode constructor: no child held; Drop flips `flag` to true.
- Pinned-future return type to keep the Runner trait object-safe.
- `pub trait Runner` — All process spawns must implement this trait.
- Run `<bin> <args>` to completion, capturing stdout+stderr.
- Spawn `<bin> <args>` and collect stdout line-by-line.
- Spawn `<bin> <args>` as a long-lived child; return a handle.
- `pub struct RealRunner` — Production runner — real `tokio::process::Command` invocations.
- `pub fn bin_in_path` — Path-or-name resolver — used by `which(1)`-style discovery in

## Related

- parent: `kei-llm-llamacpp/Cargo.toml`
- imports: crate, std, tokio

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
