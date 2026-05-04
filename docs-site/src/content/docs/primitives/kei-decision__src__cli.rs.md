---
title: cli.rs
path: kei-decision/src/cli.rs
dna_hash: sha256:529a48bbfc98e502
language: rust
size_loc: 70
generated: by-keidocs
---

# kei-decision/src/cli.rs

CLI shapes for `kei-decision`. Five subcommands, dispatched in `main.rs`.

Exit codes (per spec):
0 — success
1 — file / IO error
2 — no actions found / parse error
3 — kei-spawn invocation failed

## Public API

- Parse a master report and emit JSON of raw actions (with kind).
- Path to MASTER-REPORT.md.
- Parse + classify + topo-sort + score-rank.
- Truncate to top N (default: all).
- Emit markdown table instead of JSON.
- Parse + rank + emit one task.toml per action under <out>/.
- Full chain: parse + rank + emit + kei-spawn + (optional) kei-ledger.
- Skip kei-spawn invocation; only emit task files.
- Truncate to top N (default: all).
- Skip pre-fork ledger row.
- Scan a research dir tree, ingest each MASTER-REPORT.md into a cumulative
- Root directory to walk (e.g. ~/Projects/KnowledgeVault/research).

## Related

- parent: `kei-decision/Cargo.toml`
- imports: clap, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
