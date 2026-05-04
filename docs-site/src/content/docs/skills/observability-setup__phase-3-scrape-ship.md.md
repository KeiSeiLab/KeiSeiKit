---
title: phase-3-scrape-ship
path: observability-setup/phase-3-scrape-ship.md
dna_hash: sha256:bbe09d405a69a5e0
language: markdown
size_loc: 121
generated: by-keidocs
---

# observability-setup/phase-3-scrape-ship.md

## Public API

- `Phase 3 — Scrape + ship wiring` — Produce two concrete config artefacts in the target repo:
- `3a — Emit AskUserQuestion (one call)` — ```json
- `3b — Generate scrape config` — **If `TOPOLOGY == "Prometheus pulls /metrics"`** — write `config/prometheus.yml`:
- `3c — Generate log-ship invocation` — Build `config/log-ship.env` referencing `_primitives/log-ship.sh` with fields
- `config/log-ship.env — env bundle for _primitives/log-ship.sh` — 
- `Source before piping app stdout:` — 
- `set -a && . config/log-ship.env && set +a` — 
- `./app 2>&1 | ~/.claude/agents/_primitives/log-ship.sh --target $LOG_SHIP_TARGET --endpoint "$LOG_SHIP_ENDPOINT" --label "job=$SERVICE"` — LOG_SHIP_TARGET="${LOG_SHIP_TARGET:-stdout}"      # stdout | loki | datadog | http
- `LOG_SHIP_DD_API_KEY=...   # ← put in ~/.claude/secrets/.env or service .env — NEVER in git` — 
- `LOG_SHIP_BEARER=...       # generic HTTP target bearer — same rule` — ```
- `3d — Verify scrape end-to-end` — Before finishing the phase, invoke `_primitives/metrics-scrape.sh` against
- `Verify-criterion` — - `config/prometheus.yml` OR `config/otel-collector.yaml` written.

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
