---
title: lib.rs
path: kei-frustration-loop/src/lib.rs
dna_hash: sha256:c199f982d8974068
language: rust
size_loc: 26
generated: by-keidocs
---

# kei-frustration-loop/src/lib.rs

Library surface for `kei-frustration-loop`.

Owns the per-user / online learning loop on top of the
`frustration-matrix` batch classifier:
* `persistence` — per-user firmware paths + atomic file swap
* `feedback`    — JSONL feedback log (Correct / Wrong / NewCategory)
* `nightly`     — Phase-0 nightly scan cron hook
* `bootstrap`   — install-time first-bake of per-user firmware
* `auto_train`  — feedback-threshold-triggered retrain

Constructor Pattern: this crate is the LOOP responsibility — different
lifecycle and consumer than `frustration-matrix` (the offline batch
classifier + n-gram firmware trainer). Re-exports only.

All five modules consume the public surface of `frustration-matrix`
(`categories::compile_all`, `Firmware`, `Firmware::train_from_dir`)
via path dependency, never via internal helpers.

## Related

- parent: `kei-frustration-loop/Cargo.toml`

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
