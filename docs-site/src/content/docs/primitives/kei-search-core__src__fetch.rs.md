---
title: fetch.rs
path: kei-search-core/src/fetch.rs
dna_hash: sha256:83d9d4cd893b79f8
language: rust
size_loc: 23
generated: by-keidocs
---

# kei-search-core/src/fetch.rs

Source fetcher trait — frozen interface, default impl is a no-op stub.

Actual WebFetch/WebSearch integration is out-of-scope for v0.14 part A.
Later milestones plug real providers (anthropic-websearch, SerpAPI, etc.).

## Public API

- `pub trait SourceFetcher` — Implement this trait to integrate a live search provider.
- Fetch sources for `claim`. Returns (source, cost_microcents).
- `pub struct StubFetcher` — Default stub — returns empty. Frozen interface, no runtime side-effects.

## Related

- parent: `kei-search-core/Cargo.toml`
- imports: crate

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
