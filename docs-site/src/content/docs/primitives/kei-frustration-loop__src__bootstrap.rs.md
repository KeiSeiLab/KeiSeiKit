---
title: bootstrap.rs
path: kei-frustration-loop/src/bootstrap.rs
dna_hash: sha256:87615ebcf2f01c45
language: rust
size_loc: 133
generated: by-keidocs
---

# kei-frustration-loop/src/bootstrap.rs

Install-time bootstrap — first scan of existing chatlogs + first
per-user firmware bake.

Idempotent: if `<user>.firmware.gz` already exists, we return without
re-training. The caller is expected to call this exactly once at
install time, then only rely on `auto_train` thereafter.

Constructor Pattern: this cube wires existing primitives only —
`frustration_matrix::firmware_corpus` for corpus loading,
`frustration_matrix::firmware` for training, `persistence` for atomic
write, `nightly` for the initial scan.

## Public API

- Outcome of a bootstrap call.
- User identifier the firmware was baked for.
- Absolute path of the per-user firmware on disk after bootstrap.
- Number of trace files visited during the initial scan.
- Number of category hits the initial scan emitted into the queue.
- True iff a per-user firmware already existed (no retraining done).
- `pub fn bootstrap` — Run the install-time bootstrap for `user` against `traces_dir`.
- Load corpus from `traces_dir`, train firmware, atomic-write to `dest`.
- Save the firmware to a temp sibling then `rename` over `dest`.
- Initial scan over the WHOLE traces dir (since_ts = 0).
- Build the `<dest>.tmp` sibling.
- Wall-clock now in Unix seconds. 0 if the system clock is broken.

## Related

- parent: `kei-frustration-loop/Cargo.toml`
- imports: anyhow, crate, frustration_matrix, serde, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
