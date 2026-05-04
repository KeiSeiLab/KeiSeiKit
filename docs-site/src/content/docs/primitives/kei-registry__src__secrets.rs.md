---
title: secrets.rs
path: kei-registry/src/secrets.rs
dna_hash: sha256:ae76a1b6b60f97f5
language: rust
size_loc: 152
generated: by-keidocs
---

# kei-registry/src/secrets.rs

Secret-reference orphan detector.

Reads env-var NAMES from `.env` files (never values), greps the kit
tree for usages, returns a `SecretsReport` with per-key usage counts
and orphan list. Constructor Pattern: pure read-side cube.

## Public API

- Top 5 files where the key appears.
- Scan `scan_root`, returning scanned_files count and per-key (count, files) map.
- `pub fn compute_secrets_report` — Build a `SecretsReport`. Pure: no side effects beyond file reads.
- `pub fn render_ascii` — Render a `SecretsReport` as ASCII text.

## Related

- parent: `kei-registry/Cargo.toml`
- imports: anyhow, regex, serde, std, walkdir

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
