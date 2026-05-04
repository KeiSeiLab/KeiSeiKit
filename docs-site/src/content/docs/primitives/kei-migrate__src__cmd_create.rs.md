---
title: cmd_create.rs
path: kei-migrate/src/cmd_create.rs
dna_hash: sha256:33559c4d1e22ff6a
language: rust
size_loc: 44
generated: by-keidocs
---

# kei-migrate/src/cmd_create.rs

`kei-migrate create <name>` — scaffold a new timestamped migration pair.

## Public API

- `pub fn run` — Create `<dir>/<utc-timestamp>_<sanitized-name>.sql` + `.down.sql`. Returns paths written.

## Related

- parent: `kei-migrate/Cargo.toml`
- imports: anyhow, chrono, std

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
