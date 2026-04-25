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
