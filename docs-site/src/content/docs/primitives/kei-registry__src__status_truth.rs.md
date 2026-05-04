---
title: status_truth.rs
path: kei-registry/src/status_truth.rs
dna_hash: sha256:2236bad90ef5fdfd
language: rust
size_loc: 189
generated: by-keidocs
---

# kei-registry/src/status_truth.rs

Phase 3 Layer 3 — STATUS-TRUTH MARKER → kei-registry pipe.

Constructor Pattern: parse RULE 0.16 marker blocks + insert one row
into `cleanup_findings`. Schema is provisioned via CREATE TABLE IF
NOT EXISTS so the cube is self-contained — no global migration bump.

## Related

- parent: `kei-registry/Cargo.toml`
- imports: anyhow, rusqlite, serde

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
