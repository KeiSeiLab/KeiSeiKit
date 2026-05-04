---
title: sessions.rs
path: kei-chat-store/src/sessions.rs
dna_hash: sha256:ff3d118d9a792a69
language: rust
size_loc: 134
generated: by-keidocs
---

# kei-chat-store/src/sessions.rs

Session + message operations.

Multi-schema convergence (2026-04-23): BOTH sessions and messages
now flow through `kei_entity_store::verbs::*`. `start_session` uses
`create` against `SESSIONS_SCHEMA` (TextPk + TextArchiveEnum);
`archive_session` uses `archive`; `get_session` uses `get`;
`save_message` uses `create` against `MESSAGES_SCHEMA`.

Only the per-message aggregate update on `chat_sessions`
(message_count / total_tokens / total_cost) stays bespoke — the
engine has no "update-on-related-insert" verb.

## Public API

- Bespoke aggregate update — engine has no "increment-on-related-insert"

## Related

- parent: `kei-chat-store/Cargo.toml`
- imports: anyhow, chrono, crate, kei_entity_store, rusqlite, serde, serde_json

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
