---
title: main.rs
path: kei-decision/src/main.rs
dna_hash: sha256:05c2c5d2ad490e15
language: rust
size_loc: 163
generated: by-keidocs
---

# kei-decision/src/main.rs

kei-decision CLI entry point — clap parse + dispatch only.

Each subcommand has a thin `run_*` function declared inline below. Heavy
logic lives in the library modules (`parser`, `classifier`, `ranker`,
`emitter`, `executor`, `ledger`, `sleep_link`, `graph`).

## Related

- parent: `kei-decision/Cargo.toml`
- imports: clap, kei_decision, serde, std

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
