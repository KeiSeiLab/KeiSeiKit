---
title: round_trip.rs
path: kei-diff/tests/round_trip.rs
dna_hash: sha256:b3fd3f792f728ce1
language: rust
size_loc: 278
generated: by-keidocs
---

# kei-diff/tests/round_trip.rs

Integration tests for kei-diff.

Core property: `apply(old, diff(old, new)) == new` for every fixture.
Plus edge cases on pointer escaping, array edits, apply errors, and
the RFC 6902 wire format.

## Related

- parent: `kei-diff/tests`
- imports: kei_diff, serde_json

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
