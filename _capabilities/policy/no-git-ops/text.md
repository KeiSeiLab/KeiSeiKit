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
