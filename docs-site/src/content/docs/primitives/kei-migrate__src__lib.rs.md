---
title: lib.rs
path: kei-migrate/src/lib.rs
dna_hash: sha256:e7c0dba52271ef63
language: rust
size_loc: 54
generated: by-keidocs
---

# kei-migrate/src/lib.rs

kei-migrate — universal SQL migration runner.

Single binary, three backends (Postgres / SQLite / MySQL) autodetected
from `DATABASE_URL`. Sequential `.sql` files in `migrations/`, tracked in
`_kei_migrations` with SHA-256 checksums.

Library surface exists so integration tests can drive the primitive
without `process::Command` gymnastics.

## Public API

- End-to-end `up` entry: connect, ensure tracker, scan dir, apply pending.
- End-to-end `down` entry: revert last N applied.
- End-to-end `status` entry: returns (applied, pending) counts.

## Related

- parent: `kei-migrate/Cargo.toml`
- imports: anyhow, std

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
