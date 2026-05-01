# Phase 3 (pet-overlay) — Optional pet persona attached to this spawn

> Goal: decide whether this spawn receives a pet persona overlay, and if so
> which pet manifest to attach. The selected `pet.toml` path is stored for
> Phase 4, which passes it to `kei-spawn` as `--pet-manifest <path>` so the
> spawn ceremony bridges the overlay into the composed prompt via
> `kei_pet::compose_prompt_with_pet`.
>
> **Verify criterion:** `PET_MANIFEST_PATH` is either `None` (user declined)
> or an absolute path to a readable, `kei-pet validate`-clean `.toml` file.

This phase is additive to the existing scope/emit flow — run it AFTER
[phase-3-scope.md](phase-3-scope.md) and BEFORE [phase-4-emit.md](phase-4-emit.md).

---

## 3-pet.a — First AskUserQuestion: attach a pet?

Send ONE `AskUserQuestion`:

```json
{
  "questions": [
    {
      "question": "Apply a pet persona to this spawn?",
      "header": "Persona",
      "multiSelect": false,
      "options": [
        {
          "label": "Yes",
          "description": "Attach one pet.toml manifest from ~/.claude/pet/. The overlay prepends the persona voice/edge/forbidden-topics block to the agent's system prompt."
        },
        {
          "label": "No",
          "description": "Skip the persona overlay. The spawn uses the base prompt only — identical to pre-pet spawn behaviour."
        }
      ]
    }
  ]
}
```

Store the clicked label as `PET_ATTACH`.

- **No** → set `PET_MANIFEST_PATH = None`, emit confirmation
  `Persona: none (base prompt only)` and proceed to Phase 4.
- **Yes** → continue to 3-pet.b.

---

## 3-pet.b — Discover available pets

Run exactly one bash command (no chaining, so errors surface):

```bash
ls -1 ~/.claude/pet/*.toml 2>/dev/null | sort
```

Collect the stdout lines as `DISCOVERED`. Cases:

- **Zero files** — no manifests on disk. Offer three constructive paths:
  - (A) Run `/new-pet` to author the first one (recommended path).
  - (B) Loop back to 3-pet.a and click **No** to proceed without a pet.
  - (C) Abort the spawn — no task.toml written, no ledger row.
  Do NOT fabricate a default pet; do NOT fall through silently.
- **One file** — auto-select it, show the path, skip 3-pet.c, proceed to
  3-pet.d for validation. Log `Persona: single pet auto-selected: <path>`.
- **Two or more files** — continue to 3-pet.c.

---

## 3-pet.c — Second AskUserQuestion: which pet?

Build one option per discovered `.toml`. The `label` is the bare filename
(no extension, no directory). The `description` is a short preview of the
manifest — `pet_name` + `user_name` + `tone_primary` read out with two
extra bash calls (kept cheap):

```bash
awk -F'"' '/^pet_name/    {print $2}' <path>
awk -F'"' '/^user_name/   {print $2}' <path>
awk -F'"' '/^tone_primary/{print $2}' <path>
```

If any awk fails or returns empty, use the filename alone as the
description — do NOT fabricate fields.

Skeleton:

```json
{
  "questions": [
    {
      "question": "Which pet?",
      "header": "Pet",
      "multiSelect": false,
      "options": [
        {
          "label": "<basename-1>",
          "description": "<pet_name> — companion to <user_name>, tone <tone_primary>"
        },
        {
          "label": "<basename-2>",
          "description": "..."
        }
      ]
    }
  ]
}
```

Cap the option count at 10. If the user has >10 pets, include the first 9
alphabetically plus an "Enter path manually" tail option that triggers a
free-text prompt accepting an absolute path; re-validate via 3-pet.d.

Store the resolved absolute path as `PET_MANIFEST_PATH`.

---

## 3-pet.d — Validate the selected manifest

Run exactly one command:

```bash
kei-pet validate "<PET_MANIFEST_PATH>"
```

Fall back to `"$KEI_RUNTIME_BIN_DIR/kei-pet"` on `command not found`, mirroring
the SKILL.md runtime-resolution rule. If both fail, STOP and surface the
three install paths (A build / B export / C install.sh) — do NOT emit the
Agent-tool invocation.

On `kei-pet validate` non-zero exit: print stderr verbatim and loop back
to 3-pet.c (give the user a chance to pick a different pet). On a persistent
fail across two attempts, drop to the NO DOWNGRADE failure paths below.

---

## 3-pet.e — Verify criterion

- `PET_MANIFEST_PATH` is either `None` or an absolute filesystem path.
- When set, the file exists and `kei-pet validate` exits 0.
- No free-text was typed in 3-pet.a or 3-pet.c (only the manual-path tail
  case permits one free-text entry).

Emit confirmation:

`Persona locked: <pet_name>@<basename>.toml` or `Persona: none`.

Proceed to Phase 4. The emit phase adds `--pet-manifest <path>` to the
`kei-spawn spawn` invocation when `PET_MANIFEST_PATH` is set. The runtime
uses `kei_pet::compose_prompt_with_pet` to bridge the overlay onto the base
prompt before handing the final string to the Agent tool.

---

## 3-pet.f — Failure paths (NO DOWNGRADE)

- (A) No pets on disk → offer `/new-pet`, NOT "skip silently". The user
  clicked **Yes** in 3-pet.a for a reason.
- (B) Selected manifest fails validation twice → show the first two error
  lines verbatim, then offer: fix the pet (exit skill), pick a different
  pet (loop to 3-pet.c), or fall back to no persona (loop to 3-pet.a).
- (C) `kei-pet` binary missing → do NOT skip the validation step. Surface
  the install paths. A spawn with an unvalidated persona is worse than
  no spawn at all — the overlay is prepended to the agent prompt and a
  malformed manifest propagates there.

---

## Rules (inherit from SKILL.md)

- **Pure-click contract.** At most one free-text entry in this phase, and
  only in the manual-path tail of 3-pet.c (10+ pets edge case).
- **NO HALLUCINATION (RULE 0.4).** Never invent `pet_name`, `tone_primary`,
  or any preview field — read them from the file or leave the description
  as the bare filename.
- **Orchestrator branch first (RULE 0.13).** This phase does not invoke
  git, does not write to the project tree. It only reads `~/.claude/pet/*.toml`
  and shells out to `kei-pet validate`.
- **Constructor Pattern (RULE ZERO).** This file stays <200 LOC.
