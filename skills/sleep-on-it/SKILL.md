---
name: sleep-on-it
description: Defer a hard question, research task, or design comparison to the nightly remote agent (KeiSeiKit v0.12.0 incubation layer). Runs on top of the v0.11 sleep-sync pipeline — user fills one free-text field plus three clicks, task lands in sync-repo/sleep-queue/ and is processed before REM consolidation. Up to 5 tasks per night, 15 minutes each. Pure-click wizard except the single task-description field.
argument-hint: (no arguments)
---

# Sleep On It — Incubation Wizard (index)

## When to use

- Deferring a hard question, research task, or design comparison to the nightly cloud agent for overnight processing.
- Queuing up to 5 tasks per night (15 min each) to be processed before the REM consolidation pass.
- Requires v0.11 sleep-sync already configured; use `/sleep-setup` first if not.

Biological analog: the "sleep on it" insight effect documented by
Wagner et al. 2004, *Nature* 427:352–355
[VERIFIED: doi:10.1038/nature02223] (the original paper showed insight
gain after a sleep period that included slow-wave sleep, not strictly
REM; our metaphor is a loose mapping). During the day the user
submits open questions, research tasks, or design comparisons via this
wizard; the nightly cloud agent processes the queue before its existing
REM consolidation pass and writes results to `sync-repo/sleep-results/`.

This `SKILL.md` is the INDEX. Each phase lives in its own file and is
executed in order. Never skip a phase. Never re-order phases.

---

## Prerequisites (hard fail fast if missing)

- v0.11 sleep-sync must be configured (`~/.claude/secrets/.env` contains
  `KEI_MEMORY_REPO_PATH`, `KEI_MEMORY_SSH_KEY`, and the sync-repo exists
  under that path with a `.git/` subdir).
- `_primitives/kei-sleep-queue.sh` exists at
  `~/.claude/agents/_primitives/kei-sleep-queue.sh` and is executable.

If either is missing, print the single line

```
v0.11 sleep-sync not configured — run `/sleep-setup` first, then retry.
```

and exit the wizard. Do not attempt to queue anything offline.

---

## Pipeline overview (6 phases, 5+ AskUserQuestion)

| Phase | File | Purpose | AskUserQuestion |
|---|---|---|---|
| 1 | [phase-1-intake.md](phase-1-intake.md) | One free-text field: the question / task | 0 (prompt, non-empty validate) |
| 2 | [phase-2-type.md](phase-2-type.md) | Task type: deep / pipeline / pattern / compare / custom | 1 (click) |
| 3 | [phase-3-priority.md](phase-3-priority.md) | Priority: tonight / FIFO / weekly | 1 (click) |
| 4 | [phase-4-format.md](phase-4-format.md) | Output format: markdown / ADR / checklist / table | 1 (click) |
| 5 | [phase-5-submit.md](phase-5-submit.md) | Preview frontmatter + body, submit / edit / abort | 1 (click) |
| 6 | [phase-6-ack.md](phase-6-ack.md) | Acknowledgment with UUID + queue path + run ETA | 1 (click) |

**Minimum AskUserQuestion count: 5.** All clicks except the single
free-text task description in Phase 1.

---

## Variables the pipeline produces

| Name | Set in | Meaning |
|---|---|---|
| `TASK_TEXT` | Phase 1 | Free-text task description (non-empty) |
| `TASK_TYPE` | Phase 2 | `deep` / `pipeline` / `pattern` / `compare` / `custom` |
| `PRIORITY` | Phase 3 | `night` / `fifo` / `weekly` |
| `FORMAT` | Phase 4 | `md` / `adr` / `checklist` / `table` |
| `SUBMIT_ACTION` | Phase 5 | `submit` / `edit` / `abort` |
| `QUEUE_PATH` | Phase 5 | Path of the queue file written by `kei-sleep-queue.sh add` |
| `UUID` | Phase 5 | UUID assigned by the helper |

---

## Final report (emit after Phase 6)

```
=== SLEEP-ON-IT REPORT ===
UUID:           <UUID>
Queue file:     <QUEUE_PATH>
Task type:      <TASK_TYPE>
Priority:       <PRIORITY>
Output format:  <FORMAT>
Next run ETA:   <UTC cron time from .keisei-sync.toml>
Results land:   sync-repo/sleep-results/<UUID>.md
```

---

## Rules (apply throughout — enforced at every phase)

- **Pure-click contract.** Only Phase 1 asks for free text; every other
  decision is an `AskUserQuestion`. No `freeText` outside Phase 1.
- **Idempotent.** Re-running the wizard while a previous task is still
  pending is fine — each submission gets its own UUID and its own queue
  file. No "one pending at a time" constraint.
- **NO DOWNGRADE (RULE -1).** If the helper rejects (invalid flag, sync
  push fails), surface 2-3 constructive fix paths — never
  "cannot submit".
- **NO HALLUCINATION (RULE 0.4).** Never fabricate a UUID, queue path,
  or ETA — always echo the real helper output.
- **RULE 0.8 secrets.** Queue files never embed tokens; env refs live in
  `~/.claude/secrets/.env` only.
- **Silent failure (RULE 0.15).** If the post-submit sync push fails,
  the queue file still lives locally and will be pushed on the next
  session-end dump. The wizard must NOT block on push failure.
- **Constructor Pattern (RULE ZERO).** Every phase file < 100 LOC.

---

## References

- `~/.claude/rules/sleep-layer.md` — RULE 0.15 full text (Phase A added v0.12.0)
- `_primitives/kei-sleep-queue.sh` — the queue CRUD helper
- `_primitives/kei-sleep-sync.sh` — the session-end-dump callback (also
  invoked by `kei-sleep-queue.sh add` after write)
- `_primitives/templates/sleep-incubation-prompt.md` — cloud agent Phase A
- `_primitives/templates/sleep-trigger-prompt.md` — cloud agent Phase B
- `skills/sleep-setup/` — v0.11 one-time sync-repo wizard (prerequisite)
