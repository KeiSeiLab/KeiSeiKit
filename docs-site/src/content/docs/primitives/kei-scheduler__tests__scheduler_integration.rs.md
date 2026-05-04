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

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
