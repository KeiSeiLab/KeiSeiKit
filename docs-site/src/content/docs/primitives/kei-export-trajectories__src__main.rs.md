---
title: main.rs
path: kei-export-trajectories/src/main.rs
dna_hash: sha256:17a6fc890bea13ec
language: rust
size_loc: 152
generated: by-keidocs
---

# kei-export-trajectories/src/main.rs

kei-export-trajectories CLI.

Subcommands:
export --from-ts <ISO> --output <path.jsonl>
count  --from-ts <ISO>
verify <path.jsonl>

The `verify` command re-reads the JSONL we just wrote and confirms
the union-of-tool-stats invariant — it's how Phase 0.2 acceptance is
checked in CI without a separate Python reader.

## Public API

- Path to kei-ledger.sqlite. Defaults to
- Path to kei-memory.sqlite. Defaults to
- Repo root for resolving `.claude/agents/<id>/chatlog.md`.
- Emit ShareGPT JSONL for every agent with started_ts >= --from-ts.
- Count agents matching the same predicate; print to stdout.
- Re-read a JSONL and confirm key-set invariants.
- Parse ISO-8601 (date or full timestamp) into Unix epoch seconds.

## Related

- parent: `kei-export-trajectories/Cargo.toml`
- imports: anyhow, clap, kei_export_trajectories, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
