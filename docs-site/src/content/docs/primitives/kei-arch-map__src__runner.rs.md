---
title: runner.rs
path: kei-arch-map/src/runner.rs
dna_hash: sha256:2538615ea254b4ad
language: rust
size_loc: 72
generated: by-keidocs
---

# kei-arch-map/src/runner.rs

Verify-run orchestrator. Dispatches each Claim to the matching
`evidence::*::check` function, prints a per-claim PASS/FAIL line,
and returns Err on any FAIL.

## Public API

- `pub fn run` — Run all claims in `plan_path`. Err if any FAIL.
- `pub fn check_claim` — Check a single claim. Returns (passed, reason_if_failed).

## Related

- parent: `kei-arch-map/Cargo.toml`
- imports: anyhow, kei_arch_map, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
