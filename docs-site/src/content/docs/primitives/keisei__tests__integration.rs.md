---
title: integration.rs
path: keisei/tests/integration.rs
dna_hash: sha256:7179cce757ef705a
language: rust
size_loc: 1370
generated: by-keidocs
---

# keisei/tests/integration.rs

Integration tests for the `keisei` CLI primitives.

Constructor Pattern: one scenario per test, one assertion target.
Each test runs with `KEISEI_HOME` pointed at a tempdir so nothing
touches the real `~/.claude` / `~/.keisei`.

Sources are loaded via `#[path]` — mirror of the kei-ledger pattern.

## Public API

- Variant of `setup_home` that does NOT pre-create the `.claude` dir.
- Path the Claude-Code adapter writes at user scope, given the
- Write a brain manifest with a caller-chosen `mcp_server` string. Used
- Write a brain manifest with a caller-chosen `name`. Used to exercise
- Write a brain with an artificially large manifest.toml for the size-
- Write a schema-v2 brain manifest carrying every supported platform in
- Write a v2 brain that only has `linux-x64` — used on macOS to exercise
- Write a marker file directly with a brain_name containing ANSI control
- Write a distinct second brain manifest under `root` so multi-brain

## Related

- parent: `keisei/tests`
- imports: crate, serde_json, std, tempfile

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
