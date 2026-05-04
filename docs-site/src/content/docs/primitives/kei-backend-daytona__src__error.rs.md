---
title: error.rs
path: kei-backend-daytona/src/error.rs
dna_hash: sha256:45cb6a2163922ddf
language: rust
size_loc: 52
generated: by-keidocs
---

# kei-backend-daytona/src/error.rs

Error types for the Daytona backend.

All public APIs return `Result<_, DaytonaError>`. No panics outside tests.

## Public API

- Top-level error variant.
- 401/403 from the API — bad/missing API key.
- 404 — sandbox does not exist.
- 429/503 — caller should retry; we surface after exhausting retries.
- reqwest transport failure (DNS, TLS, timeout).
- JSON serialization/deserialization failed.
- Any non-retriable HTTP error not covered above.
- `pub type Result` — Result alias used throughout the crate.

## Related

- parent: `kei-backend-daytona/Cargo.toml`
- imports: std

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
