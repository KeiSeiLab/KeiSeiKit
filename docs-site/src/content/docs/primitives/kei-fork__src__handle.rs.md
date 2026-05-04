---
title: handle.rs
path: kei-fork/src/handle.rs
dna_hash: sha256:cfac17dc54e8fd0c
language: rust
size_loc: 42
generated: by-keidocs
---

# kei-fork/src/handle.rs

`ForkHandle` value type + `ForkStatus` enum.

`ForkHandle` is the return of `create()` and each row of `list()`. Its
fields are derived from `.KEI_FORK_META.toml` plus the worktree path
on disk. The handle is `Clone`, `serde::Serialize`, and
`serde::Deserialize` so the CLI can emit JSON and downstream callers
can round-trip it without touching the TOML file.

## Public API

- `pub fn from_cli` — Parse CLI `--status` value. Returns `None` for unknown strings so

## Related

- parent: `kei-fork/Cargo.toml`
- imports: serde, std

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
