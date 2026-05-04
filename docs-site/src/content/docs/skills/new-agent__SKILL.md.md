---
title: SKILL
path: new-agent/SKILL.md
dna_hash: sha256:6003cce86d4ee6a6
language: markdown
size_loc: 485
generated: by-keidocs
---

# new-agent/SKILL.md

## Public API

- `New Agent — Project-Specialist Wizard` — ---
- `Phase 1 — Option-picker questions (AskUserQuestion, ONE call)` — Send ALL FOUR questions in a SINGLE `AskUserQuestion` invocation so the user picks them in one pass. Use `multiSelect: false` for every question. Do NOT use free-text here.
- `Phase 1b — Follow-up (AskUserQuestion, ALWAYS run — one call)` — This call ALWAYS runs. Q6 (scrapers) is independent of Q2/Q3, so every agent answers it. Q5 defaults "No" only if user explicitly picks so.
- `Phase 2 — Free-text prompts (regular message, NOT AskUserQuestion)` — Ask the user to reply in one message with all four fields. Use this exact prompt:
- `Phase 3 — Compose the manifest` — ### 3.1 Compute `blocks` array
- `Phase 3.5 — Final name confirmation (AskUserQuestion, ONE call)` — Before writing the manifest, give the user one explicit chance to confirm or override the agent name. Send this `AskUserQuestion`. Substitute the literal slug from Phase 2 into every option label so the user sees, for example, `kei-myapp-specialist` (NOT the literal `kei-<slug>-specialist` placeholder).
- `Phase 3.6 — Cognitive modes (AskUserQuestion, ONE call, optional)` — Cognitive-mode blocks (`_blocks/mode-*.md`) add a behavioural skew to the generated agent. They compose — multi-selection is expected. **Default: pick NONE** if unsure; modes are not free (each lands verbatim in the prompt).
- `Phase 4 — Fill the template + write the manifest` — 1. Read `~/.claude/agents/_templates/specialist.toml.template` via the Read tool.
- `Phase 5 — Validate + assemble` — Run validate first, assemble only on success:
- `Phase 6 — Report` — Show a concise block to the user. `<FINAL_NAME>` is the name resolved in Phase 3.5 (default `kei-<slug>-specialist`, or the user's override).
- `Phase 8 — Project bridges (optional, click-only)` — After reporting the new agent, offer to generate cross-tool bridge files for the project's working tree (so Cursor, Copilot, Aider, Windsurf, Junie, Continue, Gemini/Antigravity, Replit, Codex CLI, Warp, Zed all see the same Constructor-Pattern ruleset). Send this `AskUserQuestion`:
- `Phase 7 — Suggested next steps (print, do NOT execute without ask)` — Offer as a final block the user can copy-paste:
- `1. Create project memory file (adjust path to your memory layout)` — touch ~/.claude/memory/<slug>-project.md
- `2. Add one-line entry to your MEMORY.md index under "## Projects"` — 
- `e.g. [[<slug>-project]] — <one-line description>` — 
- `3. Commit the new agent` — cd ~/.claude && git add \
- `Rules (apply throughout the wizard)` — - NO DOWNGRADE: every failure mode above must return constructive paths, not "can't do it".

## Related

- parent: `new-agent`

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
