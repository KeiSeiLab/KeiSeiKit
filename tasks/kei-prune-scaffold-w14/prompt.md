## No git operations

You MUST NOT invoke `git`, `gh repo`, `gh api /repos`, or any shell
command that modifies git state. The orchestrator owns every git
operation: branch creation, staging, commits, pushes, rebases, merges.

If your task requires staging or committing a change, describe the
change in your return report under a `Files written:` block. Include
one line per file with its path and approximate LOC delta. The
orchestrator will stage exactly those files and author the commit.

Do not try to work around this by piping through `bash -c`, via `env`,
or through a subshell — the gate inspects the full command string.

The bypass (`ORCHESTRATOR_META=1`) exists for orchestrator-meta agents
that legitimately create branches for sub-projects. It is not
available to you. If you believe your task genuinely requires git
access, return a short explanation instead of attempting the call;
the orchestrator will decide whether to re-spawn you with elevated
permissions or handle the git step itself.


---

## Scope — files whitelist

You MUST only Edit or Write files whose path matches one of the glob
patterns in your task's `scope.files-whitelist` list. Any other path
is outside your scope.

The whitelist is the full set of files you are authorised to touch.
If your task says the whitelist is `_primitives/_rust/kei-forge/**`,
you may not create, edit, or overwrite anything at
`_primitives/_rust/kei-other/...`, at `scripts/...`, or at the
workspace root.

Reading files outside the whitelist is allowed and often necessary
(for context, cross-references, or grep). The restriction applies
only to mutating tools (Edit, Write).

If you discover that delivering your task truly requires editing a
file outside the whitelist, STOP. Do not attempt the edit. Return a
short note describing the file and the reason. The orchestrator will
either widen the scope or re-task a different agent.

On return, the verifier walks `git diff` in your worktree and
rejects any file not matching the whitelist — even if you bypassed
the live gate.


---

## Scope — files denylist

You MUST NOT Edit or Write any file whose path matches a glob in your
task's `scope.files-denylist` list. The denylist takes precedence
over any whitelist — if a path matches both, the denylist wins and
the edit is blocked.

Typical denylist entries protect high-blast-radius files: workspace
`Cargo.toml`, `Cargo.lock`, CI configuration, shared rule files,
secrets directories, and lockfile-equivalents in other ecosystems.
Changing these demands a separate review and a different role.

Reading denylisted files is always permitted and often expected
(you may need to inspect `Cargo.toml` to understand a crate's
dependencies, for example). The restriction applies only to mutating
tools.

If your task genuinely cannot be delivered without touching a
denylisted file, STOP. Do not try to work around the restriction.
Return a short note naming the file and the reason; the orchestrator
will widen the task spec, re-spawn you, or handle the edit itself.

On return, the verifier walks `git diff` in your worktree and
rejects any denylisted path that was modified.


---

## Constructor Pattern — size limits

You MUST keep every file you write or edit under 200 lines of code,
and every function under 30 lines of code. These are hard limits,
not guidelines.

The rule comes from RULE ZERO (Constructor Pattern): one file = one
class = one responsibility. Files that breach 200 LOC should be
decomposed into sibling modules. Functions that breach 30 LOC should
be split into named sub-functions, each doing one thing.

When your change pushes a file past 200 LOC or a function past 30
LOC, split it on the spot. Do not commit with `TODO: refactor later`.

Comments, blank lines, and `use` statements count toward LOC — the
verifier counts lines in the file as `wc -l` sees them.

Exceptions:
- Auto-generated code (e.g. `include!(...)` expansions) is skipped.
- Test files are checked too — if a test file grows past 200 LOC,
  split by test concern.

On return, the verifier walks every file in your worktree diff and
reports the first file or function that exceeds the limit with its
line count. No partial credit.


---

## Cargo check must be green

On return, `cargo check --workspace` MUST pass cleanly. This is
enforced in two passes:

1. **Worktree pass** — runs from inside your worktree. This is what
   you saw while iterating. It must be green before you hand off.
2. **Simulated-merge pass** — the orchestrator applies your diff onto
   a fresh branch off main and re-runs `cargo check --workspace`.
   Your change must still compile once integrated.

Both passes must succeed. Worktree-only green is a common trap: your
changes may rely on files outside the whitelist that exist in your
worktree but will not travel with the merge, or you may have shadowed
a workspace-level type. The simulated-merge pass catches that.

Before returning:
- Run `cargo check --workspace` yourself
- Wait for it to exit 0
- Include the pass in your report

If `cargo check` fails, do not return "done". Fix the errors or, if
you cannot, return with a clear description of the failure and what
you tried. Do not claim green without evidence.

The verifier captures the last lines of stderr on failure and
includes them in the rejection report.


---

## Tests must be green

On return, `cargo test -p <crate>` MUST pass for each crate listed in
your task's `verification.cargo-test-crates`. Passing is two checks:

1. Exit code 0
2. Test count greater than or equal to `verification.test-count-min`

The test-count floor exists so that "all tests pass" cannot be
achieved by deleting or `#[ignore]`-ing failing tests. If the floor
says 44, the run must show `test result: ok. 44 passed` or more.

Enforcement runs twice:
- **Worktree pass** — inside your worktree, what you iterated on.
- **Simulated-merge pass** — after your diff is applied on a fresh
  branch off main. Tests must still pass once integrated.

Before returning:
- Run the test command yourself
- Paste the real stdout from that run into your report
- Do NOT paraphrase ("all green"), do NOT summarise ("44 passing")
  without the test output block

Past agents claimed green without running — that is the failure
mode this capability exists to prevent. The verifier runs the
command itself and compares; mismatches reject the return.


---

## No dependency bumps

You MUST NOT add, remove, or upgrade dependencies. Specifically:

- Do NOT edit the `[dependencies]`, `[dev-dependencies]`,
  `[build-dependencies]`, or `[workspace.dependencies]` sections of
  any `Cargo.toml`
- Do NOT write or regenerate `Cargo.lock`
- Do NOT `cargo add`, `cargo remove`, or `cargo update`

Each new or upgraded dependency expands the supply-chain attack
surface and can trigger breaking-change cascades across the
workspace. Dependency decisions require a separate review, a
dedicated task, and an orchestrator-approved lock diff.

Editing other sections of `Cargo.toml` (e.g. `[package]`,
`[features]`, `[[bin]]`, `[lib]`, `[package.metadata.*]`) is allowed
if the file is in your whitelist and not in your denylist. The gate
inspects the specific region of the diff.

If your task genuinely requires a new dependency, STOP. Describe the
crate, version, and reason in your return. The orchestrator will
decide whether to re-spawn you with an opt-in flag or handle the
dep-bump through a separate review.

On return, the verifier diffs `Cargo.lock` against main; any change
rejects the return.


---

## Report format

Your final return message MUST contain every field listed in your
task's `output.report-fields-required`. The verifier parses your
return and checks each required key is present and non-empty.

Use one section per field. Recognised fields include:

- `Files written:` — one line per file, with path and LOC delta
  (new file / modified / deleted). Orchestrator stages exactly
  these files; missing entries = missing commits.
- `cargo-check:` — paste the exit status and last few lines of
  stderr (or "clean" if empty).
- `cargo-test:` — paste the real `test result:` line with pass
  count. Do not paraphrase.
- `loc-delta:` — per-file net lines added minus removed.
- `blockers:` — open issues you hit; empty list if none.
- `next:` — what a follow-up agent should take on, if anything.

Example skeleton:

    Files written:
    - _primitives/_rust/kei-forge/src/lib.rs (new, 120 LOC)
    - _primitives/_rust/kei-forge/tests/render.rs (new, 45 LOC)

    cargo-check: clean
    cargo-test: test result: ok. 44 passed; 0 failed; 0 ignored
    loc-delta: +165 / -0

Keep each field on its own section. The verifier is line-oriented
and will reject returns where required fields are missing.


---

## Your task's scope (resolved from task.toml)

**files-whitelist** (you MAY Edit/Write these):
- `_primitives/_rust/kei-prune/**`

**files-denylist** (you MUST NOT Edit/Write these):
- `_primitives/_rust/Cargo.toml`

**cargo check MUST pass** for: `kei-prune`
**cargo test MUST pass** for: `kei-prune`
**minimum test count:** 8

**report MUST include fields:** `files-touched`, `cargo-check`, `cargo-test`

---

Create new primitive `kei-prune` — retire unused agents / primitives based on
kei-ledger usage stats. Mirrors biological pruning: mozg забывает то что не
активировалось достаточно долго.

## Design

1. Engine-native via kei-entity-store. No new schema (just queries over
   existing ledger `agents` table).

2. Public API:
   ```rust
   pub struct PruneCandidate { id: i64, dna: String, last_used_ts: i64, age_days: i64 }
   pub fn candidates(conn: &Connection, now: i64, min_idle_days: u32) -> Result<Vec<PruneCandidate>, Error>;
   pub fn mark_retired(conn: &Connection, id: i64, now: i64) -> Result<(), Error>;
   ```

3. CLI:
   - `kei-prune list --idle-days 90` — JSON array of candidates
   - `kei-prune mark --id 5` — mark retired (sets status='retired', no delete)
   - `kei-prune stats` — summary: active / idle / retired counts

4. Pure metadata primitive — DOES NOT delete anything. Marks ledger row
   status='retired'. Downstream tooling (archive/compact) can act on marker.

## Tests (≥8)

- candidates_returns_empty_on_fresh_db
- candidates_excludes_active_rows
- candidates_returns_idle_over_threshold
- candidates_respects_min_idle_days
- mark_retired_updates_status
- mark_retired_idempotent
- stats_counts_buckets
- retired_rows_excluded_from_candidates

## Constructor Pattern
Every file ≤200 LOC, every fn ≤30 LOC. Deps: kei-entity-store path,
rusqlite workspace, clap workspace, serde workspace, thiserror workspace.

## IMPORTANT — standalone [workspace] escape hatch
Add empty [workspace] table to crate's Cargo.toml (inside your whitelist)
so cargo check/test work before orchestrator registers in workspace.
Orchestrator will remove on merge.
