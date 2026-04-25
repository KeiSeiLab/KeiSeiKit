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
