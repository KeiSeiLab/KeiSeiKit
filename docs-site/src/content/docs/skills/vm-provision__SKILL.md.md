---
title: SKILL
path: vm-provision/SKILL.md
dna_hash: sha256:15aa0610c814f5df
language: markdown
size_loc: 136
generated: by-keidocs
---

# vm-provision/SKILL.md

## Public API

- `/vm-provision — 6-Phase VPS Pipeline (index)` — ---
- `When to use` — - Provisioning a new VPS from scratch: select provider → plan → provision → harden → verify → handoff.
- `Pipeline overview` — | Phase | File | Purpose | AskUserQuestion |
- `Hard-Gate Invariant (LOAD-BEARING)` — > **No application is deployed onto a VM that has not passed BOTH
- `Variables the pipeline produces` — | Name | Set in | Meaning |
- `Final report (emit after Phase 6)` — ```
- `Rules (enforced at every phase)` — - **Pure-click contract.** Only `INTENT` (argument) and custom port values
- `References` — - [phase-1-select-provider.md](phase-1-select-provider.md) · [phase-2-plan.md](phase-2-plan.md) · [phase-3-provision.md](phase-3-provision.md) · [phase-4-harden.md](phase-4-harden.md) · [phase-5-verify.md](phase-5-verify.md) · [phase-6-handoff.md](phase-6-handoff.md)

## Related

- parent: `vm-provision`

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
