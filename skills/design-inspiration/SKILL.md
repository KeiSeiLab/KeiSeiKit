---
name: design-inspiration
description: Deprecated alias for /research with the design-refs angle preset. Routes to /research focused on design patterns, visual archetype references, and award-winning site teardowns.
argument-hint: <design angle or site archetype, e.g. "SaaS landing dark-tech">
---

# /design-inspiration — Deprecated Alias

> **[DEPRECATED — v0.17+]** Available as a preset: `/research --angle=design-refs`.
> This standalone skill remains for backwards compatibility. `removed-after: v0.20.0 (~2026-08-01)`

## What this routes to

This skill is a Phase-5 specialization of `/research` (see
`skills/research/SKILL.md`). When invoked, it immediately hands off to
`/research` with the `design-refs` angle preselected, which runs:

- `web-researcher` teammate → Find award-winning and niche-relevant sites
  (Awwwards, CSS Design Awards, Godly, SiteInspire, One Page Love).
- `arch-analyst` teammate → For each reference: visual archetype, color
  system, type stack, motion tier, layout grid, notable interactions.
- `kei-critic` teammate → Which patterns are overused / stale? Where are
  references converging on the same AI-slop templates?

## Hand-off

```
=== /DESIGN-INSPIRATION ROUTE ===
Intake: <arguments>
Next:   /research --angle=design-refs <arguments>
```

## References

- `skills/research/SKILL.md` — full 8-phase flow
- `skills/frontend-design/SKILL.md` — archetype philosophy
- `skills/site-create/SKILL.md` — where inspiration feeds into
- `docs/CONVERGENCE-PLAN.md` §Pre-unlock quick wins #5
