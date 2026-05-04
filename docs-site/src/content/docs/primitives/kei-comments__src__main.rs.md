---
title: main.rs
path: kei-comments/src/main.rs
dna_hash: sha256:dcb317dde72937cf
language: rust
size_loc: 123
generated: by-keidocs
---

# kei-comments/src/main.rs

kei-comments — CLI dispatcher for the comment store.
Default DB: `~/.keisei/comments.sqlite`. Override via `--db <path>`.

## Public API

- Override DB path (default: ~/.keisei/comments.sqlite or $KEI_COMMENTS_DB).
- Create the schema if missing.
- Post a new comment.
- List all comments for a page (JSON array on stdout).
- Soft-delete a comment (only author may delete).
- Toggle a reaction on. Idempotent.
- Remove a reaction. Idempotent.
- Print reactions map (emoji → [authors]) for a comment as JSON.

## Related

- parent: `kei-comments/Cargo.toml`
- imports: anyhow, clap, kei_comments, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
