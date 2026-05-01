# Phase 3 — Scrape + ship wiring

Produce two concrete config artefacts in the target repo:
- `config/prometheus.yml` (or `config/otel-collector.yaml` if `STACK == otel-vendor`)
- `config/log-ship.env` — env-var bundle for `_primitives/log-ship.sh`

## 3a — Emit AskUserQuestion (one call)

```json
{
  "questions": [
    {
      "question": "Scrape / collect topology?",
      "header": "Topology",
      "multiSelect": false,
      "options": [
        {"label": "Prometheus pulls /metrics",      "description": "Prom-native. App exposes 9090. Standard for prom-grafana."},
        {"label": "OTel Collector sidecar",         "description": "Per-host collector. App → collector → backend. Uniform for logs+metrics+traces."},
        {"label": "OTel Collector central gateway", "description": "One collector pool for the cluster. HA, scales, single ingress point."},
        {"label": "Vendor agent (Datadog / BS)",    "description": "Vendor-supplied agent does discovery + shipping. Lowest ops."}
      ]
    }
  ]
}
```

Store as `TOPOLOGY`.

## 3b — Generate scrape config

**If `TOPOLOGY == "Prometheus pulls /metrics"`** — write `config/prometheus.yml`:

```yaml
global:
  scrape_interval: 15s
  evaluation_interval: 15s
scrape_configs:
  - job_name: "$SERVICE"
    metrics_path: /metrics
    static_configs:
      - targets: ["${SERVICE_HOST:-localhost}:${METRICS_PORT:-9090}"]
  - job_name: "node"
    static_configs:
      - targets: ["${NODE_HOST:-localhost}:9100"]
```

Reference: `_blocks/obs-metrics.md` for label cardinality budget, naming
conventions. Reference Prometheus config spec [VERIFIED: prometheus.io/docs/prometheus/latest/configuration/configuration/].

**If `TOPOLOGY` is an OTel variant** — write `config/otel-collector.yaml`:

```yaml
receivers:
  otlp:
    protocols:
      grpc: { endpoint: 0.0.0.0:4317 }
      http: { endpoint: 0.0.0.0:4318 }
processors:
  batch: {}
  memory_limiter: { check_interval: 1s, limit_mib: 512 }
exporters:
  prometheusremotewrite:
    endpoint: ${PROM_REMOTE_WRITE_URL}
  otlphttp/traces:
    endpoint: ${TRACES_BACKEND_URL}
service:
  pipelines:
    metrics: { receivers: [otlp], processors: [memory_limiter, batch], exporters: [prometheusremotewrite] }
    traces:  { receivers: [otlp], processors: [memory_limiter, batch], exporters: [otlphttp/traces] }
    logs:    { receivers: [otlp], processors: [memory_limiter, batch], exporters: [otlphttp/traces] }
```

Reference OTel Collector spec [VERIFIED: opentelemetry.io/docs/collector/configuration/].

**If `TOPOLOGY == "Vendor agent"`** — output the vendor install snippet
(Datadog Agent, Better Stack Vector config, etc.) and skip to 3c.

## 3c — Generate log-ship invocation

Build `config/log-ship.env` referencing `_primitives/log-ship.sh` with fields
from Phase 1's `LOG_TARGET`:

```sh
# config/log-ship.env — env bundle for _primitives/log-ship.sh
# Source before piping app stdout:
#   set -a && . config/log-ship.env && set +a
#   ./app 2>&1 | ~/.claude/agents/_primitives/log-ship.sh --target $LOG_SHIP_TARGET --endpoint "$LOG_SHIP_ENDPOINT" --label "job=$SERVICE"

LOG_SHIP_TARGET="${LOG_SHIP_TARGET:-stdout}"      # stdout | loki | datadog | http
LOG_SHIP_ENDPOINT="${LOG_SHIP_ENDPOINT:-}"        # e.g. http://loki:3100/loki/api/v1/push
# LOG_SHIP_DD_API_KEY=...   # ← put in ~/.claude/secrets/.env or service .env — NEVER in git
# LOG_SHIP_BEARER=...       # generic HTTP target bearer — same rule
```

Map Phase 1's `LOG_TARGET` → `LOG_SHIP_TARGET`:
- `stdout-only` → `stdout` (no endpoint)
- `file` → `stdout` (container runtime captures; skip shipping)
- `ship-loki` → `loki` + endpoint
- `ship-datadog` → `datadog` + endpoint + `LOG_SHIP_DD_API_KEY` via env
- `ship-http` → `http` + endpoint + optional `LOG_SHIP_BEARER`

## 3d — Verify scrape end-to-end

Before finishing the phase, invoke `_primitives/metrics-scrape.sh` against
the freshly instrumented app:

```sh
~/.claude/agents/_primitives/metrics-scrape.sh \
  "http://${SERVICE_HOST:-localhost}:${METRICS_PORT:-9090}/metrics" --format table
```

If the output is empty or the curl fails — HALT, report to user (likely Phase 2
init-call mis-wired). Do NOT proceed to Phase 4 with a silent scraper.

## Verify-criterion

- `config/prometheus.yml` OR `config/otel-collector.yaml` written.
- `config/log-ship.env` written (with `# NEVER in git` comment next to any
  secret-var placeholder — RULE 0.8).
- `metrics-scrape.sh` dry-run returns > 0 lines.
- `TOPOLOGY` stored for Phase 5's alert-rule scope.
