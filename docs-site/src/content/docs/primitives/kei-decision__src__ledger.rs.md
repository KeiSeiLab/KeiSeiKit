---
title: ledger.rs
path: kei-decision/src/ledger.rs
dna_hash: sha256:d6a4f175b07d29fd
language: rust
size_loc: 71
generated: by-keidocs
---

# kei-decision/src/ledger.rs

Pre-fork ledger row writer — shells out to `kei-ledger fork` BEFORE
kei-spawn so each ranked action gets a "queued" row immediately. Useful
when /research output is piped straight into kei-decision execute and we
want every action visible in `kei-ledger list --status running` before
any agent boots.

## Public API

- `pub fn pre_fork_ledger` — Invoke `kei-ledger fork <id> <branch> --spec-sha <sha>` (blocking).
- Search PATH, then a known fallback under `~/Projects/KeiSeiKit/...`.

## Related

- parent: `kei-decision/Cargo.toml`
- imports: anyhow, serde, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
