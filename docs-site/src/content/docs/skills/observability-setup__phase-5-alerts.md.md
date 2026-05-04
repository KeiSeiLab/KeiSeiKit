---
title: phase-5-alerts
path: observability-setup/phase-5-alerts.md
dna_hash: sha256:2ca6bd190f9e597f
language: markdown
size_loc: 145
generated: by-keidocs
---

# observability-setup/phase-5-alerts.md

## Public API

- `Phase 5 — Alert rules (error rate / p99 latency / saturation)` — Alerts are the only leg that wakes a human. Keep the set small, sharp, and
- `5a — Emit AskUserQuestion (one call)` — ```json
- `5b — Write alert rules (`alerts/$SERVICE.yaml`)` — Four starter rules, all metric names drawn from `_blocks/obs-metrics.md`
- `5c — Alertmanager / channel wiring` — **If `ALERT_CHANNEL == "Alertmanager → email"`** — write `alerts/alertmanager.yml`:
- `5d — Dead-man's-switch (all channels)` — Add a "YouAreAlive" alert that fires when Prom fails to scrape the service
- `5e — Runbook stub (mandatory)` — Write `docs/runbooks/$SERVICE.md` with one section per alert name, each
- `Verify-criterion` — - `alerts/$SERVICE.yaml` contains the four starter rules + `ScrapeDown`.

## Related

- parent: `observability-setup`

## Discussion

<script src="https://giscus.app/client.js"
        data-repo="KeiSei84/KeiSeiKit-1.0"
        data-repo-id="PLACEHOLDER_REPO_ID"
        data-category="wiki-comments"
        data-category-id="PLACEHOLDER_CATEGORY_ID"
        data-mapping="pathname"
        data-strict="0"
        data-reactions-enabled="1"
        data-emit-metadata="0"
        data-input-position="bottom"
        data-theme="preferred_color_scheme"
        data-lang="en"
        data-loading="lazy"
        crossorigin="anonymous"
        async></script>
