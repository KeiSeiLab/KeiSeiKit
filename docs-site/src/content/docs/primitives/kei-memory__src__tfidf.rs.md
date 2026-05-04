---
title: tfidf.rs
path: kei-memory/src/tfidf.rs
dna_hash: sha256:a86ea6c7303cfb6d
language: rust
size_loc: 195
generated: by-keidocs
---

# kei-memory/src/tfidf.rs

TF-IDF over session documents.

Constructor Pattern: one cube, one responsibility. Classical text
retrieval: tokens, TF, IDF, cosine similarity. Document = session_id.

Design: `index_document` no longer rebuilds IDF on every call (was
O(N·V) per insert). It marks `tokens.idf_dirty = 1`; readers
(analyze, patterns, similar) invoke `recompute_idf_if_stale` once.

## Public API

- `pub fn tokenise` — Tokenise free text into lowercase alphanumeric word stems (≥3 chars).
- `pub fn tf` — Compute term-frequencies for a single document.
- `pub fn index_document` — Record a document's tokens under `session_id`. Overwrites prior entry
- `pub fn recompute_idf` — Recompute the full IDF table unconditionally. Cheap for N < 10k sessions.
- `pub fn recompute_idf_if_stale` — Recompute IDF only if any token row is marked dirty. Returns `true` when
- `pub fn session_vector` — Fetch a session's (token → tf·idf) sparse vector.
- `pub fn query_vector` — Compute a TF·IDF vector for ad-hoc query text, using existing corpus IDF.
- Pull (session_id → tf·idf vector) for every session that shares at least
- `pub fn top_similar` — Return the top-k sessions by cosine similarity against `query`.

## Related

- parent: `kei-memory/Cargo.toml`
- imports: crate, regex, rusqlite, std

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
