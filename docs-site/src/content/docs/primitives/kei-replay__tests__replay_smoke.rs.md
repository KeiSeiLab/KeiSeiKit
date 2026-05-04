---
title: replay_smoke.rs
path: kei-replay/tests/replay_smoke.rs
dna_hash: sha256:9c61622780094205
language: rust
size_loc: 229
generated: by-keidocs
---

# kei-replay/tests/replay_smoke.rs

Smoke tests for kei-replay.

Covers: happy-path replay, missing DNA, drift detection, diff.

Each test builds its own isolated tempdir with:
- SQLite ledger seeded with the relevant `agents` row
- `<worktree>/tasks/<agent-id>/task.toml`
- `<kit_root>/_roles/<role>.toml`
- `<kit_root>/_capabilities/<cat>/<slug>/text.md`
Then calls `replay::replay` / `diff::diff` directly (skips the CLI layer).

## Related

- parent: `kei-replay/tests`
- imports: kei_agent_runtime, kei_replay, rusqlite, std, tempfile

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
