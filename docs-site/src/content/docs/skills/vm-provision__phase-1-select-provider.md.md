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

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
