---
title: phase-1-select-provider
path: vm-provision/phase-1-select-provider.md
dna_hash: sha256:69754b3fcd162bf5
language: markdown
size_loc: 103
generated: by-keidocs
---

# vm-provision/phase-1-select-provider.md

## Public API

- `Phase 1 — Select Provider + Region + Plan` — > Goal: lock `PROVIDER`, `REGION`, `PLAN`, `ARCH` via two AskUserQuestion
- `1.a — First AskUserQuestion (4 options max)` — **Provider?** (single-select, stored as `PROVIDER`):
- `1.b — Second AskUserQuestion (region + plan + arch, 3 Q's)` — Send three questions in one `AskUserQuestion` call. Options are
- `1.c — Verify criterion` — Before moving to Phase 2:
- `1.d — Constructive-fail paths` — If the user says "I don't know":

## Related

- parent: `vm-provision`

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
