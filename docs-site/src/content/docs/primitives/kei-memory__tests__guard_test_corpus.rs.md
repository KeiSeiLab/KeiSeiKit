---
title: guard_test_corpus.rs
path: kei-memory/tests/guard_test_corpus.rs
dna_hash: sha256:8c10492ef70bf650
language: rust
size_loc: 180
generated: by-keidocs
---

# kei-memory/tests/guard_test_corpus.rs

Adversarial corpus for `injection_guard::scan`.

Constructor Pattern: one file = one concern (corpus + assertions).
50 samples across 5 categories — P2.1.b restores `base64_blobs` as
a Block-tier category (was Warn-only in the prior shape):

1. prompt_overrides    (10)
2. invisible_unicode   (10)
3. ssh_keys            (10)
4. curl_with_secret    (10)
5. base64_blobs        (10)   ← restored as Block-tier

Imports via the `kei_memory` lib (Wave B introduced `[lib]` target).

Acceptance: at minimum 45 of 50 samples must reach `Block` severity.

## Public API

- P2.1.b — base64 blobs >= 1024 chars on a single line are now

## Related

- parent: `kei-memory/tests`
- imports: kei_memory

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
