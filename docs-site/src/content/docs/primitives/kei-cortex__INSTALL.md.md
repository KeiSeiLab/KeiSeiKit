---
title: INSTALL
path: kei-cortex/INSTALL.md
dna_hash: sha256:5fe9bc23273f179b
language: markdown
size_loc: 67
generated: by-keidocs
---

# kei-cortex/INSTALL.md

## Public API

- `kei-cortex — Cortex daemon` — 
- `What it is` — Local HTTP daemon on 127.0.0.1:9797 that backs the cortex-ui Svelte app.
- `Install` — ./install.sh --profile=cortex
- `Env vars` — Required in `~/.claude/secrets/.env` (RULE 0.8 — never hardcode keys):
- `Host requirements` — Soft-checked by the installer when `kei-cortex` / `cortex-ui` are in scope:
- `Multi-tenant (Wave 25)` — Current: single bearer per daemon, one user per process. Wave 25 wires
- `See also` — - `_ts_packages/packages/cortex-ui/` — browser frontend

## Related

- parent: `kei-cortex`

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
