---
title: simulated_merge.rs
path: kei-agent-runtime/src/simulated_merge.rs
dna_hash: sha256:db16b7b926add583
language: rust
size_loc: 113
generated: by-keidocs
---

# kei-agent-runtime/src/simulated_merge.rs

Simulated-merge executor + glob matcher.

Schema §Verify execution — worktree short-circuit → simulated merge:
orchestrator creates temp worktree off main, applies agent's diff, runs
verifies from that vantage to catch integration regressions invisible
in agent's isolated worktree.

## Public API

- `pub fn run_simulated_merge` — Create a temp worktree off `main_repo` at HEAD of `main`, apply the agent's
- `pub fn apply_diff` — Apply a unified diff to `dir` via `git apply --index`. Empty diff is a no-op.
- `pub fn run_git` — Run `git <args>` in `dir`, return stdout as UTF-8 string.
- `pub fn glob_match` — Shell-style glob match. Supports `**` (any directories) and `*` (any chars

## Related

- parent: `kei-agent-runtime/Cargo.toml`
- imports: anyhow, crate, std

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
