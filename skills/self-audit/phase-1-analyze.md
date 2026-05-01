# Phase 1 — Analyze

Run the `kei-memory` analyzer against the target session and aggregate
its output into a `FINDINGS` list for downstream phases.

## 1a — Resolve session

If the caller passed an argument, use it verbatim as `SESSION`. Otherwise
resolve via:

```
kei-memory analyze --last 1 --summary
```

Parse the `session=<id>` field from the first line. That is `SESSION`.

If the command fails (exit != 0) OR returns `(no sessions ingested yet)`
— return the 3 constructive paths from the skill's RULE -1 clause and
stop; do not proceed to Phase 2.

## 1b — Retrospective

```
kei-memory analyze --session <SESSION>
```

Capture the full report as `REPORT`. It includes: duration, tool-call
count, error count, top tools, top files.

## 1c — In-session patterns

```
kei-memory patterns --session <SESSION>
```

Capture each line as `{event_class, count, session_id: SESSION}` and
append to `FINDINGS` with `scope = "in-session"`.

## 1d — Cross-session patterns

```
kei-memory patterns --cross-session
```

Capture each line as `{event_class, count, session_id: null}` and
append to `FINDINGS` with `scope = "cross-session"`.

## Verify-criterion

- `SESSION` is a non-empty session id.
- `REPORT` is captured (even if empty).
- `FINDINGS` is a list (possibly empty). Empty → Phase 5 short-circuit
  (nothing to triage, nothing to route).
