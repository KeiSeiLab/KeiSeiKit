# Phase 5 — Alert rules (error rate / p99 latency / saturation)

Alerts are the only leg that wakes a human. Keep the set small, sharp, and
actionable. Four starter rules; expand only after running a real incident.

## 5a — Emit AskUserQuestion (one call)

```json
{
  "questions": [
    {
      "question": "Alert delivery channel?",
      "header": "Channel",
      "multiSelect": false,
      "options": [
        {"label": "Alertmanager → email",  "description": "Self-host Prometheus Alertmanager, SMTP relay. Simplest, free."},
        {"label": "Alertmanager → webhook","description": "Alertmanager POSTs to our own HTTP endpoint (Telegram bot, Slack, custom)."},
        {"label": "Better Stack Uptime",   "description": "Push-based; Better Stack runs the schedule + escalation. Paid."},
        {"label": "PagerDuty",             "description": "Enterprise escalation + on-call rotation. Paid, SRE-grade."},
        {"label": "Custom webhook (other)","description": "Vendor-specific (Opsgenie, VictorOps, Discord). User supplies URL."}
      ]
    }
  ]
}
```

Store as `ALERT_CHANNEL`.

## 5b — Write alert rules (`alerts/$SERVICE.yaml`)

Four starter rules, all metric names drawn from `_blocks/obs-metrics.md`
convention — no inventions. Reference Prometheus alerting-rules spec
[VERIFIED: prometheus.io/docs/prometheus/latest/configuration/alerting_rules/].

```yaml
groups:
  - name: $SERVICE-red
    interval: 30s
    rules:
      - alert: HighErrorRate
        expr: |
          (
            sum by(service)(rate(http_requests_total{service="$SERVICE",status=~"5.."}[5m]))
            /
            sum by(service)(rate(http_requests_total{service="$SERVICE"}[5m]))
          ) > 0.05
        for: 5m
        labels: { severity: page, team: "$TEAM" }
        annotations:
          summary: "$SERVICE: 5xx > 5% for 5 min"
          runbook: "docs/runbooks/$SERVICE.md#high-error-rate"

      - alert: HighLatencyP99
        expr: |
          histogram_quantile(0.99,
            sum by(le,service)(rate(http_request_duration_seconds_bucket{service="$SERVICE"}[5m]))
          ) > ${P99_BUDGET_SEC:-1.0}
        for: 10m
        labels: { severity: page, team: "$TEAM" }
        annotations:
          summary: "$SERVICE: p99 > ${P99_BUDGET_SEC:-1.0}s for 10 min"
          runbook: "docs/runbooks/$SERVICE.md#high-latency"

  - name: node-use
    interval: 30s
    rules:
      - alert: CpuSaturated
        expr: 100 - avg by(instance)(rate(node_cpu_seconds_total{mode="idle"}[5m])) * 100 > 90
        for: 15m
        labels: { severity: ticket }
        annotations:
          summary: "{{ $labels.instance }}: CPU > 90% for 15 min"

      - alert: DiskFull
        expr: (node_filesystem_avail_bytes / node_filesystem_size_bytes) < 0.10
        for: 5m
        labels: { severity: page }
        annotations:
          summary: "{{ $labels.instance }}:{{ $labels.mountpoint }} < 10% free"
```

Budget knobs (`P99_BUDGET_SEC`, CPU %, disk %) are ENV-overridable defaults;
tune per-service after one week of baseline data.

## 5c — Alertmanager / channel wiring

**If `ALERT_CHANNEL == "Alertmanager → email"`** — write `alerts/alertmanager.yml`:

```yaml
route: { group_by: ['alertname', 'service'], receiver: "mail" }
receivers:
  - name: mail
    email_configs:
      - to: "${ALERT_EMAIL}"
        from: "${ALERT_FROM_EMAIL}"
        smarthost: "${SMTP_HOST}:${SMTP_PORT:-587}"
        auth_username: "${SMTP_USER}"
        auth_password_file: "/run/secrets/smtp_password"   # never inline
```

**If `ALERT_CHANNEL == "Alertmanager → webhook"`** — use `webhook_configs`
pointing at `$ALERT_WEBHOOK_URL` (env-supplied).

**If `ALERT_CHANNEL == "Better Stack Uptime"`** — note URL in
`alerts/README.md`; Better Stack config lives in their UI. Pair each Prom
alert with a Better Stack Heartbeat for dead-man's-switch coverage
[VERIFY: betterstack.com/docs/uptime/heartbeats/].

**If `ALERT_CHANNEL == "PagerDuty"`** — Alertmanager `pagerduty_configs` with
`routing_key_file` (never `routing_key:` inline — RULE 0.8).

**If `ALERT_CHANNEL == "Custom webhook"`** — ask user for endpoint URL and
whether auth is Bearer / HMAC / custom header; wire via
`webhook_configs.http_config`.

## 5d — Dead-man's-switch (all channels)

Add a "YouAreAlive" alert that fires when Prom fails to scrape the service
for 5 min. Pair with a heartbeat external monitor (Better Stack, UptimeRobot,
or a cron that checks Alertmanager). Without it, the alerting system can
fail silently.

```yaml
- alert: ScrapeDown
  expr: up{job="$SERVICE"} == 0
  for: 5m
  labels: { severity: page }
  annotations: { summary: "$SERVICE: Prometheus cannot scrape for 5 min" }
```

## 5e — Runbook stub (mandatory)

Write `docs/runbooks/$SERVICE.md` with one section per alert name, each
containing: symptom, first-check, rollback, escalation. Empty runbook links
in annotations are a documented anti-pattern — fill the stub now with at
least "TODO after first incident".

## Verify-criterion

- `alerts/$SERVICE.yaml` contains the four starter rules + `ScrapeDown`.
- Delivery channel config written (or referenced in `alerts/README.md` for
  vendor-managed channels).
- `docs/runbooks/$SERVICE.md` stub exists with one section per alert.
- `ALERTS` list populated for the final report.
- No credential literal in any generated file — env / file-refs only.
