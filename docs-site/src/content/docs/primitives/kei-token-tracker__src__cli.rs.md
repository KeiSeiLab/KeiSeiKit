---
title: cli.rs
path: kei-token-tracker/src/cli.rs
dna_hash: sha256:ce22156ca395caaa
language: rust
size_loc: 143
generated: by-keidocs
---

# kei-token-tracker/src/cli.rs

Clap dispatch for the `kei-token-tracker` CLI binary.

## Public API

- Print total event count.
- List most recent events (default 20).
- Aggregate by model since N days ago.
- Render the Phase D nightly markdown report.
- Optional output path; default = stdout.
- `pub fn run` — Entry point; returns a process exit code so the bin can exit cleanly.

## Related

- parent: `kei-token-tracker/Cargo.toml`
- imports: chrono, clap, crate, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
