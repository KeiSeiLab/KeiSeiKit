---
name: observability-setup
description: Hub-and-spoke pipeline for installing the logs + metrics + traces triad on an existing service. Decomposes into 5 phases — scale/stack intake, code-side instrumentation, scrape+ship wiring, dashboard import, alert rules. Pure-click except for env-specific values (endpoints, tokens). Reuses `_blocks/obs-structured-logs.md`, `_blocks/obs-metrics.md`, `_blocks/obs-traces.md`, `_primitives/metrics-scrape.sh`, `_primitives/log-ship.sh`.
argument-hint: <service-or-repo-name>
---

# Observability-Setup — 5-Phase Pipeline (index)

> See `_blocks/pipeline-5phase-template.md` for the 5-phase wizard contract
> and `_blocks/rule-pure-click-contract.md` for the AskUserQuestion rule.
> Skill-specific phase tables are inline below.

You are installing observability on an existing service or repo. The user tells
you which service. You walk five phases, each with an `AskUserQuestion`
click-batch. Every durable decision lands in a named file inside the target
repo (`observability.md`, `prometheus.yml`, `otel-collector.yaml`, Grafana
dashboard JSON, Alertmanager rules).

This `SKILL.md` is the INDEX. Each phase lives in its own file and runs in
order. Never skip a phase — skipping Phase 4 gives you metrics with no
dashboards; skipping Phase 5 gives you dashboards nobody watches.

---

## Pipeline overview (5 phases + final report)

| Phase | File | Purpose | AskUserQuestion |
|---|---|---|---|
| 1 | [phase-1-intake.md](phase-1-intake.md) | Scale / stack / log target click-batch | 1× (3 questions) |
| 2 | [phase-2-instrument.md](phase-2-instrument.md) | Code-side SDK + config diff | 1× |
| 3 | [phase-3-scrape-ship.md](phase-3-scrape-ship.md) | Metrics scrape + log forward wiring | 1× |
| 4 | [phase-4-dashboards.md](phase-4-dashboards.md) | RED + USE + per-service dashboards | 1× |
| 5 | [phase-5-alerts.md](phase-5-alerts.md) | Error rate / p99 latency / saturation | 1× |

**Minimum AskUserQuestion count: 5.** (Phase 1 bundles three related questions
into one `AskUserQuestion` call with `multiSelect` per question, per native
protocol.)

---

## Variables the pipeline produces

| Name | Set in | Meaning |
|---|---|---|
| `SERVICE` | argument | Service/repo name the user invokes the skill with |
| `SCALE` | Phase 1 | `single-host` / `small-cluster` / `prod` |
| `STACK` | Phase 1 | `prom-grafana` / `otel-vendor` / `better-stack` / `custom` |
| `LOG_TARGET` | Phase 1 | `stdout-only` / `file` / `ship-loki` / `ship-datadog` / `ship-http` |
| `LANGUAGES` | Phase 2 | Subset of `{rust, go, python, node, swift}` — SDKs to wire |
| `SCRAPE_CFG` | Phase 3 | `prometheus.yml` / `otel-collector.yaml` path |
| `SHIP_CMD` | Phase 3 | `log-ship.sh` invocation for the service |
| `DASHBOARDS` | Phase 4 | List of imported / generated dashboard slugs |
| `ALERTS` | Phase 5 | List of alert rule names |

---

## Final report (emit after Phase 5)

```
=== OBSERVABILITY-SETUP REPORT ===
Service:       <SERVICE>
Scale:         <SCALE>   Stack: <STACK>   Logs: <LOG_TARGET>
Instrumented:  <LANGUAGES>
Scrape cfg:    <SCRAPE_CFG>
Ship cmd:      <SHIP_CMD>
Dashboards:    <DASHBOARDS>
Alerts:        <ALERTS>
Next action:   commit + deploy + watch first 30 min of traffic
```

---

## Rules (apply throughout)

- **Pure-click contract.** Only values that must be typed are endpoint URLs,
  API keys (via env, never a prompt), and the service name (intake argument).
- **NO HALLUCINATION (RULE 0.4).** Never invent Grafana dashboard IDs. If the
  user wants a dashboard, either generate the JSON from `_blocks/obs-metrics.md`
  naming conventions or link to the official exporter README. Dashboard IDs
  from `grafana.com/dashboards/` MUST be verified via WebFetch in-session.
- **Reuse over rewrite.** Phase 2 always cites `_blocks/obs-structured-logs.md`,
  `_blocks/obs-metrics.md`, `_blocks/obs-traces.md`. Phase 3 invokes
  `_primitives/metrics-scrape.sh` and `_primitives/log-ship.sh` — do not
  re-implement their logic inline.
- **Secrets via env (RULE 0.8).** API keys for Datadog, Better Stack, Grafana
  Cloud, etc. ALWAYS read from env (`LOG_SHIP_DD_API_KEY`, `GF_API_KEY`). Never
  write a token into any generated file.
- **Constructor Pattern.** Each phase file < 100 LOC. This index < 120 LOC.
- **Surgical Changes.** Only write to the target service repo's
  `observability.md`, `config/prometheus.yml`, `config/otel-collector.yaml`,
  `dashboards/*.json`, `alerts/*.yaml`. Do NOT touch application source beyond
  the minimum init-call required by Phase 2.

---

## References

- [phase-1-intake.md](phase-1-intake.md) · [phase-2-instrument.md](phase-2-instrument.md) · [phase-3-scrape-ship.md](phase-3-scrape-ship.md) · [phase-4-dashboards.md](phase-4-dashboards.md) · [phase-5-alerts.md](phase-5-alerts.md)
- `_blocks/obs-structured-logs.md` — JSON-lines field taxonomy (Phase 2 + Phase 3)
- `_blocks/obs-metrics.md` — RED / USE signal families + naming (Phase 4 + Phase 5)
- `_blocks/obs-traces.md` — W3C traceparent + OTLP transport (Phase 2 + Phase 3)
- `_primitives/metrics-scrape.sh` — Prometheus `/metrics` pretty-print + alert-check
- `_primitives/log-ship.sh` — stdin → stdout+forward (Loki / Datadog / custom HTTP)
- Prometheus docs [VERIFIED: prometheus.io/docs/]
- OpenTelemetry docs [VERIFIED: opentelemetry.io/docs/]
- Grafana dashboards catalog [VERIFY: grafana.com/grafana/dashboards/]
- Better Stack docs [VERIFY: betterstack.com/docs/]
