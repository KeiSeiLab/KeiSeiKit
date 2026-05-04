---
title: INTEGRATION
path: kei-cortex/src/tool/INTEGRATION.md
dna_hash: sha256:6fccfe0d2e45d967
language: markdown
size_loc: 230
generated: by-keidocs
---

# kei-cortex/src/tool/INTEGRATION.md

## Public API

- `Integration handoff — `tool/` substrate → `handlers/chat.rs`` — 
- `Wave 44c update (2026-04-24, F-HIGH-5)` — `run_with_tools` now takes `tokio_util::sync::CancellationToken` instead of
- `What's ready (in this branch)` — - `kei_cortex::tool::ToolRegistry::with_project_root(...)` — production
- `Wave 44a — REQUIRED orchestrator-side patch (chat_stream.rs)` — The current `handlers/chat_stream.rs::run_loop_stream` calls
- `What Wave 44a fixes (security findings)` — | Finding | Fix location |
- `Cubes added` — - `tool/path_sandbox.rs` — chroot + basename + dotfile deny (~150 LOC)
- `Cubes rewritten` — - `tool/bash.rs`           — tokenizer-based check (shell-words crate)
- `New workspace deps (Cargo.toml)` — ```toml
- `SSRF + secrets handling notes for the orchestrator` — - `webfetch.rs` reject DNS-resolved IPs in private/loopback/link-local/
- `The 3-line patch the orchestrator applies` — Inside `handlers/chat.rs::chat()`, replace the current
- `Notes for the orchestrator` — - `ModelInvoker` is `Arc<dyn Fn(...) -> BoxFuture<...>>`. Build it once at
- `Why this is a handoff, not a wired change` — RULE 0.13 — orchestrator owns merges. Six parallel agents are touching
- `Reconciling `atomic_write` with `handlers/tool_apply.rs`` — `handlers/tool_apply.rs:172-185` has its own private `atomic_write`
- `Test changes summary` — - `tests/bash_sandbox_denies.rs` — extended from 18 to 47 cases

## Related

- parent: `kei-cortex/Cargo.toml`

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
