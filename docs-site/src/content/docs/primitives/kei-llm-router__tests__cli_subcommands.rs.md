---
title: cli_subcommands.rs
path: kei-llm-router/tests/cli_subcommands.rs
dna_hash: sha256:25f9de79912735ee
language: rust
size_loc: 82
generated: by-keidocs
---

# kei-llm-router/tests/cli_subcommands.rs

Test 7 — clap parses all four subcommands.

Constructor Pattern: this test exists ONLY to lock the CLI surface;
a regression renaming a subcommand or dropping a flag will fail here
before it lands in production.

## Related

- parent: `kei-llm-router/tests`
- imports: clap, kei_llm_router

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
