# Phase 4 — Dashboards (RED + USE + per-service)

Every metric without a dashboard is dead weight. Two mandatory dashboards,
one optional per-service dashboard.

## 4a — Emit AskUserQuestion (one call)

```json
{
  "questions": [
    {
      "question": "Dashboard provisioning path?",
      "header": "Dashboards",
      "multiSelect": false,
      "options": [
        {"label": "Generate from metric names", "description": "Author JSON from _blocks/obs-metrics.md naming + RED/USE rules. Full control, no external deps."},
        {"label": "Import from grafana.com",    "description": "Import a community dashboard by ID. Requires WebFetch to verify the ID lives + matches our metric names."},
        {"label": "Vendor-native",              "description": "Datadog / Honeycomb / Better Stack auto-generate from instrumented metrics. No JSON files in repo."},
        {"label": "Skip (placeholder)",         "description": "Emit dashboards/TODO.md only — revisit after launch. NOT recommended for prod."}
      ]
    }
  ]
}
```

Store as `DASH_PATH`.

## 4b — RED dashboard (mandatory, write regardless of `DASH_PATH` choice)

Write `dashboards/red-$SERVICE.json` with three panels:

1. **Rate** — `sum by(route)(rate(http_requests_total{service="$SERVICE"}[1m]))`
2. **Errors** — `sum by(route)(rate(http_requests_total{service="$SERVICE",status=~"5.."}[1m]))` plotted alongside rate → visual error-fraction.
3. **Duration** — `histogram_quantile(0.99, sum by(le,route)(rate(http_request_duration_seconds_bucket{service="$SERVICE"}[5m])))` for p50, p95, p99.

Variables: `$service`, `$route`, `$interval` (1m / 5m / 15m).

Reference `_blocks/obs-metrics.md` for naming convention (`_total`, `_seconds`,
`_bucket`, `le` label) — do NOT invent alternate names.

## 4c — USE dashboard (mandatory, write regardless)

Write `dashboards/use-node.json` with four rows (all backed by `node_exporter`
metrics — confirmed names from [VERIFIED: github.com/prometheus/node_exporter/tree/master/docs]):

1. **CPU utilization** — `100 - avg by(instance)(rate(node_cpu_seconds_total{mode="idle"}[5m])) * 100`
2. **Memory utilization** — `(1 - node_memory_MemAvailable_bytes / node_memory_MemTotal_bytes) * 100`
3. **Disk saturation** — `rate(node_disk_io_time_weighted_seconds_total[5m])` per device
4. **Network errors** — `rate(node_network_receive_errs_total[5m])` + `rate(node_network_transmit_errs_total[5m])`

## 4d — Per-service dashboard (optional — only if `DASH_PATH == "Generate from metric names"`)

Run `_primitives/metrics-scrape.sh --format json` against the service,
extract the distinct metric names, and emit one panel per metric group (group
= metric name minus `_bucket` / `_sum` / `_count` suffix). This is
mechanical — no creativity, no invented names.

## 4e — If `DASH_PATH == "Import from grafana.com"`

**NO HALLUCINATION.** Do NOT cite any dashboard ID you have not WebFetched
this session. Walk the user through:

1. Ask user for the Grafana.com dashboard URL they want (they find it; we
   verify).
2. `WebFetch https://grafana.com/grafana/dashboards/<id>/` and confirm:
   - dashboard exists (non-404)
   - datasource type matches their Prom install
   - referenced metric names appear in our scrape output (run
     `metrics-scrape.sh --format json`)
3. Save the verified URL and a SHA256 of the JSON payload in
   `dashboards/imports.md` — audit trail for re-verification.

If the metric names don't match — HALT. Do NOT edit the dashboard JSON to
"translate" names; instead, ask user to either pick a different dashboard or
rename metrics at source (Phase 2 rerun).

## 4f — If `DASH_PATH == "Vendor-native"`

Emit `dashboards/README.md` noting which vendor auto-generates and pointing
at the vendor's documentation URL (`[VERIFY: <url>]` — real URL only). Do
NOT generate JSON in this case.

## Verify-criterion

- RED + USE JSON files exist in `dashboards/` (mandatory).
- If `DASH_PATH == "Import from grafana.com"`: every imported dashboard has
  a verified URL + SHA256 in `dashboards/imports.md`. Zero fabricated IDs.
- `DASHBOARDS` list populated for the final report.
