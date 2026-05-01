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
