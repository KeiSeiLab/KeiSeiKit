---
title: prepare.rs
path: kei-agent-runtime/src/prepare.rs
dna_hash: sha256:9c3174be3a894bdf
language: rust
size_loc: 199
generated: by-keidocs
---

# kei-agent-runtime/src/prepare.rs

Orchestrator-facing wrapper: task.toml → everything needed to invoke
Claude Code's `Agent` tool in a single copy-paste-ready bundle.

Per RULE 0.13, the orchestrator (main session) owns branch creation,
`isolation: "worktree"` selection, and the actual Agent-tool call. This
module only assembles the arguments — no git, no spawn, no shell.

Wire: `prepare()` = role resolution + `compose_prompt()` + role→subagent_type
resolution + `Dna::compose`. Deliberately does NOT create `tasks/<id>/` on
disk (that is `spawn::prepare_agent`'s job) so orchestrator can inspect
before committing. The "ledger row" field is a pretty-printed string, not
a DB write — ledger persistence is the orchestrator's step.

## Public API

- Everything the orchestrator needs to hand the Claude `Agent` tool.
- Layer G — composition fingerprint, `<role>::<caps>::<scope>::<body>-<nonce>`.
- `pub fn prepare` — Assemble an `AgentInvocation` from a parsed task.toml.
- Auto-generate agent-id if absent, otherwise reuse + validate.
- Load role metadata and assert it is spawnable (RULE 0.13 guard).
- Compose Layer G DNA fingerprint using the effective (resolved) agent-id.
- `pub fn render_human` — Human-readable block — copy into Claude Code's Agent-tool dialog.

## Related

- parent: `kei-agent-runtime/Cargo.toml`
- imports: anyhow, crate, serde, std

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
