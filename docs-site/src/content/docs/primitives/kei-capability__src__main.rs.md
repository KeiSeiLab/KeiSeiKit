---
title: main.rs
path: kei-capability/src/main.rs
dna_hash: sha256:c899b0ff1b6a2c66
language: rust
size_loc: 172
generated: by-keidocs
---

# kei-capability/src/main.rs

kei-capability — hook-protocol CLI adapter.

Subcommands:
- `check <name>`  — reads tool-use JSON from stdin, runs registry
gate, emits permissionDecision JSON, exits 0 or 2.
- `verify <name>` — reads env (AGENT_ID, TASK_TOML, WORKTREE_PATH,
MAIN_REPO, RUN_MODE), runs registry verify,
exits 0 on pass or non-zero with stderr message.
- `fork <source> --as <new-name> [--kit-root <dir>]` — copy an
existing capability dir under a new
`<cat>::<slug>` name and record lineage.

## Public API

- PreToolUse gate — stdin holds hook payload JSON.
- On-return verify — env carries context.
- Fork a capability: copy dir under a new <cat>::<slug> name with lineage.
- Existing `<cat>::<slug>` to clone.
- New `<cat>::<slug>` name for the fork.
- Kit root (contains `_capabilities/`); defaults to cwd.

## Related

- parent: `kei-capability/Cargo.toml`
- imports: clap, kei_agent_runtime, kei_capability, serde_json, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
