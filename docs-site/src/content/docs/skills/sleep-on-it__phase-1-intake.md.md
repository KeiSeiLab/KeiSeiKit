---
title: phase-1-intake
path: sleep-on-it/phase-1-intake.md
dna_hash: sha256:718ffdebc7b4bad6
language: markdown
size_loc: 58
generated: by-keidocs
---

# sleep-on-it/phase-1-intake.md

## Public API

- `Phase 1 — Task intake (one free-text field)` — The single free-text field in the wizard. Everything else is a click.
- `1a — Prerequisite check` — Before prompting, verify the v0.11 pipeline is live:
- `Resolve sync-repo path from env or secrets file.` — 
- `shellcheck disable=SC1091` — [ -f "${HOME}/.claude/secrets/.env" ] && . "${HOME}/.claude/secrets/.env"
- `1b — Free-text prompt` — Emit a plain chat message (NOT `AskUserQuestion` — a free-text message
- `1c — Validate` — - Reject if `TASK_TEXT` is empty or only whitespace.
- `Verify-criterion` — - `TASK_TEXT` is non-empty and <= 4000 chars.

## Related

- parent: `sleep-on-it`

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
