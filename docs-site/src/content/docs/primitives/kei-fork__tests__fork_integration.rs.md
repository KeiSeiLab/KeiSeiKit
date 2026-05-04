---
title: fork_integration.rs
path: kei-fork/tests/fork_integration.rs
dna_hash: sha256:1a592b4f20cc2998
language: rust
size_loc: 406
generated: by-keidocs
---

# kei-fork/tests/fork_integration.rs

Integration tests for kei-fork — hermetic, ledger skipped.

Each test spins up a fresh `TempDir`, runs `git init` + initial
commit, then drives the public API. `KEI_FORK_SKIP_LEDGER=1` keeps
the test tree free of SQLite side-effects.

NOTE: `KEI_FORK_SKIP_LEDGER` is process-wide. Tests set it once in
`setup_kit()` — do not unset mid-test.

Tests that mutate *other* env vars (`KEI_FORK_FORCE_LEDGER_FAIL`,
`KEI_FORK_GIT_BIN`) serialize against all other tests via the
`ENV_LOCK` mutex below — cargo test runs in parallel by default and
leaking a binary override into a peer test would be catastrophic.

## Public API

- Serializes every test in this binary. Cargo runs tests in parallel
- HIGH #1 (Critic F1): `git add -A` used to bleed `.DONE` and
- HIGH #1 secondary: kit-internal prefixes (`_forks/`, `_archive/`)
- HIGH #2 (Critic F7a): `create()` must roll back the worktree and
- HIGH #3 (Security #3): refname validator must reject a
- HIGH #3: NUL byte inside the refname must be rejected before git
- HIGH #3: dotty traversal also rejected.
- HIGH #4 (Security #4): the `git` binary MUST be resolvable via the

## Related

- parent: `kei-fork/tests`
- imports: kei_fork, std, tempfile

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
