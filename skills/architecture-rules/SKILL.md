---
name: architecture-rules
description: Architectural rules engine — pattern validation, duplication detection, stack compatibility, and design pattern enforcement. Auto-triggers during code review, architecture planning, and when creating new modules. Triggers on "проверь архитектуру", "review architecture", "check patterns", "проверь дублирование", "stack compatibility".
user-invocable: false
---

# Architecture Rules Engine

This skill provides the rules database for architectural validation.
It is loaded by other skills (architecture, debug-deep, code-review) when they need to validate decisions.

## How to Use

When analyzing code or planning architecture, load and apply rules from:
- `~/.claude/skills/architecture-rules/references/patterns.md` — design patterns catalog
- `~/.claude/skills/architecture-rules/references/antipatterns.md` — what to avoid
- `~/.claude/skills/architecture-rules/references/stack-compat.md` — technology compatibility matrix
- `~/.claude/skills/architecture-rules/references/duplication.md` — duplication detection rules
- `~/.claude/skills/architecture-rules/references/checklist.md` — pre-implementation checklist

For each architectural decision, validate against these references and report violations.
