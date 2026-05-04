---
title: skill_aggregator_cli.rs
path: kei-ledger/src/skill_aggregator_cli.rs
dna_hash: sha256:d0dddaa9f95ffc69
language: rust
size_loc: 88
generated: by-keidocs
---

# kei-ledger/src/skill_aggregator_cli.rs

CLI dispatch for the `aggregate-skills` subcommand.

Constructor Pattern: formatting logic lives here, not in `dispatch.rs`
(which stays under 200 LOC). Two output modes: JSON array and Markdown
table. Sorted by success_rate ascending (worst first) so the nightly
operator sees the most urgent skills at the top.

## Public API

- Seconds in 30 days (default lookback window).

## Related

- parent: `kei-ledger/Cargo.toml`
- imports: crate, rusqlite, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
