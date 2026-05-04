---
title: phase-4-dashboards
path: observability-setup/phase-4-dashboards.md
dna_hash: sha256:052590293bdbbab7
language: markdown
size_loc: 88
generated: by-keidocs
---

# observability-setup/phase-4-dashboards.md

## Public API

- `Phase 4 — Dashboards (RED + USE + per-service)` — Every metric without a dashboard is dead weight. Two mandatory dashboards,
- `4a — Emit AskUserQuestion (one call)` — ```json
- `4b — RED dashboard (mandatory, write regardless of `DASH_PATH` choice)` — Write `dashboards/red-$SERVICE.json` with three panels:
- `4c — USE dashboard (mandatory, write regardless)` — Write `dashboards/use-node.json` with four rows (all backed by `node_exporter`
- `4d — Per-service dashboard (optional — only if `DASH_PATH == "Generate from metric names"`)` — Run `_primitives/metrics-scrape.sh --format json` against the service,
- `4e — If `DASH_PATH == "Import from grafana.com"`` — **NO HALLUCINATION.** Do NOT cite any dashboard ID you have not WebFetched
- `4f — If `DASH_PATH == "Vendor-native"`` — Emit `dashboards/README.md` noting which vendor auto-generates and pointing
- `Verify-criterion` — - RED + USE JSON files exist in `dashboards/` (mandatory).

## Related

- parent: `observability-setup`

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
