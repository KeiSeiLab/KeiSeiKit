---
title: main.rs
path: kei-model-router/src/main.rs
dna_hash: sha256:4063fe8a71eba1f0
language: rust
size_loc: 206
generated: by-keidocs
---

# kei-model-router/src/main.rs

kei-model-router CLI.

Subcommands:
pricing                — print verified pricing table (default)
select <agent> [--prompt P]
— query router decision for given agent
spawn. Reads ledger at $KEI_LEDGER_DB.
calibrate              — re-fit kernel weights against ledger
outcomes. Print baseline vs best MSE.
--help

## Public API

- Read `~/.claude/settings.json::router.pinned[agent]` if present.

## Related

- parent: `kei-model-router/Cargo.toml`
- imports: kei_model_router, rusqlite

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
