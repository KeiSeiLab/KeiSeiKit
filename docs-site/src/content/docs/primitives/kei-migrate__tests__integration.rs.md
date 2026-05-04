---
title: integration.rs
path: kei-migrate/tests/integration.rs
dna_hash: sha256:3110226e9fa9ddc0
language: rust
size_loc: 174
generated: by-keidocs
---

# kei-migrate/tests/integration.rs

Integration tests for kei-migrate against a SQLite file (safe, no deps).

SQLite is chosen as the test backend because it has no server dependency
and the sqlx-Any path through it exercises the same code path as Postgres
/ MySQL for everything except dialect-specific DDL (which we abstract in
`db::Backend::create_tracker_sql`).

## Related

- parent: `kei-migrate/tests`
- imports: kei_migrate, std, tempfile

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
