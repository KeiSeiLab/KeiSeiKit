---
title: classifier.rs
path: kei-memory/src/classifier.rs
dna_hash: sha256:5ae2a6bcf3914cd3
language: rust
size_loc: 172
generated: by-keidocs
---

# kei-memory/src/classifier.rs

Event-class classifier — replaces ingest::classify_default.

Constructor Pattern: this cube only emits a class label.
Persistence + extraction live elsewhere. Order-of-precedence is
intentional and documented in `classify` — most specific first.

Wave A motive — old `classify_default` had three hardcoded substring
checks (permission_denied / worktree_error / cargo_workspace) and no
explicit table. Hard to extend, hard to test, no recurrence-class
support for "user_correction" / "retry_loop" patterns the audit
self-loop relies on.

## Public API

- Pre-compiled regex set. Lazy-initialised on first `classify` call.
- `pub fn classify` — Classify one event into a stable label.

## Related

- parent: `kei-memory/Cargo.toml`
- imports: regex, std

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
