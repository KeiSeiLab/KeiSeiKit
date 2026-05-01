# Phase 7 — Recipe assembly (branches on `T`)

Before branching, resolve auto-detect if `T == "Auto-detect"`.

## 7a — Resolve auto-detect (conditional)

Infer target type from architecture (Phase 5):

- Expression mentions a project's CLAUDE.md + stack block + deploy block →
  **Agent**.
- Expression is a 3-phase flow with AskUserQuestion calls → **Skill**.
- Expression is a trigger + enforcement pair, pattern-matched on tool input
  → **Hook** (and, usually, companion **Rule**).
- Expression is documentation + wiki indexing, no automation → **Rule**.
- Expression is a single reusable 20-40 LOC markdown — already handled in
  Phase 6 → **Block**.

Present the inferred type:

```json
{
  "questions": [
    {
      "question": "Detected target: <X>. Proceed?",
      "header": "Auto-detect",
      "multiSelect": false,
      "options": [
        {"label": "Yes — proceed with <X>",                "description": "Run the <X> branch below"},
        {"label": "Change to Agent",                        "description": "Override the inference — go to 7b"},
        {"label": "Change to Skill",                        "description": "Override — go to 7c"},
        {"label": "Change to Hook",                         "description": "Override — go to 7d"},
        {"label": "Change to Rule",                         "description": "Override — go to 7e"},
        {"label": "Block only (no assembly)",               "description": "Already handled in Phase 6 — skip to final report"}
      ]
    }
  ]
}
```

Substitute `<X>` with the literal inferred label.

## 7b — Agent branch

Hand off to the `new-agent` skill — it already codifies the 8-phase wizard
(`skills/new-agent/SKILL.md`). Two handoff methods:

1. **Invoke via Agent tool** with `subagent_type: kei-code-implementer` (or
   equivalent), prompt: "Run the `new-agent` skill wizard. Use these
   already-decided fields from compose-solution: stack, deploy, paid-APIs,
   ML, secrets, scrapers. Slug, description, path, gotchas are
   derived from DESC. Blocks list preference (from Phase 5 architecture):
   <list>."
2. **Instruct user** to run `/new-agent` in a fresh turn if Agent
   delegation is unavailable; paste the Phase-5 architecture into that
   session.

Compose-solution steps back here — `new-agent` owns the rest.

## 7c — Skill branch

Compose a new `skills/<slug>/SKILL.md` inline:

```markdown
---
name: <slug>
description: <one-line derived from DESC>
argument-hint: <if the skill takes a target, e.g. "<project or path>">
---

# <Human Name> — <one-line>

<2-3 sentences: what the skill does, when to invoke, who owns the output.>

## Phase 1 — Intake (<AskUserQuestion | free-text>)

<Derived from architecture Phase 5. Escalate-recurrence style: if the
decision space is discrete, use AskUserQuestion; otherwise one free-text
line, strictly validated.>

## Phase 2 — <Action>

<Steps derived from Phase 5 expression. Verify-criterion per step.>

## Phase 3 — Report

<What the user sees at the end. Concise report block.>

## Rules

<Borrow from _blocks/baseline.md if generic enforcement needed.>
```

Minimum three phases (intake / action / report). AskUserQuestion pattern
follows `escalate-recurrence` (see
`~/.claude/skills/escalate-recurrence/SKILL.md` globally, or
`skills/new-agent/SKILL.md` Phase-1b style locally).

Preview + final confirm:

```json
{
  "questions": [
    {
      "question": "Write this skill?",
      "header": "Skill",
      "multiSelect": false,
      "options": [
        {"label": "Write to skills/<slug>/SKILL.md", "description": "Save permanently; user can invoke as /<slug>"},
        {"label": "Edit (free-text)",                "description": "Reply with one message describing changes"},
        {"label": "Abort",                           "description": "Stop — nothing gets written"}
      ]
    }
  ]
}
```

On `Write` → `mkdir -p skills/<slug>/ && Write skills/<slug>/SKILL.md`.

## 7d — Hook branch

Delegate to the `escalate-recurrence` skill
(`~/.claude/skills/escalate-recurrence/SKILL.md`). That skill already owns
hook scaffolding at 4 severities (block / enforce / warn / remind) + 5
event types (PreToolUse:Bash, PreToolUse:Edit|Write, PostToolUse:*,
UserPromptSubmit, Stop) and registers via the `update-config` skill.

Instruct the user:

> Run `/escalate-recurrence` in a fresh turn. Use these already-decided
> fields from compose-solution:
> - Pattern name: `<slug>`
> - Two+ concrete trigger instances: <from DESC and Phase-5 architecture>
> - Suggested severity: <warn | enforce | block | remind> — based on
>   <one-line justification from DESC>
> - Suggested event: <PreToolUse:Bash | Edit|Write | UserPromptSubmit | ...>

Or invoke via Agent tool if delegation is permitted.

Compose-solution steps back — `escalate-recurrence` owns writes.

## 7e — Rule branch

Same handoff as 7d — `escalate-recurrence` owns the rule + wiki pipeline
(it writes `~/.claude/rules/<slug>.md`, updates `RULES.md`, `MEMORY.md`, and
optionally `CLAUDE.md` Rules Index). Instruct the user to run
`/escalate-recurrence` with Phase-1 choice "No hook" if the user wants
documentation-only.

## 7f — Block only

Already handled in Phase 6. Skip to final report.

## Verify-criterion

- Exactly one branch ran (7b / 7c / 7d / 7e / 7f).
- The resulting artefact path is captured for the final report.
