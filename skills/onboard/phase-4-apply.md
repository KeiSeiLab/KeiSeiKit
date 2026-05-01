# Phase 4 — Apply (branches on `MODE`)

Three mode-specific branches. Each ends with `APPLIED` and `SKIPPED` lists
ready for Phase 5.

## 4-pre — Multi-project bulk shortcut (AskUserQuestion, conditional)

Only emit if `len(PATHS) > 1` AND `GRANULARITY == "mixed"`. This lets the
user opt into a bulk shortcut mid-apply without rewinding to Phase 1.

```json
{
  "questions": [
    {
      "question": "Multi-project detected — apply same config to all matching projects?",
      "header": "Bulk shortcut",
      "multiSelect": false,
      "options": [
        {"label": "Yes — bulk same config",     "description": "Apply one agent/hook/primitive set to every project in PATHS. One decision covers all."},
        {"label": "No — per-project decisions",  "description": "Keep per-project mode (default): each project gets its own apply loop"},
        {"label": "Subset — pick projects",      "description": "Pick a subset of PATHS via free-text reply (comma-separated list of basename matches)"}
      ]
    }
  ]
}
```

Store as `BULK_MODE`. For single-project runs, skip this call.

## 4a — Full-auto branch (1 confirm total)

### Summary preview

Display a single table:

```
=== APPLY PREVIEW (full-auto) ===
Agents to create:    <N> (<name1>, <name2>, ...)
Hooks to apply:      <M> (<hook1>, <hook2>, ...)
Primitives to queue: <K> (<p1>, <p2>, ...)
Target projects:     <path1>, <path2>, ...
Estimated delegate calls: <N /new-agent + M /escalate-recurrence + K kei-sleep-queue add>
```

### Single confirm (AskUserQuestion)

```json
{
  "questions": [
    {
      "question": "Apply all proposed candidates in full-auto?",
      "header": "Confirm",
      "multiSelect": false,
      "options": [
        {"label": "Yes — apply everything",      "description": "Run all delegations with scan-derived defaults. No further prompts from onboard (downstream wizards may still prompt)."},
        {"label": "Downgrade to step-by-step",   "description": "Re-enter Phase 4 in step-by-step mode — per-candidate confirm"},
        {"label": "Abort",                       "description": "Stop — nothing is applied. Proposals remain in chat for your reference."}
      ]
    }
  ]
}
```

### Execution (on "Yes — apply everything")

For each agent candidate:

- Invoke Agent tool with `subagent_type: kei-code-implementer`, prompt:
  "Run the `new-agent` skill wizard non-interactively using these
  scan-derived fields: stack=<Q1>, deploy=<Q2>, paid-apis=<Q3>, ml=<Q4>,
  secrets=<Q5>, scrapers=<Q6>, slug=<slug>, description=<1-line>,
  path=<project-path>, gotchas=<3-5 lines from scan>. Proposed name:
  `kei-<slug>-specialist`."
- Record the resulting manifest path + generated `.md` path in `APPLIED`.

For each hook candidate that's CREATE:

- Invoke Agent tool with prompt: "Run the `escalate-recurrence` skill for
  pattern `<hook-name>`, severity `<suggested>`, event `<suggested>`,
  triggers `<scan evidence>`."
- Record the resulting hook path + rule path in `APPLIED`.

For each hook candidate that's "document/enable":

- Append a one-liner to the target project's CLAUDE.md (via
  `/new-agent` — the wizard handles the bridge). No separate apply step
  needed.

For each primitive candidate:

- Emit the install command. Two paths:
  - If primitive install is trivial (shell-only, no deps) → suggest
    `install.sh --add=<name>` in Phase 5 final report.
  - If primitive install is heavy (builds Rust, needs env vars) → invoke
    `kei-sleep-queue add` via Bash:
    ```bash
    ~/.claude/agents/_primitives/kei-sleep-queue.sh add \
       "install-primitive-<name>" \
       "install.sh --add=<name> from kit repo"
    ```
- Record the queue entry or suggested install command in `APPLIED`.

### Verify

- Every delegation returned a success signal (agent path exists on disk;
  escalate-recurrence reported write completion; sleep-queue returned a
  UUID).
- Failed delegations → move the candidate to `SKIPPED` with failure
  reason. Do NOT retry silently; report in Phase 5.

## 4b — Step-by-step branch (≥N confirms)

Iterate over `CANDIDATES` in order (agents first, then hooks, then
primitives). For each, emit:

```json
{
  "questions": [
    {
      "question": "<candidate-kind>: <name> — apply?",
      "header": "Candidate",
      "multiSelect": false,
      "options": [
        {"label": "Apply",          "description": "Run the delegation with scan-derived defaults (same as full-auto for this one)"},
        {"label": "Skip",           "description": "Record in SKIPPED list; move to next candidate"},
        {"label": "Modify (manual)", "description": "Drop into the full /new-agent or /escalate-recurrence wizard for this one candidate — you fill every field"}
      ]
    }
  ]
}
```

Branches:

- `Apply` → same execution as 4a for this candidate.
- `Skip` → append to `SKIPPED` with reason "user-declined-step-by-step".
- `Modify (manual)` → delegate to the appropriate wizard with scan-derived
  fields as SUGGESTIONS (user can override each). Agent candidates go to
  `/new-agent`; hooks/rules go to `/escalate-recurrence`.

### Verify

- Every candidate in `CANDIDATES` received exactly one click.
- `APPLIED + SKIPPED == CANDIDATES` (no silent drops).

## 4c — Full-manual branch (per-candidate wizard walk)

For EVERY candidate, fully delegate to the appropriate pipeline with
scan-derived defaults pre-populated but every wizard-phase click surfaced
to the user:

- **Agent** → invoke `/new-agent` (or Agent tool with `subagent_type:
  kei-code-implementer`) with prompt: "Run new-agent wizard INTERACTIVELY.
  Pre-fill from scan: slug=<slug>, path=<project-path>, description=<1-
  line>. User will click every phase. The 8-phase flow stands unchanged."
- **Hook/Rule** → invoke `/escalate-recurrence` with prompt: "Run
  escalate-recurrence INTERACTIVELY. Pre-fill pattern-name, severity
  suggestion, event suggestion from scan. User clicks every phase."
- **Primitive** → emit a one-line prompt: "Install `<name>` via
  `install.sh --add=<name>`? (yes / queue for sleep / skip)" — one
  AskUserQuestion per primitive:

```json
{
  "questions": [
    {
      "question": "Primitive: <name> — install mode?",
      "header": "Primitive",
      "multiSelect": false,
      "options": [
        {"label": "Install now",              "description": "Print install.sh --add=<name> command for user to run"},
        {"label": "Queue via kei-sleep-queue", "description": "Add to sleep queue — heavy installs run during a sleep session"},
        {"label": "Skip",                      "description": "No install; record as SKIPPED"}
      ]
    }
  ]
}
```

### Verify

- Every candidate walked its full wizard OR was explicitly skipped.
- Wizard-emitted AskUserQuestion calls are counted in the final AskUser
  total (they belong to the delegated skill, not to onboard, but onboard
  reports them for transparency).

## Verify-criterion (Phase 4 overall)

- Exactly one branch ran per project (4a / 4b / 4c).
- `APPLIED` and `SKIPPED` together cover every entry in `CANDIDATES`.
- At least one confirm-gate fired (full-auto has 1; step-by-step has ≥N;
  full-manual has ≥N wizard-internal calls).
- RULE 0.4: no fabricated delegation paths — if a wizard invocation fails,
  report the raw failure text, do not invent success.
