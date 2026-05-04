---
title: SKILL
path: hooks-control/SKILL.md
dna_hash: sha256:ccda2073f0673092
language: markdown
size_loc: 123
generated: by-keidocs
---

# hooks-control/SKILL.md

## Public API

- `Hooks Control — Runtime Hook Enable/Disable` — ---
- `When to use` — - Temporarily disabling noisy advisory hooks for the current shell session without editing `~/.claude/settings.json`.
- `Pipeline (one phase, up to 2 AskUserQuestion batches)` — ### Phase 1 — Show state + pick action
- `Disable selected hooks for this shell session:` — export KEI_DISABLED_HOOKS=<comma-joined-names>
- `Switch profile for this shell session:` — export KEI_HOOK_PROFILE=<choice>
- `Clear all runtime hook overrides (back to full / everything on):` — unset KEI_DISABLED_HOOKS KEI_HOOK_PROFILE
- `Rules` — - **Click-only.** Every decision is `AskUserQuestion`. No free-text.
- `Final report` — ```
- `References` — - `hooks/*.sh` — each kit hook sources the v0.15.1 runtime-controls block

## Related

- parent: `hooks-control`

## Discussion

<script src="https://giscus.app/client.js"
        data-repo="KeiSei84/KeiSeiKit-1.0"
        data-repo-id="PLACEHOLDER_REPO_ID"
        data-category="wiki-comments"
        data-category-id="PLACEHOLDER_CATEGORY_ID"
        data-mapping="pathname"
        data-strict="0"
        data-reactions-enabled="1"
        data-emit-metadata="0"
        data-input-position="bottom"
        data-theme="preferred_color_scheme"
        data-lang="en"
        data-loading="lazy"
        crossorigin="anonymous"
        async></script>
