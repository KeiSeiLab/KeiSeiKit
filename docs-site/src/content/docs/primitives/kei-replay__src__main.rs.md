---
title: main.rs
path: kei-replay/src/main.rs
dna_hash: sha256:7d106c0e73914deb
language: rust
size_loc: 114
generated: by-keidocs
---

# kei-replay/src/main.rs

kei-replay — CLI dispatcher.

Commands:
kei-replay <dna>            — reconstruct; print task.toml + prompt
kei-replay <dna> --verify   — also fail non-zero on body-hash drift
kei-replay diff <a> <b>     — compare two DNAs, print facet report

## Public API

- Override ledger DB path (default: $KEI_LEDGER_DB or ~/.claude/agents/ledger.sqlite)
- Reconstruct the spawn for a DNA string.
- DNA string: role::caps::scope::body-nonce
- Repo root holding _roles/ and _capabilities/ (default: cwd)
- Explicit task.toml path (skips ledger lookup for the file path)
- Fail with exit 2 when recomputed body hash differs from DNA.
- Diff two DNA strings facet-by-facet.

## Related

- parent: `kei-replay/Cargo.toml`
- imports: clap, kei_replay, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
