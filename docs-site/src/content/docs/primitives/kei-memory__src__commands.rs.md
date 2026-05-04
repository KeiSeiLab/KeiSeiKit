---
title: commands.rs
path: kei-memory/src/commands.rs
dna_hash: sha256:738f88c0db5df472
language: rust
size_loc: 123
generated: by-keidocs
---

# kei-memory/src/commands.rs

Command handlers — one function per CLI subcommand.

Constructor Pattern: each handler <30 LOC, single responsibility.
Pulled out of main.rs to keep the dispatcher under the 200 LOC limit.

## Related

- parent: `kei-memory/Cargo.toml`
- imports: crate, rusqlite, std

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
