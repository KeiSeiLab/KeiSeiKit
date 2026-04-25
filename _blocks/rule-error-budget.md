# ERROR BUDGET — 3-Level Escalation

Counter: each FAILED attempt on the SAME problem = +1. Success = reset.

- **Level 1 (attempt 2 failed)**: STOP. Rollback (`git stash`). Re-read plan. Formulate ALTERNATIVE. Explain to user before continuing.
- **Level 2 (attempt 3 failed)**: STOP. Approach exhausted. Run focused research. Audit affected module. Check `wrong-paths.md`. New plan with evidence grades → user approval → THEN code.
- **Level 3 (still stuck)**: ESCALATE. Tell user "more complex than initially thought". Suggest workaround / simplify scope / defer / redesign.

**Prohibited:** third attempt with same approach; skipping Level 1; silent research without notifying user.
