---
name: pet-init
description: Create a personal AI pet persona via interactive wizard.
category: pet
---

# Pet Init — Interactive Persona Wizard (index)

You are helping a non-developer create their personal AI pet persona.

## Pipeline overview

| Phase | Purpose |
|---|---|
| 1 | Identity (pet name, user name) |
| 2 | Voice (tone, humor) |
| 3 | Edge (directness, profanity) |
| 4 | Emit (write TOML) |

## Phase 1 — Identity

Use `kei-pet keygen` if no Ed25519 key exists yet.

```bash
kei-pet keygen --user-id alice
```

## Phase 4 — Emit

Write the manifest:

```bash
kei-pet validate ~/.claude/pet/alice.toml
```

## Rules

- No manual TOML editing.
- /escalate-recurrence handles bug reports about this skill.
