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
