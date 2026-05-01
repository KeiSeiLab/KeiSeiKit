## Fork audit — 6-point checklist

When reviewing a writer's fork diff, your return MUST address each of
the six points below. Each point is independently falsifiable from
the diff — "looks fine" without point-by-point evidence is not a
valid audit.

1. **Diff coverage.** Every file in the diff must correspond to a
   file declared in the writer's task whitelist. Orphan writes
   (outside whitelist) → FAIL. Include the exact path of any orphan
   in your verdict.

2. **Test evidence.** The writer's return MUST include a real
   `cargo-test:` (or equivalent) output line with a visible pass
   count. "Tested mentally" / "tests should pass" / any paraphrase
   → FAIL. Cross-check the test count matches new test files in
   the diff.

3. **Scope adherence.** No edits outside the writer's declared
   whitelist. Adjacent-file refactors, drive-by typo fixes, or
   unasked re-formatting → FAIL (RULE: Surgical Changes).

4. **Capability enforcement.** If the writer's role required
   capabilities (e.g. `output::report-format`), every required field
   must be present and non-empty in the return. Missing field → FAIL.

5. **Constructor-pattern LOC limits.** Any new `.rs` file must be
   ≤200 LOC; any function ≤30 LOC. Larger files → FAIL unless the
   writer has an explicit documented exception (file-level comment).

6. **Blocker disclosure.** The writer's return must contain a
   `blockers:` field — either empty (list) or an enumerated list.
   Silent dropping of known issues → FAIL. Silence = FAIL, not PASS.

For each of the six points, cite the exact path / line / excerpt
from the diff that establishes PASS or FAIL. The verdict is derived
from these six points:

- **PASS** — all 6 points evidence PASS.
- **FAIL** — any point evidence FAIL. Include remediation suggestion
  per failed point (file, line, exact edit the writer should make).
- **INCONCLUSIVE** — point N cannot be evaluated from the available
  diff (e.g. tests didn't run, CI output missing). State which point
  and what would make it evaluable.
