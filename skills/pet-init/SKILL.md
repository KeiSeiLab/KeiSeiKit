---
name: pet-init
description: Create a personal AI pet persona via interactive wizard. No TOML editing required.
category: pet
---

# Pet Init — Interactive Persona Wizard (index)

You are helping a non-developer create their personal AI pet persona. The
output is a valid `pet.toml` manifest conforming to the `kei-pet` schema
(see `_primitives/_rust/kei-pet/examples/minimal.toml`). The user NEVER
edits TOML by hand — every field is gathered through `AskUserQuestion`
batches or short free-text prompts.

This `SKILL.md` is the INDEX. Each phase lives in its own file and runs in
strict order. Never skip or re-order phases.

---

## Pipeline overview (4 phases)

| Phase | File | Purpose | AskUserQuestion |
|---|---|---|---|
| 1 | [phase-1-identity.md](phase-1-identity.md) | Pet name, user name, addressing style, languages | 1 batch (2 questions) |
| 2 | [phase-2-voice.md](phase-2-voice.md) | Tone, humor style, humor frequency | 1 batch (4 questions) |
| 3 | [phase-3-edge.md](phase-3-edge.md) | Directness, initiative, profanity, forbidden topics | 1 batch (3 questions) + 1 free-text |
| 4 | [phase-4-emit.md](phase-4-emit.md) | Compose TOML, keygen if needed, write file, summary | 0 |

Exit: `~/.claude/pet/<user_id>.toml` written, summary displayed, next-step
suggestion shown.

---

## Variables the pipeline produces

| Name | Set in | Shape |
|---|---|---|
| `PET_NAME`          | Phase 1 | string, 1-30 chars |
| `USER_NAME`         | Phase 1 | string |
| `ADDRESSING`        | Phase 1 | `by-name` / `formal` / `casual` |
| `LANGUAGES`         | Phase 1 | array of ISO codes |
| `TONE_PRIMARY`      | Phase 2 | `warm` / `neutral` / `formal` / `playful` |
| `TONE_SECONDARY`    | Phase 2 | array, 0-2 entries |
| `HUMOR_STYLE`       | Phase 2 | `none` / `dry` / `witty` / `silly` |
| `HUMOR_FREQUENCY`   | Phase 2 | `rare` / `occasional` / `frequent` |
| `DIRECTNESS`        | Phase 3 | `gentle` / `balanced` / `direct` / `blunt` |
| `INITIATIVE`        | Phase 3 | `wait` / `nudge` / `proactive` |
| `PROFANITY`         | Phase 3 | `never` / `rare` / `contextual` |
| `FORBIDDEN_TOPICS`  | Phase 3 | array of strings, may be empty |
| `USER_ID`           | Phase 4 | Ed25519 short id (from `kei-pet keygen`) |

---

## Rules (apply throughout)

- **No manual TOML.** The user never sees or edits raw TOML until after
  Phase 4 emits the file. Any correction = re-run `/pet-init`.
- **RULE 0.4 (NO HALLUCINATION).** Never invent defaults silently. Every
  field is either asked or explicitly defaulted in the phase file.
- **RULE 0.8 (SECRETS).** The Ed25519 secret key (created by
  `kei-pet keygen`) is written by the primitive into its own keystore —
  this skill never reads or displays secret-key material. Only the
  public `user_id` short-hash is surfaced to the user.
- **NO DOWNGRADE.** If Phase 4 cannot write the file (permission error,
  disk full, keygen failure), return 2-3 constructive paths — never
  "can't be done".
- **Constructor Pattern.** Each phase file is a single cube ≤200 LOC.
  This index stays ≤200 LOC.
- **Surgical Changes.** The only file written by this skill is
  `~/.claude/pet/<user_id>.toml`. No other artefacts.

---

## Exit report (emit after Phase 4)

```
=== PET-INIT REPORT ===
Pet name:    <PET_NAME>
User:        <USER_NAME>
File:        ~/.claude/pet/<USER_ID>.toml
Size:        <bytes>
Keygen:      <reused existing | newly created>
Next:        /pet-chat  or  kei-pet render --pet ~/.claude/pet/<USER_ID>.toml
```

---

## References

- [phase-1-identity.md](phase-1-identity.md)
- [phase-2-voice.md](phase-2-voice.md)
- [phase-3-edge.md](phase-3-edge.md)
- [phase-4-emit.md](phase-4-emit.md)
- `_primitives/_rust/kei-pet/examples/minimal.toml` — schema reference
- `_primitives/_rust/kei-pet/examples/full.toml` — optional-section reference
