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

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
