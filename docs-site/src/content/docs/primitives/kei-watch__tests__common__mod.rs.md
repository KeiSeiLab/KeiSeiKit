---
title: mod.rs
path: kei-watch/tests/common/mod.rs
dna_hash: sha256:794dd0357e652e95
language: rust
size_loc: 54
generated: by-keidocs
---

# kei-watch/tests/common/mod.rs

Shared helpers for the integration tests.

`tests/` files are separate crates; common code lives under
`tests/common/mod.rs` per cargo convention (not a top-level
`tests/common.rs`, which would itself be a test binary).

## Public API

- `pub fn wait_for` — Pull events until one matches `pred` or the global timeout elapses.
- `pub fn same_path` — macOS reports paths under `/private/var/...`; tempdirs live at `/var/...`.

## Related

- parent: `kei-watch/tests/common`
- imports: kei_watch, std

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
