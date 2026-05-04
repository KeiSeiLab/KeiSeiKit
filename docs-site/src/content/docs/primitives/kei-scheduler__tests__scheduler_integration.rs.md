---
title: scheduler_integration.rs
path: kei-scheduler/tests/scheduler_integration.rs
dna_hash: sha256:d6d6477ff4582df8
language: rust
size_loc: 189
generated: by-keidocs
---

# kei-scheduler/tests/scheduler_integration.rs

Integration tests for kei-scheduler. Uses `Store::open_memory` so
each test owns a throwaway DB and a deterministic wall clock
(`now` passed explicitly where the API allows).

`schedule()` + `cancel()` internally read `Utc::now()` once; that's
fine because we check relative ordering (`next_run_at` compared to
a post-call `Utc::now()` lower bound), not absolute values.

## Related

- parent: `kei-scheduler/tests`
- imports: chrono, kei_scheduler

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
