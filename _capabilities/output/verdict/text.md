## Verdict output format

Your return report MUST contain a single `verdict:` line, followed by
a `findings:` block. The verdict value MUST be exactly one of:

- `PASS` — every audited point passes. No blocking issues. Merger
  may proceed to integrate the fork into main.
- `FAIL` — at least one audited point fails. Merger MUST NOT merge.
  Each failure MUST have a remediation entry under `findings:`.
- `INCONCLUSIVE` — a required audit point could not be evaluated
  (e.g. tests failed to run, diff unavailable). Merger MUST NOT
  merge; orchestrator re-spawns the writer or the auditor.

Skeleton:

    verdict: PASS
    findings: none
    body-sha: <sha256 of the fork diff, 64 hex chars>
    audited-agent: <writer agent-id being reviewed>

    verdict: FAIL
    findings:
      - point: 2
        file: _primitives/_rust/kei-spawn/src/pipeline.rs
        evidence: "No `cargo-test:` line in writer's return"
        remediation: "Re-run `cargo test -p kei-spawn` and paste stdout"
      - point: 5
        file: _primitives/_rust/kei-spawn/src/pipeline.rs
        evidence: "File is 243 LOC (limit 200)"
        remediation: "Split pipeline.rs into pipeline.rs + pipeline_io.rs"
    body-sha: <sha256>
    audited-agent: <writer agent-id>

Rules:

- `verdict:` must be on its own line with no surrounding prose.
- `findings:` is a YAML-style block even for PASS (use `findings: none`).
- `body-sha:` is the SHA-256 of the concatenated fork diff as reported
  by `kei-fork body-sha <agent-id>` (or equivalent).
- `audited-agent:` is the agent-id of the writer under review — not
  your own id.

The merger role reads these four fields mechanically. Missing field
or malformed verdict value → merger refuses to proceed, orchestrator
re-spawns.
