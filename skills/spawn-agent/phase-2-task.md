# Phase 2 — Task description (the only typed phase)

> Goal: capture a 1-3 sentence task description. This is the SOLE free-text
> input in the skill.
> **Verify criterion:** `TASK` non-empty, ≤2000 chars, not a single word.

---

## 2.a — Prompt

Print this exact message to the user (NOT an AskUserQuestion — regular chat):

> Describe the task in 1-3 sentences. Be concrete about the deliverable.
>
> Good: "Add a --verbose flag to kei-ledger list that prints the parent_branch and dna columns. Update the integration test accordingly."
>
> Weak: "Improve ledger output" (too vague, agent will ask clarifying questions and burn turns).
>
> Excellent tasks name: (1) the target file or module, (2) the verifiable
> deliverable, (3) any existing pattern to mirror.

Wait for the user's reply. Capture the full message as `TASK`.

---

## 2.b — Validation

Apply these checks in order. On any failure, re-print the guidance and
ask again. Do NOT fall through to Phase 3 on failure.

1. **Non-empty.** `len(TASK.trim()) > 0`.
2. **Minimum length.** `len(TASK.trim()) >= 20`. Shorter than 20 chars is
   almost certainly a single-word request — ask for more detail.
3. **Maximum length.** `len(TASK) <= 2000`. If longer, ask the user to
   trim: `kei-spawn` accepts up to 4000 chars but tasks >2000 typically
   indicate the user is trying to write the agent's plan for it. Push back.
4. **No secrets.** Quick regex scan for `sk-`, `ghp_`, `Bearer `, `AKIA`,
   `xoxb-`, `-----BEGIN`, `:AAG` (Telegram bot token infix). If any hits,
   STOP and ask the user to remove — tokens belong in `~/.claude/secrets/.env`
   per RULE 0.8, never in task descriptions.
5. **No git commands.** If `TASK` contains `git commit`, `git push`,
   `git add`, `git merge`, warn the user that per RULE 0.13 the spawned
   agent will be explicitly banned from git — if the task REQUIRES git
   operations, this skill is the wrong tool (use `/new-project` or
   orchestrator-meta flow instead).

---

## 2.c — Auto-augmentation

Before storing `TASK`, prepend a fixed preamble so the spawned agent sees
the orchestrator-branch-first rule verbatim. This is NOT optional — RULE
0.13 requires the ban-phrase in every non-trivial agent prompt:

```
You MUST NOT invoke git, bash state-changing commands, or shell commands
that mutate the repo. Tools allowed: Read, Write, Edit, Glob, Grep (plus
read-only Bash for explorer role; test-runners only for edit-* roles).
Write files to the paths inside the whitelist. Return a file-list block
in your final report. Orchestrator owns git.

--- TASK ---

<user's TASK here, verbatim>
```

Store the composed prompt as `TASK_FULL`. Keep the raw user text as
`TASK` for the report.

---

## 2.d — Verify criterion

Both `TASK` and `TASK_FULL` populated. `TASK` passes all 5 validation
checks. Emit a single-line confirmation:

`Task captured: <first 60 chars of TASK>... (<N> chars)`

Proceed to Phase 3.

---

## 2.e — Failure paths (NO DOWNGRADE)

If the user cannot articulate the task after two prompts:

- (A) Suggest invoking `/research` or `/debug-deep` first to clarify the
  problem, then return to `/spawn-agent` once the deliverable is concrete.
- (B) Offer to scaffold a skeleton task description from the user's rough
  words — show the skeleton, ask for approval.
- (C) Abort this invocation — better no agent than a confused one.
