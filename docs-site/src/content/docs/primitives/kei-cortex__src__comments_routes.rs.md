---
title: comments_routes.rs
path: kei-cortex/src/comments_routes.rs
dna_hash: sha256:64fc0fb6c430752f
language: rust
size_loc: 152
generated: by-keidocs
---

# kei-cortex/src/comments_routes.rs

Sovereign-comment HTTP surface — `/api/v1/cortex/comments/*`.

Backed by sibling `kei-comments` primitive. Auth is supplied by the
existing cortex Bearer middleware (`routes_auth::require_bearer`)
when this router is merged into `build_api_router`.

Constructor Pattern split:
* `comments_routes.rs` — router + async handlers (this file)
* `comments_routes_init.rs` — store bootstrap + validators

`CommentStore` wraps a rusqlite `Connection` which is `!Sync`, so the
handle uses `std::sync::Mutex` and every handler defers the SQLite
work to `tokio::task::spawn_blocking`.

## Public API

- `pub fn comments_router` — Construct the comments sub-router. Caller merges it into the main

## Related

- parent: `kei-cortex/Cargo.toml`
- imports: axum, crate, kei_comments, serde, serde_json

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
