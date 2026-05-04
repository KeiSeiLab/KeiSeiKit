---
title: skill_aggregator_cli.rs
path: kei-ledger/src/skill_aggregator_cli.rs
dna_hash: sha256:d0dddaa9f95ffc69
language: rust
size_loc: 88
generated: by-keidocs
---

# kei-ledger/src/skill_aggregator_cli.rs

CLI dispatch for the `aggregate-skills` subcommand.

Constructor Pattern: formatting logic lives here, not in `dispatch.rs`
(which stays under 200 LOC). Two output modes: JSON array and Markdown
table. Sorted by success_rate ascending (worst first) so the nightly
operator sees the most urgent skills at the top.

## Public API

- Seconds in 30 days (default lookback window).

## Related

- parent: `kei-ledger/Cargo.toml`
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
