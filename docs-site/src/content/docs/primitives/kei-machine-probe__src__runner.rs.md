---
title: runner.rs
path: kei-machine-probe/src/runner.rs
dna_hash: sha256:43b66c0c6c85a705
language: rust
size_loc: 104
generated: by-keidocs
---

# kei-machine-probe/src/runner.rs

Runner trait — the seam every detector goes through.

Constructor Pattern: ALL `std::process::Command::new` lives here. Every
detector (arch / memory / gpu / os / tooling) accepts a `&dyn Runner`
so unit tests can substitute a fixture-backed mock without touching the
host system.

Mock layout: each command becomes a fixture file
`<sanitized-cmd>.stdout`. Sanitization replaces every byte outside
`[A-Za-z0-9._-]` with `_`. Example:
`sysctl -n hw.model` → `sysctl_-n_hw.model.stdout`
`which ollama`      → `which_ollama.stdout`

## Public API

- `pub trait Runner` — One run = one (cmd, args) → stdout. Failures map to `Err` (caller
- `pub struct SystemRunner` — Default impl — shells out to the real host.
- `pub fn fixture_stem` — Sanitize a `(cmd, args)` pair into a fixture filename stem.
- `pub struct MockRunner` — Test / CI runner that reads stdout from `<dir>/<stem>.stdout` files.
- Optional in-memory overrides keyed by sanitized stem (no extension).

## Related

- parent: `kei-machine-probe/Cargo.toml`
- imports: anyhow, std

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
