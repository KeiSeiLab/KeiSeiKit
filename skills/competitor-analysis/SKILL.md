---
name: competitor-analysis
description: Deprecated alias for /research with the competitors angle preset. Routes to /research Phase-5 verification wave focused on competitor mapping, architecture analysis, and market positioning.
argument-hint: <topic or market, e.g. "Rust web frameworks">
---

# /competitor-analysis — Deprecated Alias

> **[DEPRECATED — v0.17+]** Available as a preset: `/research --angle=competitors`.
> This standalone skill remains for backwards compatibility. `removed-after: v0.20.0 (~2026-08-01)`

## What this routes to

This skill is a Phase-5 specialization of `/research` (see
`skills/research/SKILL.md`). When invoked, it immediately hands off to
`/research` with the `competitors` angle preselected, which runs:

- `practical` teammate → Find ALL competitors (not only the obvious ones).
  Check market map, Crunchbase, ProductHunt, G2/Capterra. Who launched in
  the last 6 months? Who died? Who pivoted?
- `arch-analyst` teammate → For each competitor: tech stack, architectural
  patterns, API design, infra, open-source components. Weak points. What's
  hard to replicate (moat).
- `trends` teammate → Where is the market moving? Hype cycle. Regulation.
  Timing — too early or too late?

## Hand-off

```
=== /COMPETITOR-ANALYSIS ROUTE ===
Intake: <arguments>
Next:   /research --angle=competitors <arguments>
```

## References

- `skills/research/SKILL.md` — full 8-phase flow
- `docs/CONVERGENCE-PLAN.md` §Pre-unlock quick wins #5
