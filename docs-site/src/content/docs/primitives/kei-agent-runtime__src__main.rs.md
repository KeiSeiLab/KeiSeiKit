---
title: main.rs
path: kei-agent-runtime/src/main.rs
dna_hash: sha256:331015e67e89e1c3
language: rust
size_loc: 199
generated: by-keidocs
---

# kei-agent-runtime/src/main.rs

kei-agent-runtime — CLI dispatcher for compose | spawn | verify | run.

## Public API

- Compose prompt from a task.toml and write tasks/<agent-id>/prompt.md.
- Prepare spawn dir (tasks/<agent-id>/) — orchestrator invokes Agent tool.
- Run every verify capability declared by the task's role.
- One-shot helper: compose + spawn + verify (tests only).
- Assemble everything orchestrator needs to invoke Agent tool.
- Output format: human (default) | json | toml

## Related

- parent: `kei-agent-runtime/Cargo.toml`
- imports: clap, kei_agent_runtime, std

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
