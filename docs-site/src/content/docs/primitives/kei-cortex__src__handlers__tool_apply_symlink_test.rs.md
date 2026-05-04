---
title: tool_apply_symlink_test.rs
path: kei-cortex/src/handlers/tool_apply_symlink_test.rs
dna_hash: sha256:34d4215f15c0dc1a
language: rust
size_loc: 100
generated: by-keidocs
---

# kei-cortex/src/handlers/tool_apply_symlink_test.rs

Wave 44b F-CRIT-4 regression tests for symlink-based escape attempts
against `tool_apply::apply`. Split out of `tool_apply_test.rs` to keep
the file under the Constructor Pattern 200-LOC ceiling. Includes the
shared helpers from the parent module via `use super::*`.

## Public API

- Outside path that the symlink will point at (a real file under another

## Related

- parent: `kei-cortex/Cargo.toml`
- imports: std, tempfile

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
