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
