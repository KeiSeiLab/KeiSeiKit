---
title: INTEGRATION
path: kei-cortex/src/context/INTEGRATION.md
dna_hash: sha256:477b1f14f969e236
language: markdown
size_loc: 64
generated: by-keidocs
---

# kei-cortex/src/context/INTEGRATION.md

## Public API

- `context — orchestrator integration` — This module is **disjoint**: it does not import `chat.rs`, `persona.rs`, or
- `Required `AppConfig` additions (orchestrator-side)` — `config.rs` must expose `cwd: PathBuf` and `project_root: PathBuf` (both
- `Patch for `handlers/chat.rs::chat`` — ```rust
- `Surface guarantees` — - `discover` is pure I/O on the local filesystem. No network. Symlinks
- `Testing notes for the orchestrator` — When orchestrator-side tests stub `cwd` / `project_root`, point both at a

## Related

- parent: `kei-cortex/Cargo.toml`

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
