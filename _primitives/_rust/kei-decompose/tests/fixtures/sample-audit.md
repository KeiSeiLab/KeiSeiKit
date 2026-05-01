# Wave 50 audit report

## Wave 50

This audit reviewed the kit for ad-hoc MD output and dead-end formats.

## Priority Matrix

| # | Severity | Finding | Fix | Complexity | Blast | Score | [E] |
|---|----------|---------|-----|------------|-------|-------|-----|
| 1 | high | research → action gap | wire kei-decompose | 2-3h | medium | 8 | E2 |
| 2 | medium | sleep follow-ups never queued | add sleep adapter | 1-2h | low | 6 | E3 |
| 3 | low | architecture decisions stale | adopt /architecture | 4h | low | 3 | E4 |

## Apply Plan

1. Land kei-decompose
2. Adopt across kit
3. Deprecate kei-decision (keep as research adapter)
