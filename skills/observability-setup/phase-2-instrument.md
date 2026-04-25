# Phase 2 — Code-side instrumentation (SDK + config diff)

Decide WHICH SDK to wire per language, emit the init-call diff, and cite the
behavioural blocks that govern field names.

## 2a — Detect languages in the target service

Run (via Bash):

```bash
{ ls "$SERVICE_DIR"/Cargo.toml 2>/dev/null && echo rust; } ; \
{ ls "$SERVICE_DIR"/go.mod     2>/dev/null && echo go; } ; \
{ ls "$SERVICE_DIR"/pyproject.toml "$SERVICE_DIR"/requirements*.txt 2>/dev/null && echo python; } ; \
{ ls "$SERVICE_DIR"/package.json 2>/dev/null && echo node; } ; \
{ ls "$SERVICE_DIR"/Package.swift 2>/dev/null && echo swift; }
```

Store de-duplicated result as `LANGUAGES` (≥1; if 0 — halt, ask user to point
to the actual service directory).

## 2b — Emit AskUserQuestion (one call)

```json
{
  "questions": [
    {
      "question": "Instrumentation style?",
      "header": "Style",
      "multiSelect": false,
      "options": [
        {"label": "Full (logs+metrics+traces)", "description": "Wire all three legs. Recommended for any service talking to another."},
        {"label": "Logs + metrics only",        "description": "Skip traces. OK for background workers without fan-out."},
        {"label": "Metrics-only",               "description": "Minimal. Only if you already have a separate log shipper."},
        {"label": "Traces-only",                "description": "Rare — only if logs+metrics already ship via external agent."}
      ]
    }
  ]
}
```

Store as `STYLE`.

## 2c — Per-language SDK table (reference, no user click)

| Lang | Logs | Metrics | Traces |
|---|---|---|---|
| rust | `tracing` + `tracing-subscriber` json fmt | `metrics` + `metrics-exporter-prometheus` OR `opentelemetry-rust` | `opentelemetry` + `opentelemetry-otlp` + `tracing-opentelemetry` |
| go | `log/slog` + `slog.NewJSONHandler` | `prometheus/client_golang` OR `go.opentelemetry.io/otel/metric` | `go.opentelemetry.io/otel` + auto-instrument |
| python | `structlog` + `JSONRenderer` | `prometheus-client` OR `opentelemetry-sdk` | `opentelemetry-sdk` + `opentelemetry-instrumentation-<lib>` |
| node | `pino` | `prom-client` OR `@opentelemetry/sdk-metrics` | `@opentelemetry/sdk-node` + auto-instrumentations |
| swift | `swift-log` + JSON backend | (server-side only) `swift-otel` | `swift-otel` |

Detailed field taxonomy and forbiddens → `_blocks/obs-structured-logs.md`,
`_blocks/obs-metrics.md`, `_blocks/obs-traces.md`. Cite these files; do NOT
duplicate their content in the generated code.

## 2d — Generate init diffs

For each language in `LANGUAGES`, emit a unified-diff patch to the target
service's entrypoint (`main.rs`, `main.go`, `app.py`, `index.ts`, `main.swift`)
that:

1. Initializes the chosen logger (JSON formatter, `level` from env, stdout).
2. If `STYLE` includes metrics: starts a `/metrics` HTTP endpoint on a dedicated
   port (default 9090 or env `METRICS_PORT`).
3. If `STYLE` includes traces: initializes OTel tracer provider with OTLP
   exporter pointing at `${OTEL_EXPORTER_OTLP_ENDPOINT:-http://localhost:4318}`.
4. Injects `trace_id` + `span_id` into every log record (integration between
   logger and tracer — language-specific; see the three reference blocks).

Do NOT edit application-level handler code in this phase — only the init
path. Handler-level spans belong to a follow-up task.

## Verify-criterion

- `LANGUAGES` non-empty.
- `STYLE` set.
- A diff exists for every language in `LANGUAGES`.
- Every diff cites the relevant `_blocks/obs-*.md` file in a comment.
- No diff contains a hard-coded token, endpoint, or service name literal —
  everything via env vars.
