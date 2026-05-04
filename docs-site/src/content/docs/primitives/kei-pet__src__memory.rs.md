---
title: memory.rs
path: kei-pet/src/memory.rs
dna_hash: sha256:6882fc3897f6c91d
language: rust
size_loc: 167
generated: by-keidocs
---

# kei-pet/src/memory.rs

Persistent conversation memory indexed by (user_id, pet_name).

Each row is a single message exchange turn (role = "user" | "assistant" |
caller-defined). Storage is SQLite. No FTS: `search` is a simple LIKE scan
scoped by the (user_id, pet_name) tuple.

Scope boundary: this module does not open connections — the caller
supplies a `rusqlite::Connection` (on-disk or in-memory). That keeps the
module hermetically testable and lets the host choose the DB path.

## Public API

- Conversation stream identity: one stream per (user, pet) pair.
- A single recorded interaction row.
- Errors surfaced by this module.
- P2.1.b — content rejected by the injection check before persistence.
- `pub fn ensure_schema` — Create the `pet_conversations` table and its (user_id, pet_name, ts DESC)
- `pub fn record_interaction` — Insert one interaction row, returning its rowid.
- `pub fn recent` — Return up to `limit` most recent interactions for `tag`, newest first.
- `pub fn search` — Return up to `limit` interactions whose `text` contains `query` as a
- Escape LIKE metacharacters (`%`, `_`, `\`) so callers can pass raw text.

## Related

- parent: `kei-pet/Cargo.toml`
- imports: crate, rusqlite

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
