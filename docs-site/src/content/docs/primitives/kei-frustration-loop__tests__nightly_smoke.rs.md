---
title: nightly_smoke.rs
path: kei-frustration-loop/tests/nightly_smoke.rs
dna_hash: sha256:2653fd4a33839291
language: rust
size_loc: 100
generated: by-keidocs
---

# kei-frustration-loop/tests/nightly_smoke.rs

Nightly-scan smoke test. Build a tiny traces dir with three JSONL files,
one of which contains an `опять` user line that the regex SSoT
`repeat-signal` category must match. Assert ScanReport.hits >= 1 and
the queue file has at least one row.

## Related

- parent: `kei-frustration-loop/tests`
- imports: frustration_matrix, kei_frustration_loop, std, tempfile

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
