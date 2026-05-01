# Phase 3 — Scope (files whitelist + optional denylist)

> Goal: produce a concrete `WHITELIST` (glob patterns the agent may touch)
> and optionally an explicit `DENYLIST`. For read-only / explorer roles
> the whitelist is advisory; for edit-* roles it is ENFORCED by kei-spawn.
> **Verify criterion:** `WHITELIST` non-empty list of glob strings.

---

## 3.a — First AskUserQuestion: scope preset

Send ONE `AskUserQuestion` call. Presets cover ≥80% of real invocations;
pick "Custom" only if none fit.

```json
{
  "questions": [
    {
      "question": "Scope preset?",
      "header": "Scope",
      "multiSelect": false,
      "options": [
        {
          "label": "Single crate (Rust)",
          "description": "crates/<name>/** — typical edit-local scope. Also includes that crate's tests/ and Cargo.toml."
        },
        {
          "label": "Single skill (Markdown)",
          "description": "skills/<name>/** — pure-markdown skill authoring. No code compilation."
        },
        {
          "label": "Single agent manifest",
          "description": "agents/_manifests/<name>.toml + agents/_blocks/*.md — agent fleet authoring."
        },
        {
          "label": "Docs / rules only",
          "description": "**/*.md — read-only or explorer roles that only touch documentation."
        },
        {
          "label": "Whole project (read-only)",
          "description": "** — read-only or explorer roles. Not valid for edit-* (too broad; use edit-shared with explicit globs)."
        },
        {
          "label": "Custom",
          "description": "Enter glob patterns as one free-text line (comma- or newline-separated)."
        }
      ]
    }
  ]
}
```

Store the clicked label as `SCOPE_PRESET`.

---

## 3.b — Resolve preset to `WHITELIST`

- **Single crate (Rust)** → follow up with ONE free-text prompt: `Crate name?`
  Validate `[a-z0-9-]+`. Build: `[ "crates/<name>/**", "Cargo.toml" ]`.
- **Single skill (Markdown)** → follow up: `Skill name?` Validate `[a-z0-9-]+`.
  Build: `[ "skills/<name>/**" ]`.
- **Single agent manifest** → follow up: `Agent name?` Validate `[a-z0-9-]+`.
  Build: `[ "agents/_manifests/<name>.toml", "agents/_blocks/*.md" ]`.
- **Docs / rules only** → Build: `[ "**/*.md" ]`. Warn if ROLE is edit-* —
  docs-only edits rarely need worktree isolation; suggest `explorer` or
  `read-only` instead.
- **Whole project (read-only)** → BLOCK if ROLE is edit-local or edit-shared.
  Print: "Whole-project scope is not allowed for edit roles. Use edit-shared
  with explicit globs naming the ≥2 modules you will touch." Loop back to 3.a.
  Otherwise build: `[ "**" ]`.
- **Custom** → follow up: `Enter glob patterns (comma- or newline-separated).`
  Parse, trim, validate each glob against the rules in 3.c. Build the list.

---

## 3.c — Glob validation rules

Apply to every pattern in `WHITELIST`:

1. **No absolute paths.** Must not start with `/` or `~/`. globs are
   repo-relative.
2. **No parent traversal.** Reject any pattern containing `..`.
3. **No leading dot-dir unless explicit.** `.git/**`, `.claude/**` must
   be typed in full; reject accidental `.**`.
4. **At least one literal char.** Reject `**` alone without a scoping prefix
   unless ROLE is read-only or explorer AND SCOPE_PRESET was "Whole project".
5. **Max count.** ≤20 globs. If the user pastes more, ask them to consolidate.

On any failure, print the offending pattern and the rule that tripped;
re-prompt for that one line; do NOT fall through.

---

## 3.d — Second AskUserQuestion: explicit denylist?

Send the second `AskUserQuestion` call:

```json
{
  "questions": [
    {
      "question": "Denylist?",
      "header": "Deny",
      "multiSelect": false,
      "options": [
        {
          "label": "Auto (recommended)",
          "description": "kei-spawn applies the role-default denylist: secrets/**, **/*.env, target/**, node_modules/**, .git/**, dist/**, .keisei/** — covers 95% of cases."
        },
        {
          "label": "Explicit",
          "description": "Enter additional deny globs on top of the auto default. Use when the task whitelist accidentally includes sensitive subpaths."
        },
        {
          "label": "None (override auto)",
          "description": "Override the auto defaults and pass an empty denylist. BLOCKED for edit-* roles — read-only / explorer only."
        }
      ]
    }
  ]
}
```

Resolve:

- **Auto** → `DENYLIST = []`, let `kei-spawn` apply its role defaults. Most
  common path.
- **Explicit** → follow up: `Enter deny globs (comma- or newline-separated).`
  Validate via 3.c rules. `DENYLIST = [ "<user globs>" ]`. `kei-spawn` will
  UNION these with the role defaults (not replace).
- **None (override auto)** → if ROLE ∈ {edit-local, edit-shared} BLOCK and
  loop back. Otherwise set a marker `DENYLIST_OVERRIDE = true`; Phase 4
  will pass `--no-default-deny` to `kei-spawn`. Warn the user that this
  disables the `secrets/**` and `.env` safety nets.

---

## 3.e — Verify criterion

- `WHITELIST` is a non-empty list (length ≥ 1).
- Every pattern passes 3.c validation.
- `DENYLIST` resolved (may be empty list — Auto path).
- If ROLE is edit-* and WHITELIST == `[ "**" ]`, REJECT and loop to 3.a.

Emit confirmation:

`Scope locked: <N> whitelist globs, deny=<auto|explicit:N|override>`

Proceed to Phase 4.

---

## 3.f — Failure paths (NO DOWNGRADE)

If the user cannot choose a preset and Custom produces invalid globs twice:

- (A) Offer to inspect the current repo with `rg --files | head -50` and
  propose 2-3 concrete whitelists based on what's actually there.
- (B) Suggest downgrading ROLE from `edit-shared` to `explorer` — explorer
  accepts `[ "**" ]` and still reads everything, without write risk.
- (C) Abort this invocation and ask the user to run `/spawn-agent` again
  once the target files are clearer.
