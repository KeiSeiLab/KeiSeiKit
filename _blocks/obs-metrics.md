# OBSERVABILITY — Metrics (Prometheus + OTel + RED/USE)

Metrics are numeric time series scraped or pushed on a fixed cadence (10-60 s). Two signal families to cover:

**RED (request-driven services — APIs, workers):**
- **R**ate — requests per second
- **E**rrors — error rate (5xx / failed jobs)
- **D**uration — latency distribution (p50 / p95 / p99)

**USE (resources — CPU, memory, disk, network):**
- **U**tilization — % busy
- **S**aturation — queue depth / wait time
- **E**rrors — hardware / syscall errors

Source: Google SRE Book "Four Golden Signals" [VERIFIED: sre.google/sre-book/monitoring-distributed-systems/] + Brendan Gregg USE [VERIFIED: brendangregg.com/usemethod.html] + Tom Wilkie RED [VERIFIED: thenewstack.io/monitoring-microservices-red-method/].

**Metric types (Prometheus model, inherited by OTel):**

| Type | Use for | Example |
|---|---|---|
| Counter | Monotonic cumulative count | `http_requests_total{route, status}` |
| Gauge | Instantaneous value (up/down) | `queue_depth`, `memory_bytes` |
| Histogram | Latency / size distribution with buckets | `http_request_duration_seconds_bucket` |
| Summary | Client-side quantiles (prefer histogram — can aggregate) | — avoid unless Prom-server-side quantile is impossible |

**Naming convention (Prometheus exposition, OTel convention 1.27+):**
- Suffix units: `_seconds`, `_bytes`, `_total` for counters [VERIFIED: prometheus.io/docs/practices/naming/]
- Lowercase snake_case, dots forbidden in Prom names (OTel dots become underscores on export)
- Cardinality budget: < 10 labels per metric, < 100 values per label — runaway cardinality kills Prometheus [VERIFIED: prometheus.io/docs/practices/naming/#labels]

**Stack (self-host, single-host or small cluster):**
- `node_exporter` on every host (port 9100) — USE metrics for CPU/mem/disk/net [VERIFIED: github.com/prometheus/node_exporter]
- App exposes `/metrics` on app port (Prom client library per language)
- Prometheus scrapes every 15 s, retention 15 d local (longer → remote_write to Mimir / Thanos / vendor)
- Grafana dashboards connect to Prometheus datasource

**OpenTelemetry path (vendor-agnostic, OTLP collector in front):**
- App uses OTel SDK → OTLP/gRPC (port 4317) or OTLP/HTTP (port 4318) [VERIFIED: opentelemetry.io/docs/specs/otlp/]
- OTel Collector receives OTLP, exports to Prometheus remote_write / vendor (Honeycomb, Datadog, Grafana Cloud)
- Same collector handles logs + traces (see `obs-traces`) → single deploy unit

**Language bindings:**
- Rust: `metrics` + `metrics-exporter-prometheus` OR `opentelemetry-rust` [VERIFIED: docs.rs/opentelemetry]
- Go: `prometheus/client_golang` (native Prom) OR `go.opentelemetry.io/otel/metric`
- Python: `prometheus-client` OR `opentelemetry-sdk` with `opentelemetry-exporter-otlp`
- Node/TS: `prom-client` OR `@opentelemetry/sdk-metrics`

**Forbidden:** high-cardinality labels (`user_id`, `trace_id`, `timestamp` — never a label); per-request gauges (use histograms); Summary where Histogram works (Summaries don't aggregate across instances); pushing metrics from a long-running service (use `/metrics` scrape; Pushgateway is for short-lived jobs ONLY per Prom docs); renaming metrics without a deprecation window (breaks dashboards silently).
