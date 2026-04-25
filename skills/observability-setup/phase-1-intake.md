# Phase 1 — Intake (scale / stack / log target)

Three orthogonal questions bundled into ONE `AskUserQuestion` call. Every
subsequent phase branches on the answers.

## 1a — Emit AskUserQuestion (one call, three questions)

```json
{
  "questions": [
    {
      "question": "Deployment scale?",
      "header": "Scale",
      "multiSelect": false,
      "options": [
        {"label": "Single-host",    "description": "One VM / container. Prom + Grafana + app on one box. < 100 rps. Retention 7-15 d."},
        {"label": "Small-cluster",  "description": "2-10 nodes. Central Prom, node_exporter everywhere. OTel Collector optional."},
        {"label": "Prod",           "description": ">10 nodes OR regulated. Remote-write storage, HA Prom, vendor or Mimir/Tempo."}
      ]
    },
    {
      "question": "Target stack?",
      "header": "Stack",
      "multiSelect": false,
      "options": [
        {"label": "Prom + Grafana",     "description": "Self-host. Prometheus + node_exporter + Grafana + optional Loki + optional Tempo."},
        {"label": "OTel + vendor",      "description": "OTel Collector in front of Honeycomb / Datadog / Grafana Cloud / Lightstep."},
        {"label": "Better Stack",       "description": "Logs + Uptime + Heartbeat SaaS. Lowest ops, USD-priced per GB."},
        {"label": "Custom",             "description": "CloudWatch / GCP Ops / Elastic / Splunk — describe in followup."}
      ]
    },
    {
      "question": "Log destination?",
      "header": "Logs",
      "multiSelect": false,
      "options": [
        {"label": "stdout-only",        "description": "Dev / single-host. Container runtime captures, no shipper."},
        {"label": "File + rotate",      "description": "journald or logrotate on disk. Read via SSH when debugging."},
        {"label": "Ship to Loki",       "description": "Vector / Fluent Bit → Loki (self-host) or Grafana Cloud Logs."},
        {"label": "Ship to Datadog",    "description": "Datadog Agent or direct HTTP intake via log-ship.sh."},
        {"label": "Ship to custom HTTP","description": "Generic JSON POST via log-ship.sh --target http."}
      ]
    }
  ]
}
```

## 1b — Store answers

- First answer → `SCALE` ∈ {`single-host`, `small-cluster`, `prod`}
- Second answer → `STACK` ∈ {`prom-grafana`, `otel-vendor`, `better-stack`, `custom`}
- Third answer → `LOG_TARGET` ∈ {`stdout-only`, `file`, `ship-loki`, `ship-datadog`, `ship-http`}

## 1c — Immediate sanity checks (emit as plain message, no clicks)

- If `SCALE == single-host` AND `STACK == otel-vendor`: warn — vendor OTel
  Collector is overkill for one host; suggest Prom+Grafana OR direct vendor
  SDK. Ask user to confirm or switch.
- If `STACK == better-stack` AND `LOG_TARGET == ship-loki`: warn — Better
  Stack is its own log backend, shipping to Loki duplicates cost. Ask user
  to confirm or switch.
- If `SCALE == prod` AND `LOG_TARGET == stdout-only`: warn — prod without
  shipping loses logs on node death. Ask user to confirm or switch.

Sanity-check confirmations are free-text "ok" / "switch to X" — no extra
AskUserQuestion needed (the user's next message resolves them).

## Verify-criterion

- `SCALE`, `STACK`, `LOG_TARGET` all set to one of their enumerated values.
- Any sanity-check warnings either confirmed or resolved by an answer-revise.
- If any variable is unset — re-ask the failing one only; do not fall through.
