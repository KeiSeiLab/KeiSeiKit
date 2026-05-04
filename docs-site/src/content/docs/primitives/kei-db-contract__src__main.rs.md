---
title: main.rs
path: kei-db-contract/src/main.rs
dna_hash: sha256:51b00a434d5e5930
language: rust
size_loc: 68
generated: by-keidocs
---

# kei-db-contract/src/main.rs

kei-db-contract — CLI entrypoint.

Diffs SQL migrations against TypeScript types in a project root.
Exit 0 when --strict not set OR no drift; exit 1 in --strict with drift; exit 2 on I/O error.

## Public API

- Project root.
- Migrations directory (relative to project root).
- TS source root (relative to project root). Walked recursively for `.ts`/`.tsx`.
- Output format.
- Exit 1 if drift_count > 0.

## Related

- parent: `kei-db-contract/Cargo.toml`
- imports: clap, kei_db_contract, std

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
