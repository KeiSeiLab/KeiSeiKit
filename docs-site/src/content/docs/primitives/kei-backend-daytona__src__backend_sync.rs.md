---
title: backend_sync.rs
path: kei-backend-daytona/src/backend_sync.rs
dna_hash: sha256:79100c5479459643
language: rust
size_loc: 75
generated: by-keidocs
---

# kei-backend-daytona/src/backend_sync.rs

Push/pull helpers used by `DaytonaBackend`'s lifecycle hooks.

Kept in a separate cube so `backend.rs` stays under the 200-LOC
Constructor-Pattern limit. The functions here accept the pieces they
need explicitly — they do NOT borrow `DaytonaBackend` — so they can be
unit-tested without the full backend assembled.

Sync errors are logged via `eprintln!` and swallowed; the lifecycle
does not abort because of a sync failure (the sandbox is still usable
and a later retry can resync).

## Public API

- Sentinel file pulled back from the sandbox on `release(persist=true)`.
- Push the configured local tree into the sandbox.
- Pull the sentinel marker file back from the sandbox.

## Related

- parent: `kei-backend-daytona/Cargo.toml`
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
