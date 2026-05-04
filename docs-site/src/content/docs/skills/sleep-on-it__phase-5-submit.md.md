---
title: phase-5-submit
path: sleep-on-it/phase-5-submit.md
dna_hash: sha256:15bb724eaa622623
language: markdown
size_loc: 114
generated: by-keidocs
---

# sleep-on-it/phase-5-submit.md

## Public API

- `Phase 5 — Preview and submit (click)` — Show the user exactly what will be written, then submit via the helper.
- `5a — Render preview` — Print a fenced block with the frontmatter + body preview:
- `5b — Click` — Emit ONE `AskUserQuestion`:
- `5c — Dispatch` — - `SUBMIT_ACTION == "Edit"` → restart from Phase 1 (clears all variables).
- `5d — Invoke `kei-sleep-queue.sh add`` — Write the task text to a temp file, then:
- `Verify-criterion` — - `SUBMIT_ACTION ∈ {Submit, Edit, Abort}`.

## Related

- parent: `sleep-on-it`

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
