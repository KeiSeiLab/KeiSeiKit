---
title: SKILL
path: spawn-agent/SKILL.md
dna_hash: sha256:db7829fa606c340b
language: markdown
size_loc: 137
generated: by-keidocs
---

# spawn-agent/SKILL.md

## Public API

- `/spawn-agent — Click-only Agent-tool composer (index)` — ---
- `When to use` — - Composing a ready-to-paste `Agent`-tool invocation via the `kei-spawn` CLI without writing it by hand.
- `Pipeline overview` — | Phase | File | Purpose | AskUserQuestion |
- `Variables the pipeline produces` — | Name | Set in | Meaning |
- `Role → defaults map (LOAD-BEARING)` — | ROLE | subagent_type | isolation | Bash? | Writes? |
- `Final report (emit after Phase 4)` — ```
- `Runtime binary resolution` — `kei-spawn` must be on `PATH` OR reachable via `$KEI_RUNTIME_BIN_DIR`. The
- `Rules (enforced at every phase)` — - **Pure-click contract.** Only `TASK` (Phase 2) is typed. Every other
- `References` — - [phase-1-role.md](phase-1-role.md) · [phase-2-task.md](phase-2-task.md) · [phase-3-scope.md](phase-3-scope.md) · [phase-4-emit.md](phase-4-emit.md)

## Related

- parent: `spawn-agent`

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
