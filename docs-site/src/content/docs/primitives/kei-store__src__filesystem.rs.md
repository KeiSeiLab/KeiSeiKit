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

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
