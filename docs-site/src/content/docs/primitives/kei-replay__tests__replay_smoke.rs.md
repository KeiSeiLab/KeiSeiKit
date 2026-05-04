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
