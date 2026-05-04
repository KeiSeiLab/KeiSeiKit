---
title: memory_nudge_smoke.rs
path: kei-cortex/tests/memory_nudge_smoke.rs
dna_hash: sha256:5420ddca509a9296
language: rust
size_loc: 104
generated: by-keidocs
---

# kei-cortex/tests/memory_nudge_smoke.rs

Smoke test for the periodic memory-nudge scheduler.

Constructor Pattern: one file = one scenario per test fn.
Drives the scheduler through 12 simulated turns, asserts:
* trigger fires at turn 10 (Hermes default interval),
* counter resets after fire,
* "Nothing to save." short-circuit is recognised by the
review-task path (no memory writes spawned for it).

## Related

- parent: `kei-cortex/tests`
- imports: kei_cortex, std, tokio

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
