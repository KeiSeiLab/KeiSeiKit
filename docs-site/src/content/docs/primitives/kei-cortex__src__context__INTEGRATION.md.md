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

<script src="https://giscus.app/client.js"
        data-repo="KeiSei84/KeiSeiKit-1.0"
        data-repo-id="PLACEHOLDER_REPO_ID"
        data-category="wiki-comments"
        data-category-id="PLACEHOLDER_CATEGORY_ID"
        data-mapping="pathname"
        data-strict="0"
        data-reactions-enabled="1"
        data-emit-metadata="0"
        data-input-position="bottom"
        data-theme="preferred_color_scheme"
        data-lang="en"
        data-loading="lazy"
        crossorigin="anonymous"
        async></script>
