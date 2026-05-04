---
title: cli.rs
path: kei-frustration-loop/src/cli.rs
dna_hash: sha256:047d35a25a3dd1ea
language: rust
size_loc: 92
generated: by-keidocs
---

# kei-frustration-loop/src/cli.rs

CLI surface — clap parser + 5 loop subcommands + dispatch.

Constructor Pattern: this cube owns the clap `Cli`/`Cmd` definitions
and routes each variant to a runner in `runners.rs`. Keep <200 LOC.

## Public API

- Top-level CLI surface for the per-user frustration learning loop.
- Five loop subcommands. Order = the order they show in `--help`.
- First-install bootstrap — train per-user firmware + queue initial hits.
- Phase-0 nightly scan over chatlogs since `--since` (Unix seconds).
- Append one user-correction row to the per-user feedback log.
- Trigger per-user retrain when feedback log clears the threshold.
- Inspect which firmware will be used for `--user`.
- `pub fn dispatch` — Dispatch the parsed CLI to the matching runner.

## Related

- parent: `kei-frustration-loop/Cargo.toml`
- imports: anyhow, clap, crate, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
