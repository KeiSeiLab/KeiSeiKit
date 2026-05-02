---
name: hooks-control
description: Runtime enable/disable of KeiSeiKit hooks via env vars (v0.15.1). Click-only wizard that emits shell `export` / `unset` commands for the user to paste. Supports per-hook disable, profile switch (full / advisory-off / minimal / off), or full re-enable. Does NOT execute anything — user controls their shell.
argument-hint: (none — fully click-driven)
---

# Hooks Control — Runtime Hook Enable/Disable

## When to use

- Temporarily disabling noisy advisory hooks for the current shell session without editing `~/.claude/settings.json`.
- Switching hook profiles (full / advisory-off / minimal / off) for a focused debugging session.
- Re-enabling hooks after a session where they were selectively turned off.

Click-only wizard. Helps you toggle KeiSeiKit hooks **for the current shell
session** via env vars, without editing `~/.claude/settings.json`. The skill
emits shell commands; it NEVER runs them.

Two env vars are honoured by every kit-shipped hook (v0.15.1+):

| Var | Meaning |
|---|---|
| `KEI_DISABLED_HOOKS` | Comma- or space-list of hook base names (no `.sh`). Matching is **tokenized exact-match** (v0.15.1 fix — earlier versions used substring-glob, which let `foo-all-bar` disable every hook). The literal `all` token still disables every hook. |
| `KEI_HOOK_PROFILE` | One of `full` (default), `advisory-off`, `minimal`, `off`. |

| Profile | What stays on |
|---|---|
| `full` (default) | Every hook |
| `advisory-off` | Disables pure-stderr advisories: `recurrence-suggest`, `citation-verify`, `error-spike-detector`, `milestone-commit-hook`. |
| `minimal` | Only the four kit-shipped hooks needed for structural integrity or observability: `no-hand-edit-agents`, `assemble-validate`, `agent-fork-logger`, `session-end-dump`. User-global safety hooks (``, `secrets-guard`, ``, ``) are not shipped by the kit but are respected when present in `~/.claude/hooks/`. |
| `off` | Every hook off (escape hatch — use when debugging hook interactions). |

---

## Pipeline (one phase, up to 2 AskUserQuestion batches)

### Phase 1 — Show state + pick action

Print current state:
```
Current KEI_DISABLED_HOOKS: <value or "(unset)">
Current KEI_HOOK_PROFILE:   <value or "full (default)">
Active kit-shipped hooks:   <list of 9 minus disabled set>
```

`AskUserQuestion` — **What do you want to do?**
1. Disable specific hook(s) — this shell session only
2. Switch profile — `full` / `advisory-off` / `minimal` / `off`
3. Re-enable everything — clear both env vars
4. Show state only — emit no commands

### Phase 2a — Hook multi-select (if picked 1)

`AskUserQuestion` multi-select over the 10 kit-shipped hook names:
`assemble-agents`, `assemble-validate`, `no-hand-edit-agents`, `tomd-preread`,
`agent-fork-logger`, `orchestrator-dirty-check`, `site-wysiwyd-check`,
`error-spike-detector`, `milestone-commit-hook`, `session-end-dump`.

Emit:
```sh
# Disable selected hooks for this shell session:
export KEI_DISABLED_HOOKS=<comma-joined-names>
```
For persistence, tell the user to paste into `~/.zshrc` / `~/.bashrc` by
hand. Do NOT edit rc files.

### Phase 2b — Profile picker (if picked 2)

`AskUserQuestion` over `full` / `advisory-off` / `minimal` / `off`. Emit:
```sh
# Switch profile for this shell session:
export KEI_HOOK_PROFILE=<choice>
```

### Phase 2c — Re-enable everything (if picked 3)

Emit directly (no further question):
```sh
# Clear all runtime hook overrides (back to full / everything on):
unset KEI_DISABLED_HOOKS KEI_HOOK_PROFILE
```

### Phase 2d — State only (if picked 4)

Stop after the state block.

---

## Rules

- **Click-only.** Every decision is `AskUserQuestion`. No free-text.
- **Never execute.** The skill prints shell commands as code blocks; the
  user runs them. Any `export` from a tool call would evaporate at skill
  exit — the shell running hooks is a subshell.
- **No rc edits.** If the user wants persistence, we say "paste into your
  shell rc". The skill MUST NOT modify `~/.zshrc` / `~/.bashrc`.
- **RULE 0.4 — no invented hook names.** Only the 10 names in Phase 2a
  are valid choices. Never suggest a name not in the kit.
- **RULE -1 — NO DOWNGRADE.** If the user asks "can I silence all safety
  hooks?", present tradeoffs; point at `KEI_HOOK_PROFILE=off` with a
  warning that sensitive-content and generated-file protections also go down.

---

## Final report

```
=== HOOKS-CONTROL REPORT ===
Action:     <picked option>
Commands:   <N lines emitted>
Scope:      current shell session (unless pasted into rc)
Verify:     `env | grep KEI_` after pasting
Undo:       unset KEI_DISABLED_HOOKS KEI_HOOK_PROFILE
```

---

## References

- `hooks/*.sh` — each kit hook sources the v0.15.1 runtime-controls block
- `README.md` → "Runtime hook controls" section
- `~/.claude/rules/recurrence-escalate.md` — severity ladder notes that
  hooks can be silenced at runtime, no rule deletion required
