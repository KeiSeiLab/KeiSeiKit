---
title: main.rs
path: kei-spawn/src/main.rs
dna_hash: sha256:241bcfc4826369c1
language: rust
size_loc: 174
generated: by-keidocs
---

# kei-spawn/src/main.rs

kei-spawn — CLI dispatcher.

Four subcommands:
- `spawn <task.toml>` — prepare invocation + ledger fork, emit JSON
- `drive <task.toml>` — spawn + attempt driver invocation (v0.1: stub,
returns exit 64 NotImplemented after emitting SpawnOutput JSON)
- `verify <agent-id> <worktree>` — run verify pipeline, update ledger
- `list-pending` — forward `kei-ledger list --status running`

Exit codes:
0  success (spawn, verify-clean, list-pending)
1  generic failure (any Err from the pipeline)
2  verify-failed (capabilities failed but pipeline ran)
64 drive NotImplemented (v0.1 stub path)

## Public API

- Prepare an Agent-tool invocation + register ledger row.
- Path to task.toml.
- kit root (default: cwd).
- Also derive downstream handoff chain from role's `[pipeline]`
- Spawn + invoke driver (v0.1: stub — emits SpawnOutput then exit 64).
- Path to task.toml.
- kit root (default: cwd).
- Run verify pipeline + update ledger status.
- agent-id previously emitted by `kei-spawn spawn`.
- Worktree path reported by the Claude harness on agent return.
- Show all running ledger rows.

## Related

- parent: `kei-spawn/Cargo.toml`
- imports: clap, kei_spawn, serde, std

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
