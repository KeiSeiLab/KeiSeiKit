---
title: error.rs
path: kei-cleanup/src/error.rs
dna_hash: sha256:2ab4be1e664868ca
language: rust
size_loc: 46
generated: by-keidocs
---

# kei-cleanup/src/error.rs

Error type for kei-cleanup runtime — see CLEANUP-RUNTIME-SPEC.md §3.3.

Scanner failures are non-fatal at the runtime level: a missing
external tool returns `ToolNotFound`, which the dispatcher records
as `scanners_skipped` rather than aborting the whole run.

## Public API

- Runtime error type — see CLEANUP-RUNTIME-SPEC.md §3.3.
- External CLI (cargo-machete / cargo-udeps) not on PATH.
- Failed to read or parse a Cargo.toml.
- Path that failed.
- Underlying io / toml error as string.
- Failed to walk workspace tree.
- Generic IO error.
- JSON serialisation failure (report writer).
- External tool returned non-zero exit but produced no parseable output.
- Tool name (e.g. "cargo-machete").
- Captured stderr or status detail.

## Related

- parent: `kei-cleanup/Cargo.toml`
- imports: std, thiserror

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
