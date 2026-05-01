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
