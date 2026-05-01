## Severity grade on findings

Every finding in your return MUST carry a severity grade:
`[HIGH]`, `[MEDIUM]`, or `[LOW]`. Write the grade as the first
token of the finding's header.

Grading rubric:
- **[HIGH]** — auth, crypto, memory safety, data loss, IP leak,
  network protocol flaw, unsound FFI, secret in source, or any
  issue that could compromise a production deploy.
- **[MEDIUM]** — input validation, error handling, resource
  exhaustion, config drift, missing test coverage on a critical
  path, performance regression with measurable impact.
- **[LOW]** — docs inaccuracy, formatting, non-idiomatic code,
  comment drift, minor style, opportunistic refactor.

Example:

    **[HIGH]** Unbounded allocation in request parser
    - File: crates/api/src/parse.rs:47
    - Class: resource exhaustion
    - Scenario: attacker sends 2GB body, process OOMs
    - Fix: cap read at 16 MiB via `take(...)`

    **[LOW]** Typo in module docstring
    - File: crates/api/src/lib.rs:3

The verifier parses your return, locates every `## ` section
containing the word "Finding" (case-insensitive) or matching the
format above, and rejects the return if any finding lacks a
`[HIGH|MEDIUM|LOW]` token.

Empty finding lists are fine — state "No findings" and no grade
is required.
