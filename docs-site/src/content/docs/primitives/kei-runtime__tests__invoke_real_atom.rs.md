---
title: invoke_real_atom.rs
path: kei-runtime/tests/invoke_real_atom.rs
dna_hash: sha256:3dc57b15479bd60f
language: rust
size_loc: 81
generated: by-keidocs
---

# kei-runtime/tests/invoke_real_atom.rs

Integration test — `kei-runtime invoke` actually executes `kei-task::create`.

Wire-up:
1. Pre-build `kei-task` in the workspace target dir.
2. Point the `--root` at the workspace's `_primitives/_rust/` so the
runtime discovers the real atom metadata (`kei-task/atoms/create.md`).
3. Point `KEI_RUNTIME_BIN_DIR` at the target dir so the runtime resolves
the `kei-task` binary without polluting $PATH.
4. Invoke → expect exit 0 and a JSON result containing `id` as integer.

## Public API

- Absolute path to `_primitives/_rust/` (the atom-discovery root).
- Build `kei-task` so the runtime can spawn it. Uses the current profile's

## Related

- parent: `kei-runtime/tests`
- imports: std

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
