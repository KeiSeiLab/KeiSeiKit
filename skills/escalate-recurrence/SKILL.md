---
name: escalate-recurrence
description: Triggered when Claude detects the same mistake ≥2× in one session, or the user says "опять / again / second time / already said". Interactively codifies the pattern into a rule + hook + wiki entry via option-picker questions. Pure-click flow — no free-text decisions. Writes rule file, generates hook scaffold at user-chosen severity, registers hook in settings.json via update-config. See ~/.claude/rules/recurrence-escalate.md.
---

# Escalate Recurrence — Interactive Codifier

You are converting an observed repeating mistake into a permanent guardrail: a rule file, a wiki entry, and (optionally) an enforcement hook. The entire decision chain is AskUserQuestion — the user clicks, you generate.

---

## Phase 0 — Sanity check (do this BEFORE asking anything)

1. State in ONE paragraph what you observed:
   - The mistake pattern (abstract, not specific instance)
   - Two or more concrete instances with file:line or tool-call evidence
   - Your root-cause hypothesis
2. Grep existing rules for coverage:
   ```bash
   grep -rinE "<3-5 distinctive keywords from the pattern>" ~/.claude/rules/ 2>/dev/null | head -10
   ```
3. If an existing rule ALREADY covers the pattern → offer `(A) extend that rule / (B) new sibling rule / (C) cancel` via AskUserQuestion and branch. Do NOT duplicate rules.

---

## Phase 1 — Pure-click decisions (AskUserQuestion, SINGLE call)

Emit all four questions in one `AskUserQuestion` invocation. Use `multiSelect: false`.

```json
{
  "questions": [
    {
      "question": "Codify this pattern as a rule?",
      "header": "Rule",
      "multiSelect": false,
      "options": [
        {"label": "Yes — write rule file",   "description": "New file at ~/.claude/rules/<topic>.md"},
        {"label": "Extend existing rule",    "description": "Append section to the closest matching rule"},
        {"label": "No — session-only",       "description": "Note in chat, don't persist"},
        {"label": "Later — remind me",       "description": "Skip now, flag in recurrence-log for next session"}
      ]
    },
    {
      "question": "Wikify (add to indices)?",
      "header": "Wiki",
      "multiSelect": false,
      "options": [
        {"label": "Yes — full (RULES.md + MEMORY.md + CLAUDE.md Rules Index)", "description": "Discoverable from all three entry points"},
        {"label": "Partial — RULES.md only", "description": "Registry entry only, no MEMORY.md pointer"},
        {"label": "No",                       "description": "Rule file exists but not indexed"}
      ]
    },
    {
      "question": "Create enforcement hook?",
      "header": "Hook severity",
      "multiSelect": false,
      "options": [
        {"label": "block (exit 2)",               "description": "Hard deny. Reserve for data-loss / secret / IP patterns."},
        {"label": "enforce (exit 1)",             "description": "Error — Claude must fix before retry"},
        {"label": "warn (exit 0 + stderr)",       "description": "Advisory; tool call proceeds. Safest first step."},
        {"label": "remind (UserPromptSubmit)",    "description": "Passive reminder on every user turn"},
        {"label": "No hook",                      "description": "Rule only, no automation"}
      ]
    },
    {
      "question": "Hook event type?",
      "header": "Hook event",
      "multiSelect": false,
      "options": [
        {"label": "PreToolUse:Bash",           "description": "Gate on shell commands — cmd-string pattern match"},
        {"label": "PreToolUse:Edit|Write",     "description": "Gate on file modifications — path / content pattern match"},
        {"label": "PostToolUse:*",             "description": "React AFTER a tool call (e.g. auto-rebuild, auto-log)"},
        {"label": "UserPromptSubmit",          "description": "Runs on every user turn — for remind-level hooks"},
        {"label": "Stop",                      "description": "Runs when Claude finishes — for end-of-turn reminders"},
        {"label": "N/A — no hook chosen",      "description": "If previous answer was 'No hook'"}
      ]
    }
  ]
}
```

Store as `R`, `W`, `S`, `E` (rule / wiki / severity / event).

If `S == "No hook"` and `E != "N/A — no hook chosen"` → re-ask just the event question with a reminder. Never silently fix user inconsistency.

---

## Phase 2 — Generate artefacts (in memory — do NOT Write yet)

Compose all files the user will see in the diff preview:

### 2.1 — Rule file (if `R` is Yes or Extend)

Path: `~/.claude/rules/<slug>.md` (slug = kebab-case short pattern name, 2–4 words).

Template:
```markdown
# RULE — <Human Pattern Name> (YYYY-MM-DD ADDED)

> <one-sentence pattern description>.

## Incident

<The recurrence evidence: 2+ concrete instances with file:line or tool-call refs.>

## The Rule

<Actionable rule, imperative voice.>

## Triggers

- <Specific detectable phrase / pattern 1>
- <Specific detectable phrase / pattern 2>

## Enforcement

- Hook: `~/.claude/hooks/<slug>-guard.sh` (<event>, severity <S>)
- Bypass: <bypass env var name, if applicable>

## Why this and not "remember to check"

<One sentence on why a hook beats memory for this pattern.>

## Rule lock

YYYY-MM-DD. Never override without explicit user revocation.
```

Fill every `<...>` placeholder from Phase 0 + Phase 1 answers. Literal `{{placeholder}}` is forbidden — the assembler `validator.rs` rejects it.

### 2.2 — Hook scaffold (if `S != "No hook"`)

Path: `~/.claude/hooks/<slug>-guard.sh`.

**Template per event type:**

`PreToolUse:Bash`:
```bash
#!/bin/sh
# <slug>-guard — <one-line purpose>
# Severity: <S>; Event: PreToolUse:Bash
# Rule: ~/.claude/rules/<slug>.md

command -v jq >/dev/null 2>&1 || exit 0
set -eu

CMD=$(jq -r '.tool_input.command // empty')
[ -n "$CMD" ] || exit 0

# Trigger pattern — tighten during dogfooding
case "$CMD" in
  *<pattern-literal>*)
    echo "[<slug>] <reminder text>. See ~/.claude/rules/<slug>.md" >&2
    exit <exit-code-from-severity>
    ;;
esac
exit 0
```

`PreToolUse:Edit|Write`:
```bash
#!/bin/sh
command -v jq >/dev/null 2>&1 || exit 0
set -eu

FILE=$(jq -r '.tool_input.file_path // empty')
CONTENT=$(jq -r '.tool_input.content // .tool_input.new_string // empty')
[ -n "$FILE" ] || exit 0

case "$FILE" in
  *<path-pattern>*)
    printf '%s' "$CONTENT" | grep -qE '<content-pattern>' || exit 0
    echo "[<slug>] <reminder>" >&2
    exit <exit-code>
    ;;
esac
exit 0
```

`UserPromptSubmit`:
```bash
#!/bin/sh
command -v jq >/dev/null 2>&1 || exit 0
PROMPT=$(jq -r '.prompt // empty')
case "$PROMPT" in
  *<trigger-phrase>*)
    echo "[<slug>] reminder: <text>. See ~/.claude/rules/<slug>.md" >&2
    ;;
esac
exit 0
```

**Exit code → severity map:**
- block → `exit 2`
- enforce → `exit 1`
- warn → `exit 0` (but still echo to stderr)
- remind → `exit 0` (UserPromptSubmit only — echo to stderr)

Replace every `<...>` with the pattern-specific literal extracted from the Phase 0 evidence. Narrow match preferred over broad match — false-positive fatigue kills a hook faster than any bug.

### 2.3 — Wikification diffs (if `W != No`)

Compute the diff lines to add:

**`~/.claude/rules/RULES.md`** (append to table body, keep alphabetical order if easy; otherwise end):
```
| <slug>.md | <one-line coverage> | Rules Index (added YYYY-MM-DD) | ~/.claude/hooks/<slug>-guard.sh |
```

**`~/.claude/projects/-Users-denisparfionovich/memory/MEMORY.md`** (one-liner under `## Rules & Feedback`):
```
- [[../../../../rules/<slug>]] — <one-line coverage>
```

**`~/.claude/CLAUDE.md` Rules Index table** (if `W == "Yes — full"`):
```
| <short topic> | [[<slug>]] | <one-line covers> |
```

### 2.4 — Settings registration (if `S != "No hook"`)

Compute the JSON hook entry:
```json
{
  "matcher": "<event matcher>",
  "hooks": [
    {
      "type": "command",
      "command": "~/.claude/hooks/<slug>-guard.sh",
      "statusMessage": "<slug> <severity> check..."
    }
  ]
}
```

This gets merged into `~/.claude/settings.json` under `hooks.PreToolUse` / `hooks.UserPromptSubmit` / etc.

---

## Phase 3 — Confirm via diff preview (AskUserQuestion — keep it click-only)

Show all generated artefacts inline in ONE message, THEN emit a single `AskUserQuestion`:

```
=== GENERATED — REVIEW BEFORE WRITE ===

1. New rule: ~/.claude/rules/<slug>.md
<full content, 30–80 lines>

2. New hook: ~/.claude/hooks/<slug>-guard.sh   (severity: <S>, event: <E>)
<full content, 15–30 lines>

3. RULES.md append:
<one row>

4. MEMORY.md append:
<one line>

5. settings.json merge (PreToolUse / UserPromptSubmit / etc):
<json block>
```

Then:

```json
{
  "questions": [
    {
      "question": "Write these artefacts + register hook?",
      "header": "Confirm",
      "multiSelect": false,
      "options": [
        {"label": "Confirm — write all",   "description": "Phase 4 runs: files written, hook registered via update-config"},
        {"label": "Edit rule (1)",         "description": "Regenerate the rule file with changes you specify"},
        {"label": "Edit hook (2)",         "description": "Regenerate the hook scaffold with changes you specify"},
        {"label": "Edit wiki diffs (3-4)", "description": "Regenerate RULES.md / MEMORY.md entries"},
        {"label": "Edit settings (5)",     "description": "Regenerate the settings.json merge block"},
        {"label": "Abort",                 "description": "Stop — nothing gets written, no files touched"}
      ]
    }
  ]
}
```

On `Confirm` → Phase 4. On any `Edit <N>` → ask ONE free-text line for what to change in that artefact (unavoidable — arbitrary content edit), regenerate, re-preview. On `Abort` → stop, write nothing, do NOT touch `~/.claude/`.

---

## Phase 4 — Write + register (on confirm only)

Execute in order (each via its right tool — do NOT shell out when a tool exists):

1. `Write` → `~/.claude/rules/<slug>.md`
2. If hook: `Write` → `~/.claude/hooks/<slug>-guard.sh`, then `Bash` → `chmod +x ~/.claude/hooks/<slug>-guard.sh`
3. `Edit` → `~/.claude/rules/RULES.md` (append the row)
4. `Edit` → `~/.claude/projects/-Users-denisparfionovich/memory/MEMORY.md` (append the line)
5. `Edit` → `~/.claude/CLAUDE.md` (add Rules Index row) — only if `W == "Yes — full"`
6. If hook: invoke the `update-config` skill with the settings-merge spec. Do NOT directly hand-edit `settings.json` — the skill knows how to merge without clobbering foreign entries.
7. `Edit` → append one line to `~/.claude/memory/recurrence-log.md` (create if absent):
   ```
   YYYY-MM-DD | <slug> | severity=<S> | event=<E> | pattern="<one-line>" | instances=<N>
   ```

---

## Phase 5 — Report

```
✓ Rule codified:      ~/.claude/rules/<slug>.md
✓ Hook registered:    ~/.claude/hooks/<slug>-guard.sh   (severity <S>, event <E>)
✓ Wikified:           RULES.md, MEMORY.md<, CLAUDE.md if full>
✓ Log entry:          ~/.claude/memory/recurrence-log.md

Next time this pattern triggers, the hook <blocks | errors | warns | reminds>.
To upgrade severity: invoke /escalate-recurrence again on the same pattern.
To disable: remove the entry from settings.json (or set "enabled": false).
```

---

## Rules (apply throughout)

- **Pure click wherever possible.** Only slug and pattern-literal require typing. Everything structural is AskUserQuestion.
- **Narrow match preferred.** A hook with false-positives gets ignored, then removed. Start narrow, widen on next recurrence.
- **Start at lowest severity that solves the pain.** Upgrade on re-trigger, never auto-promote.
- **Never write `{{placeholder}}` literals.** The assembler `validator.rs` rejects them; same hygiene applies here.
- **No Patching.** A new rule codifies a pattern, it doesn't paper over an existing rule's gap. If the gap is in an existing rule → extend that rule (Phase 1 option "Extend existing").
- **Escape hatch.** Every hook MUST document its bypass mechanism (env var like `<SLUG>_BYPASS=1` or marker string). Without escape, emergency work stalls.
