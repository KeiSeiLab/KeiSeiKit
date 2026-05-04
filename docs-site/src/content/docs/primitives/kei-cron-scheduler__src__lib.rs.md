---
title: lib.rs
path: kei-cron-scheduler/src/lib.rs
dna_hash: sha256:7243e52eaecb5c65
language: rust
size_loc: 19
generated: by-keidocs
---

# kei-cron-scheduler/src/lib.rs

P4.2 — Hermes-equivalent cron / at / interval scheduler.

Three-mode schedule parsing (one-shot duration, recurring interval, cron
expression, ISO timestamp) on top of a JSON-on-disk job store. Mirrors the
Hermes `cron/jobs.py:102-209` parsing surface 1:1 so existing operators can
migrate without re-learning the schedule grammar.

## Related

- parent: `kei-cron-scheduler/Cargo.toml`

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
