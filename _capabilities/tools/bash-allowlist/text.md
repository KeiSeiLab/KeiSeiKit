## Bash — allowlist gate

You MAY use `Bash`, but only for commands that match this allowlist.
Anything else is blocked at the gate.

Default-allowed command prefixes:
- `cargo ...` — build, check, test, fmt, clippy, run
- `rustc ...` — direct compilation probes
- `rustup ...` — toolchain inspection
- `mkdir ...` — create directories inside the worktree
- `ls ...` — directory listing
- `pwd` — print working directory
- `rm -rf /tmp/...` — cleanup under `/tmp` only

Everything else is denied, including (non-exhaustive): `git`,
`gh`, `curl`, `wget`, `npm`, `pip`, `python`, `node`, `bash -c`,
`sudo`, `sh`, `env VAR=...`, `docker`, `kubectl`, `ssh`, `scp`,
process-tree manipulation, and compound commands that chain an
allowed prefix with a denied one via `;`, `&&`, `||`, or pipes.

The gate inspects the full command string. Do not try to hide a
denied call behind a heredoc, variable expansion, or `xargs`. If
you need a tool that is not on the allowlist, STOP and describe
the need in your return — the orchestrator will either widen the
role or handle the step directly.

Prefer dedicated tools over Bash whenever possible: `Read`/`Write`
for files, `Glob`/`Grep` for search.
