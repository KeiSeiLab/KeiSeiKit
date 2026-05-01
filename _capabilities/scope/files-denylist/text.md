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
