---
title: lib.rs
path: kei-spawn/src/lib.rs
dna_hash: sha256:ad4d277e7692acd5
language: rust
size_loc: 49
generated: by-keidocs
---

# kei-spawn/src/lib.rs

kei-spawn — automation envelope around kei-agent-runtime + kei-ledger.

Orchestrator flow pre-kei-spawn:
1. Write task.toml manually
2. Run `kei-agent-runtime prepare`
3. Invoke Agent tool (harness-internal, orchestrator-only)
4. Run `kei-ledger fork`
5. On return, run `kei-agent-runtime verify`

With kei-spawn, steps 2 + 4 collapse to one `kei-spawn spawn <task.toml>` call
and step 5 collapses to one `kei-spawn verify <agent-id> <worktree>` call.
Step 3 (the actual Agent tool invocation) STILL belongs to the orchestrator
because Claude Code's `Agent` tool is harness-internal — it can't be invoked
from Rust. `kei-spawn` emits a JSON bundle the orchestrator pastes.

Design constraints:
- Constructor Pattern: one module = one responsibility, ≤200 LOC file,
≤30 LOC fn.
- Optional HTTP via the `http-driver` Cargo feature (reqwest::blocking +
rustls). Off by default — v0.1 ships `ManualDriver` only.
- No git / no shell — ledger interactions go through `kei-ledger` as a
subprocess to avoid adding kei-ledger as a direct dep while it still
lacks a lib.rs (can't link to a bin-only crate).

Per RULE 0.13: kei-spawn NEVER creates branches or commits. The orchestrator
owns git state. kei-spawn only writes into `tasks/<agent-id>/` and invokes
`kei-ledger` (which itself only writes to SQLite).

## Related

- parent: `kei-spawn/Cargo.toml`

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
