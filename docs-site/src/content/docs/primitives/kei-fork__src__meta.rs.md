---
title: meta.rs
path: kei-fork/src/meta.rs
dna_hash: sha256:8123114e68e8041a
language: rust
size_loc: 49
generated: by-keidocs
---

# kei-fork/src/meta.rs

`.KEI_FORK_META.toml` — on-disk metadata written once by `create()`
and read by `list()` / `collect()` / `rescue()` / `gc()`.

Layout is stable: `agent_id`, `started_ts`, `base_branch`, `ledger_id`.
Never add fields without bumping a schema version.

## Related

- parent: `kei-fork/Cargo.toml`
- imports: crate, serde, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
