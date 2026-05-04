---
title: store.rs
path: kei-chat-store/src/store.rs
dna_hash: sha256:cd132cb8d09bd063
language: rust
size_loc: 36
generated: by-keidocs
---

# kei-chat-store/src/store.rs

Chat store — thin shim over `kei_entity_store::Store`.

Multi-schema convergence (2026-04-23): BOTH `chat_messages` and
`chat_sessions` are now engine-owned. `Store::open` hands the engine
`ALL_SCHEMAS` so migrations for both tables run in a single
atomic transaction.

Verbs dispatch per-schema: callers that act on messages pass
`MESSAGES_SCHEMA`, callers that act on sessions pass
`SESSIONS_SCHEMA`. The only bespoke SQL left is the per-message
session-counter UPDATE (`sessions.rs::bump_session_totals`) — the
engine has no "aggregate-on-related-insert" verb.

## Related

- parent: `kei-chat-store/Cargo.toml`
- imports: anyhow, crate, kei_entity_store, rusqlite, std

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
