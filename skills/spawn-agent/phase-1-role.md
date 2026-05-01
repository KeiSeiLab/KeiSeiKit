# Phase 1 — Role selection

> Goal: pick the agent role (capability tier). This single choice resolves
> `subagent_type` + `isolation` + Bash/Write permissions via `kei-spawn`.
> **Verify criterion:** `ROLE` set to one of the four fixed labels.

---

## 1.a — Single AskUserQuestion

Send ONE `AskUserQuestion` call. `multiSelect: false`. Do NOT fall through
to a default — the user must click.

```json
{
  "questions": [
    {
      "question": "Agent role / capability tier?",
      "header": "Role",
      "multiSelect": false,
      "options": [
        {
          "label": "read-only",
          "description": "Researcher. Read + Grep + Glob only. No Bash. No writes. Shared worktree. Use for: literature lookup, prior art, code exploration reports."
        },
        {
          "label": "explorer",
          "description": "Researcher + read-only Bash (ls, cat, rg, git log — no state changes). Shared worktree. Use for: audits, diagnostics, benchmark snapshots."
        },
        {
          "label": "edit-local",
          "description": "Code-implementer with worktree isolation. Can Write/Edit within a narrow whitelist (usually one module). Bash limited to cargo/test runners. Use for: single-crate changes, localized refactors."
        },
        {
          "label": "edit-shared",
          "description": "Code-implementer with worktree isolation + broader whitelist (multiple modules / cross-cutting). Bash limited to cargo/test. Use for: workspace-wide refactors, feature-slice work touching ≥2 crates."
        }
      ]
    }
  ]
}
```

Store the clicked label verbatim as `ROLE`.

---

## 1.b — Guidance (for the user, shown WITH the question)

Before sending the question, print one short paragraph:

> Pick the capability tier. Read-only and explorer never modify disk and
> are cheap to spawn. Edit-local and edit-shared get their own git worktree
> (per RULE 0.12) and inherit a Bash allowlist restricted to test runners.
> If unsure, pick `explorer` — it's the safest writeable-ish tier and
> covers most audit / diagnostic requests.

---

## 1.c — Verify criterion

`ROLE ∈ {read-only, explorer, edit-local, edit-shared}`. If the user
dismissed the question or picked something else, loop back. Do NOT proceed
to Phase 2 without ROLE set.

Emit a single-line confirmation: `Role locked: <ROLE>`. Proceed to Phase 2.

---

## 1.d — Failure paths (NO DOWNGRADE)

If the user wants a role outside the four fixed tiers (e.g. "I want a
writer with full Bash"):

- (A) Explain that the 4 tiers are the full capability grid in `kei-spawn`
  — anything else would need a new role added to the CLI. Offer to open
  a task for that.
- (B) Suggest the closest existing tier (usually `edit-shared` with a
  broader whitelist covers 90% of "I need more power" requests).
- (C) Abort this skill invocation and escalate to `/new-agent` if the user
  actually needs a new *agent manifest*, not a new *role tier*.

Never invent a fifth tier. `kei-spawn` will reject any ROLE not in the
fixed set.
