---
name: animate
description: Gateway router for web animation work. Picks between /scroll-animation, /motion-design, /web-effects, or /ai-animation via one AskUserQuestion call, then hands off. Does NOT implement animation — the four downstream skills remain the source of truth for their respective domains.
argument-hint: <optional one-line description of the animation, e.g. "hero pins and scrubs">
---

# /animate — Animation Router (gateway)

You are routing an animation request to the correct specialist skill. This
skill is a 1-click dispatcher; it never edits files itself. The four
downstream skills have disjoint decision matrices over different library
ecosystems (GSAP / Motion / WebGL / video-gen), so consolidation is
rejected per `docs/CONVERGENCE-PLAN.md`.

## When to use

Triggers: `/animate`, "add animation", "animate this", "web animation",
"motion", user unsure which animation skill applies.

## Phase 1 — Route (one AskUserQuestion call)

Ask one question with four options:

- **Scroll-driven (pin, scrub, reveal-on-scroll)** → route to
  `/scroll-animation` (GSAP ScrollTrigger / scroll-timeline).
- **Page or element transitions (enter, exit, layout-shift)** → route to
  `/motion-design` (Motion / Framer Motion library).
- **WebGL / particles / shaders / 3D backgrounds** → route to
  `/web-effects` (Three.js / R3F / shader material).
- **AI-generated video or animation clip** → route to `/ai-animation`
  (Runway / Kling / Pika / Sora via fal.ai etc.).

Per `_blocks/rule-pure-click-contract.md` — the intake line is optional
free-text; the four options are all `AskUserQuestion`.

## Phase 2 — Hand off

Emit a single line telling the user which skill to invoke next, preserving
the intake argument if one was provided:

```
=== /ANIMATE ROUTE ===
Intake:   <optional one-liner or "(none)">
Picked:   <Scroll-driven | Transitions | WebGL/effects | AI video>
Next:     /<picked-skill> <intake-or-blank>
```

Example:

```
=== /ANIMATE ROUTE ===
Intake:   hero pins while user scrolls through 3 feature cards
Picked:   Scroll-driven
Next:     /scroll-animation hero pins while user scrolls through 3 feature cards
```

## Rules

- **Surgical scope** — never write, edit, or delete any file. Output is
  one chat message with the routing report.
- **RULE 0.4 NO HALLUCINATION** — if the user picks AI-video and
  `skills/ai-animation/` does not exist on disk, report that fact and
  suggest `/web-effects` or `/motion-design` as constructive alternatives
  (RULE -1 NO DOWNGRADE).
- **No auto-invocation** — print the handoff line; let the user invoke
  the chosen skill so they keep control of the switch.

## References

- `skills/scroll-animation/SKILL.md`
- `skills/motion-design/SKILL.md`
- `skills/web-effects/SKILL.md`
- `skills/ai-animation/SKILL.md` (if present)
- `_blocks/rule-pure-click-contract.md`
- `docs/CONVERGENCE-PLAN.md` §Pre-unlock quick wins
