---
title: sleep_link.rs
path: kei-decision/src/sleep_link.rs
dna_hash: sha256:1dd020e5a5ca1e24
language: rust
size_loc: 86
generated: by-keidocs
---

# kei-decision/src/sleep_link.rs

Sleep-layer Phase B integration: scan known research-source roots for
files newer than a `--since` timestamp and return them as a unified queue.

Today we look at two roots:
- `~/.keisei/memory/sync-repo/sleep-results/`  (Phase A incubation outputs)
- `~/Projects/KnowledgeVault/research/*/MASTER-REPORT.md`  (research outputs)

No parsing here — the caller can decide which entries to feed back through
`parse_master_report` / `rank_actions`.

## Public API

- `pub fn scan_research_sources` — Walk both known roots; return any `*.md` (sleep) or `MASTER-REPORT.md`

## Related

- parent: `kei-decision/Cargo.toml`
- imports: anyhow, serde, std, walkdir

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
