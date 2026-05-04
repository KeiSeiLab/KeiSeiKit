---
title: phase-7-assemble
path: compose-solution/phase-7-assemble.md
dna_hash: sha256:18fd76e8ade1d715
language: markdown
size_loc: 156
generated: by-keidocs
---

# compose-solution/phase-7-assemble.md

## Public API

- `Phase 7 — Recipe assembly (branches on `T`)` — Before branching, resolve auto-detect if `T == "Auto-detect"`.
- `7a — Resolve auto-detect (conditional)` — Infer target type from architecture (Phase 5):
- `7b — Agent branch` — Hand off to the `new-agent` skill — it already codifies the 8-phase wizard
- `7c — Skill branch` — Compose a new `skills/<slug>/SKILL.md` inline:
- `<Human Name> — <one-line>` — <2-3 sentences: what the skill does, when to invoke, who owns the output.>
- `Phase 1 — Intake (<AskUserQuestion | free-text>)` — <Derived from architecture Phase 5. Escalate-recurrence style: if the
- `Phase 2 — <Action>` — <Steps derived from Phase 5 expression. Verify-criterion per step.>
- `Phase 3 — Report` — <What the user sees at the end. Concise report block.>
- `Rules` — <Borrow from _blocks/baseline.md if generic enforcement needed.>
- `7d — Hook branch` — Delegate to the `escalate-recurrence` skill
- `7e — Rule branch` — Same handoff as 7d — `escalate-recurrence` owns the rule + wiki pipeline
- `7f — Block only` — Already handled in Phase 6. Skip to final report.
- `Verify-criterion` — - Exactly one branch ran (7b / 7c / 7d / 7e / 7f).

## Related

- parent: `compose-solution`

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
