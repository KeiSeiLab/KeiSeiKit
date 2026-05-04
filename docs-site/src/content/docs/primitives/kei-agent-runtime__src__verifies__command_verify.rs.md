---
title: command_verify.rs
path: kei-agent-runtime/src/verifies/command_verify.rs
dna_hash: sha256:6c53baa2d3fc664f
language: rust
size_loc: 142
generated: by-keidocs
---

# kei-agent-runtime/src/verifies/command_verify.rs

Generic command-driven on-return verify (Layer D convergence, 2026-04-23).

Absorbs the "run external command, check exit, optionally parse output"
shape shared by `quality::cargo-check-green`, `quality::tests-green`,
`safety::no-dep-bump` verify. Each concrete verify is now a
`CommandVerify` const declaration in its own file; execution logic lives
here.

`quality::constructor-pattern` (LOC walker) and the `output::*` verifies
(parse agent report, no subprocess) stay in their own modules.

## Public API

- Where to run the command from.
- `<run_dir>/_primitives/_rust` if it exists, else `<run_dir>`.
- Raw `ctx.run_dir()`.
- Raw `ctx.worktree_path` (pre-merge, for git-diff verifies).
- `pub type ArgBuilder` — Decides how to build argv from `ctx` + config.
- `pub type ResultMapper` — Post-processes `Output` into a `VerifyResult`. Default = exit-code check.
- `pub struct CommandVerify` — Generic command-runner verify capability.
- Executable name (e.g. "cargo", "git").
- Literal args joined before per-crate / per-target dispatch. Used by
- Human-readable failure reason when exit != expected.
- If set, overrides the default "one shot, expected_exit" runner.
- If set, overrides `default_args`.
- If set, overrides `default_result_mapper`.
- `pub fn default_exit_mapper` — Default result mapper: pass iff exit == `expected_exit`, else Fail with
- `pub fn tail` — Utility: last `n` lines of `bytes` as a String (lossy utf-8).
- `pub fn run_in` — Helper: run `cmd args...` in `dir`, return Output or stringified err.

## Related

- parent: `kei-agent-runtime/Cargo.toml`
- imports: crate, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
