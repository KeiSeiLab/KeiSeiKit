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

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
