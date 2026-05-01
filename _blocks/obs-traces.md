# OBSERVABILITY — Distributed traces (OpenTelemetry + W3C traceparent)

A trace is a tree of spans across services, stitched by **trace_id**. Without traces, a p99-latency investigation in a microservice topology is a guessing game. OpenTelemetry is the vendor-neutral standard; pick a backend later.

**Core data model (OTel spec 1.37+):**

| Field | Meaning |
|---|---|
| `trace_id` | 16-byte hex (32 chars) — identifies the whole trace |
| `span_id` | 8-byte hex (16 chars) — identifies one operation inside the trace |
| `parent_span_id` | span_id of the caller (empty for root) |
| `name` | Short operation name (`GET /users/:id`, `db.query`) |
| `kind` | `server` / `client` / `producer` / `consumer` / `internal` |
| `attributes` | Key-value metadata (`http.method`, `db.system`, `net.peer.name`) |
| `status` | `OK` / `ERROR` + optional message |
| `events` | Timestamped points inside the span (exceptions, annotations) |
| `start_time` / `end_time` | nanosecond epoch |

**W3C Trace Context propagation (mandatory for cross-service traces):**
- Header: `traceparent: 00-<trace_id>-<span_id>-<flags>` [VERIFIED: www.w3.org/TR/trace-context/]
- Example: `traceparent: 00-4bf92f3577b34da6a3ce929d0e0e4736-00f067aa0ba902b7-01`
- Optional `tracestate: <vendor>=<value>,...` for vendor-specific data
- Every service MUST propagate both headers unchanged on outbound requests; extract on inbound to continue the trace.

**Sampling strategies (traces are expensive at volume):**
- **Head-based** (decide at root): `ParentBased(TraceIdRatioBased(p))` with p=0.01-0.10 typical.
- **Tail-based** (decide after span completes): OTel Collector `tail_sampling` processor — keep ALL errors + slow traces + sample p=0.01 rest [VERIFIED: github.com/open-telemetry/opentelemetry-collector-contrib/tree/main/processor/tailsamplingprocessor].
- Hybrid preferred: head-sample 100% in dev, tail-sample in prod.

**Transport (OTLP — the OTel wire protocol):**
- OTLP/gRPC on port 4317 (default for app → collector, binary, efficient)
- OTLP/HTTP on port 4318 (JSON / protobuf over HTTP, browser-friendly, firewall-friendly) [VERIFIED: opentelemetry.io/docs/specs/otlp/]
- Collector is the choke point: apps ship OTLP → collector → backend (Jaeger, Tempo, Honeycomb, Datadog, Grafana Cloud).

**Backends (pick by retention budget & query needs):**
- **Jaeger** — self-host, in-memory or Cassandra/Elasticsearch storage [VERIFIED: jaegertracing.io]
- **Tempo** (Grafana) — self-host, object-storage backend, cheapest at scale, trace-id-only lookup [VERIFIED: grafana.com/docs/tempo/]
- **Vendor** — Honeycomb / Datadog / Lightstep / Grafana Cloud (pay per GB, no ops)

**Language bindings:**
- Rust: `opentelemetry` + `opentelemetry-otlp` + `tracing-opentelemetry` [VERIFIED: docs.rs/opentelemetry]
- Go: `go.opentelemetry.io/otel` + auto-instrumentation for `net/http`, `database/sql` [VERIFIED: opentelemetry.io/docs/languages/go/]
- Python: `opentelemetry-sdk` + `opentelemetry-instrumentation-<lib>` auto-loaders
- Node/TS: `@opentelemetry/sdk-node` + `@opentelemetry/auto-instrumentations-node`

**Log correlation:** every log entry MUST include `trace_id` + `span_id` fields (see `obs-structured-logs`). One click in Grafana / Tempo from trace → logs.

**Forbidden:** rolling your own header format instead of W3C `traceparent` (breaks every off-the-shelf collector); sampling 100% in prod on >1k RPS service (cost + backend OOM); omitting `kind` on spans (breaks service-graph view); propagating `tracestate` across trust boundaries without validation (can be used for tracking).
