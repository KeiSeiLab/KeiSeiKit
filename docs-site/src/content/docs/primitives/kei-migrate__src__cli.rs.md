---
title: cli.rs
path: kei-migrate/src/cli.rs
dna_hash: sha256:7ccfccc904b3aa1e
language: rust
size_loc: 44
generated: by-keidocs
---

# kei-migrate/src/cli.rs

CLI surface — clap argument parsing for `kei-migrate`.

## Public API

- Database URL. Overrides $DATABASE_URL.
- Migrations directory (default: ./migrations)
- Apply all pending migrations.
- Revert the last N migrations (requires <ts>_<name>.down.sql).
- List applied vs pending migrations.
- Create a new timestamped migration scaffold: <ts>_<name>.sql (+ .down.sql).
- Short migration name, e.g. "add_users_email_index".

## Related

- parent: `kei-migrate/Cargo.toml`
- imports: clap

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
