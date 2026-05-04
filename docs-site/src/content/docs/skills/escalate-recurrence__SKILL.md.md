---
title: SKILL
path: escalate-recurrence/SKILL.md
dna_hash: sha256:dcef13827da81ca3
language: markdown
size_loc: 329
generated: by-keidocs
---

# escalate-recurrence/SKILL.md

## Public API

- `Escalate Recurrence — Interactive Codifier` — ---
- `Phase 0 — Sanity check (do this BEFORE asking anything)` — 1. State in ONE paragraph what you observed:
- `Phase 1 — Pure-click decisions (AskUserQuestion, SINGLE call)` — Emit all four questions in one `AskUserQuestion` invocation. Use `multiSelect: false`.
- `Phase 2 — Generate artefacts (in memory — do NOT Write yet)` — Compose all files the user will see in the diff preview:
- `RULE — <Human Pattern Name> (YYYY-MM-DD ADDED)` — > <one-sentence pattern description>.
- `Incident` — <The recurrence evidence: 2+ concrete instances with file:line or tool-call refs.>
- `The Rule` — <Actionable rule, imperative voice.>
- `Triggers` — - <Specific detectable phrase / pattern 1>
- `Enforcement` — - Hook: `~/.claude/hooks/<slug>-guard.sh` (<event>, severity <S>)
- `Why this and not "remember to check"` — <One sentence on why a hook beats memory for this pattern.>
- `Rule lock` — YYYY-MM-DD. Never override without explicit user revocation.
- `<slug>-guard — <one-line purpose>` — 
- `Severity: <S>; Event: PreToolUse:Bash` — 
- `Rule: ~/.claude/rules/<slug>.md` — command -v jq >/dev/null 2>&1 || exit 0
- `Trigger pattern — tighten during dogfooding` — case "$CMD" in
- `Phase 3 — Confirm via diff preview (AskUserQuestion — keep it click-only)` — Show all generated artefacts inline in ONE message, THEN emit a single `AskUserQuestion`:
- `Phase 4 — Write + register (on confirm only)` — Execute in order (each via its right tool — do NOT shell out when a tool exists):
- `Phase 5 — Report` — ```
- `Rules (apply throughout)` — - **Pure click wherever possible.** Only slug and pattern-literal require typing. Everything structural is AskUserQuestion.

## Related

- parent: `escalate-recurrence`

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
