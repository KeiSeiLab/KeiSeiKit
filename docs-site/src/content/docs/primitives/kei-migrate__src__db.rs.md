---
title: db.rs
path: kei-migrate/src/db.rs
dna_hash: sha256:1dbaa11083bc1321
language: rust
size_loc: 68
generated: by-keidocs
---

# kei-migrate/src/db.rs

Database backend detection + pool construction.

Uses `sqlx::Any` so one binary covers Postgres / SQLite / MySQL.
Detection is purely on URL scheme — no live probe needed.

## Public API

- Backend inferred from the URL scheme. Determines dialect quirks.
- `pub fn create_tracker_sql` — Backend-specific CREATE TABLE for `_kei_migrations`.
- `pub fn detect_backend` — Parse a database URL into a [`Backend`]. Never touches the network.
- Build a sqlx `AnyPool` for the given URL (max 4 conns — migration runner is not a server).

## Related

- parent: `kei-migrate/Cargo.toml`
- imports: anyhow, sqlx

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
