# OBSERVABILITY — Structured logs (JSON-lines)

Structured logging is the cheapest leg of the observability triad. One JSON object per line, stable field names, machine-parseable by any log shipper (Loki, Vector, Fluent Bit, Datadog Agent, CloudWatch). Unstructured `printf` / `logger.info("user %s did %s", u, a)` wastes the capability.

**Field taxonomy (stable across services — single source of truth):**

| Field | Type | Meaning |
|---|---|---|
| `ts` | RFC3339 string | Timestamp with timezone (`2026-04-21T12:00:00.123Z`) |
| `level` | enum | `debug` / `info` / `warn` / `error` / `fatal` |
| `msg` | string | Short human-readable summary (no interpolated values — they go in their own fields) |
| `service` | string | Emitting service name (e.g. `api-gateway`) |
| `env` | enum | `local` / `dev` / `staging` / `prod` |
| `trace_id` | hex32 | W3C traceparent trace-id (links log to trace — see `obs-traces`) |
| `span_id` | hex16 | W3C span-id of the current span |
| `request_id` | string | Per-request correlation ID (propagate via `X-Request-ID`) |
| `user_id` | string | Actor (redact PII — hash or internal ID, never email) |
| `err` | object | `{type, message, stack}` when `level >= error` |

**Emission rules:**
- Always write to **stdout** (one JSON per line). Let the container runtime / systemd capture it. Never open a log file from the app — shippers have file-locking races.
- NEVER mix plain text and JSON on stdout (breaks parsers). Config libraries must emit JSON in all environments, local included.
- `msg` stays constant per log site (e.g. `"db query failed"`). Dynamic values (query, duration_ms, table) go in their own fields. This is what makes logs queryable.
- On exception: capture `err.stack` as a single string with `\n` separators (don't split across lines).

**Language bindings (pick ONE per service, never two):**
- Rust: `tracing` + `tracing-subscriber` with `.json()` formatter [VERIFIED: docs.rs/tracing-subscriber]
- Go: `log/slog` stdlib with `slog.NewJSONHandler` (Go 1.21+) [VERIFIED: pkg.go.dev/log/slog]
- Python: `structlog` with `JSONRenderer` [VERIFIED: www.structlog.org]
- Node/TS: `pino` (`pino({ level, formatters })`) [VERIFIED: getpino.io]
- Swift/iOS: server-side only — `swift-log` with `swift-log-formatter-json` backend

**Shipping:**
- Container / k8s: stdout → Fluent Bit / Vector → Loki or vendor.
- Bare metal: systemd journald → `journalctl -o json` → Vector.
- Dev: stdout is enough; no shipper.

**Forbidden:** string interpolation in `msg` (`f"user {id}"` — id goes in its own field); writing secrets to logs (token/password/cookie values); `print()` debug leftovers in committed code; changing `level` semantics per service (keep the 5 levels stable kit-wide); logging full request/response bodies without redaction.
