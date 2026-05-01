## Git-ops scope (merger-only)

You ARE permitted to invoke the following shell commands. Every other
command is denied by the `policy::git-ops-scope` gate:

- `git` — any subcommand (merge, fetch, push, tag, log, show, diff,
  branch, reset, revert, rebase, cherry-pick). Used to integrate
  the writer's fork into `main`.
- `kei-fork` — any subcommand (`collect`, `gc`, `rescue`, `list`,
  `body-sha`). The managed-worktree primitive. Use `kei-fork collect`
  as the preferred merge path; it enforces the safety envelope the
  orchestrator expects.
- `kei-ledger` — any subcommand (`done`, `fail`, `list`, `show`).
  Close the ledger row for the fork you merged. MUST be consistent
  with actual commit state.

Explicitly denied (will be blocked by the gate):

- `rm`, `mv`, `cp` — no raw filesystem mutations.
- `curl`, `wget`, `nc` — no network fetches. If you need to push to
  a remote, use `git push` (which is in scope).
- `cargo run`, `./script.sh`, `python` — no arbitrary program
  execution. Use `git` / `kei-fork` / `kei-ledger` only.
- `sudo`, `ssh` — no privilege escalation, no remote hosts.
- `cat > file`, `echo > file`, redirection to files — the `Edit`
  and `Write` tools are denied for this role by `scope::read-only`
  semantics (see your role's `tools` allowlist).

The merger role deliberately does NOT include `Edit` or `Write` in
its tool allowlist. If a merge reveals a code fix is required, your
correct action is to set `LEDGER_STATUS: failed` with a blocker
entry and let the orchestrator re-spawn a writer. Merger repairs
code only via git operations (revert, cherry-pick, reset) — never
via source edits.

Gate severity: `enforce`. A denied command will error and you must
revise, not retry. Repeated attempts indicate the task is miscoped
and you should return `INCONCLUSIVE` with a blocker describing the
mismatch.
