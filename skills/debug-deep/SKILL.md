---
name: debug-deep
description: Deep debugging using multi-agent analysis and error pattern matching. Use when user reports a bug, error, crash, or unexpected behavior. Triggers on "debug", "fix", "broken", "crash", "error", "bug".
argument-hint: <error description or paste>
---

# Deep Debug — Holographic Error Analysis

Error: $ARGUMENTS

> [OPTIONAL INTEGRATIONS: this skill can cross-reference a project-local
> error-patterns log and an `architecture-rules` skill if either is installed.
> Both are optional — the skill works without them.]

## Phase 0: Check Error Patterns (optional)

If the project maintains an error-patterns log (conventional paths:
`$PROJECT_ROOT/error-patterns.json` or `$HOME/.claude/memory/error-patterns.json`),
read it FIRST.

Check for matching patterns with frequency="recurring" or severity="critical".
If match found — apply known fix immediately with note "Known pattern: [id]".

If no log exists, skip this phase.

## Phase 0.5: Load Architecture Rules (optional)

If an `architecture-rules` skill is installed at
`$HOME/.claude/skills/architecture-rules/`, read its
`references/antipatterns.md` and `references/stack-compat.md` — the bug may
be caused by a known anti-pattern or stack incompatibility.

If that skill is not present, skip this phase.

## Phase 1: Parallel Diagnosis (3 agents)

### Agent 1: Stack Trace Analyzer
- Parse the error message/stack trace
- Identify the exact file:line where error originates
- Trace the call chain backwards
- Read all relevant source files
- Return: root cause hypothesis + files involved

### Agent 2: Context Investigator
- Check git history — what changed recently?
- Check if similar errors in git log messages
- Look at related tests — are they passing?
- Check dependencies — version mismatches?
- Return: timeline of changes + correlation

### Agent 3: Pattern Matcher
- Search codebase for similar patterns that work
- Compare working vs broken code
- Check if it's a known anti-pattern
- Search web for this specific error message
- Return: similar working patterns + external solutions

## Phase 2: Root Cause Analysis

After agents return:
1. Cross-reference all 3 analyses
2. Identify ROOT cause (not symptom!)
3. Rate confidence: X%

```
## Root Cause Analysis

### Error Chain:
[symptom] ← [intermediate cause] ← [ROOT CAUSE]

### Root Cause: [description]
Confidence: X%

### Evidence:
- Agent 1 found: [...]
- Agent 2 found: [...]
- Agent 3 found: [...]

### Why previous fixes failed (if applicable):
[They fixed symptoms, not root cause]
```

## Phase 3: Fix Options

Present 2-3 fix approaches:

### Fix A: Architectural (correct)
Changes the structure to prevent the class of error

### Fix B: Targeted (quick)
Minimal change that fixes this specific instance

### Recommendation: [A or B] with reasoning

## Phase 4: Implement & Verify

1. Apply chosen fix
2. Run relevant tests
3. Verify fix doesn't break other things
4. Check for similar vulnerable patterns elsewhere

## Phase 5: Log Error Pattern (optional)

If the project maintains an error-patterns log, append this fix to it.

Conventional path (create if it does not exist):
`$PROJECT_ROOT/error-patterns.json` or `$HOME/.claude/memory/error-patterns.json`

Entry format:
```json
{
  "id": "[category]-[number]",
  "name": "[short pattern name]",
  "trigger": "[when this happens]",
  "wrongApproach": "[what was tried incorrectly]",
  "correctApproach": "[actual fix]",
  "severity": "critical|high|medium",
  "frequency": "recurring|very-common|common|occasional",
  "occurrences": 1,
  "lastSeen": "[today's date]"
}
```

If you maintain a project development-learnings log, append a session entry
there as well.

## Rules
- NEVER patch symptoms — find root cause
- Architecture fix > external fix
- Don't rebuild what works
- Verify every fact — no hallucinations
- Log the error pattern when you maintain such a log
