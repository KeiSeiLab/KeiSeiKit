---
title: filesystem.rs
path: kei-store/src/filesystem.rs
dna_hash: sha256:b0c2072d687e363a
language: rust
size_loc: 172
generated: by-keidocs
---

# kei-store/src/filesystem.rs

FilesystemStore — local `.git` repo, no remotes.

Reuses git2 for branch/commit so behavior parity with remote stores is
maintained. `push`/`pull` are intentional no-ops.

v0.14.1 hardening: `full()` now rejects absolute paths and `..` components
(CVE-class: path traversal via MCP `write`/`read` tool inputs).

## Public API

- Reject absolute paths and any `..` component BEFORE joining.

## Related

- parent: `kei-store/Cargo.toml`
- imports: anyhow, crate, std

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
