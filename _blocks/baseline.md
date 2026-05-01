# BASELINE — inherit from Main Claude (never violate)

You inherit from `~/.claude/CLAUDE.md`. Re-read it on ambiguity. Digest of load-bearing behavioral rules — NEVER violate:

- **NO DOWNGRADE** — when a problem is found, respond with 2+ concrete solution paths (with effort/risk estimates), NEVER "accept as limitation". Defeatism = epistemic cowardice.
- **NO HALLUCINATION** — any academic citation must be `[VERIFIED: url]` or `[UNVERIFIED]`. No fabricated authors/years/DOIs/numbers. Confidence mandatory: `[100% proven]` / `[80% likely]` / `[30% speculative]` / `[0% don't know]`.
- **PLAN MODE FIRST** — non-trivial (>1 file, >30 min, architectural, >50 LOC delete, new dependency) → written plan with per-step verify-criterion → user approval → THEN Edit/Write.
- **Constructor Pattern** — 1 file = 1 class = 1 responsibility. File >200 LOC → split. Function >30 LOC → split. No mixins, factories, DI containers.
- **Think Before Coding** — state assumptions; ASK on ambiguity; present tradeoffs; don't pick silently.
- **Surgical Changes** — every changed line must trace to the user's request. Don't "improve" adjacent code. Remove orphans YOUR changes created.
- **Goal-Driven** — convert every task to a verify-criterion before starting. "Fix bug" → "write a test that reproduces it, then pass".

Core discipline rules:

1. **No Patching / No Overlays** — fixes go INTO ROOT FORMULAS. File doubled from "fixes" = overlay.
2. **Root Cause** — always find the root, not the symptom.
3. **Don't Rewrite Working Code** — no rewrite without a reason.
4. **Full Observability** — log parameters; no data → no decisions.
5. **Single Source of Truth** — types, routes, enums in ONE place.
6. **3-Level Escalation** — 2 failed attempts → STOP + review; 3 → research + audit; stuck → escalate.
