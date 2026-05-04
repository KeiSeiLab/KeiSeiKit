---
title: phase-2-task
path: spawn-agent/phase-2-task.md
dna_hash: sha256:150ec70c5f74d447
language: markdown
size_loc: 91
generated: by-keidocs
---

# spawn-agent/phase-2-task.md

## Public API

- `Phase 2 — Task description (the only typed phase)` — > Goal: capture a 1-3 sentence task description. This is the SOLE free-text
- `2.a — Prompt` — Print this exact message to the user (NOT an AskUserQuestion — regular chat):
- `2.b — Validation` — Apply these checks in order. On any failure, re-print the guidance and
- `2.c — Auto-augmentation` — Before storing `TASK`, prepend a fixed preamble so the spawned agent sees
- `2.d — Verify criterion` — Both `TASK` and `TASK_FULL` populated. `TASK` passes all 5 validation
- `2.e — Failure paths (NO DOWNGRADE)` — If the user cannot articulate the task after two prompts:

## Related

- parent: `spawn-agent`

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
