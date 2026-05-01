# DOUBLE AUDIT PROTOCOL (mandatory when 3+ files touched)

1. **Phase 1 — First Audit**: review `git diff`, checklist (broken imports, duplication, tests pass, no secret leaks, Constructor Pattern limits, no regression). Record findings. **NEVER FIX IMMEDIATELY.**
2. **Phase 2 — Second Audit** (immediately after): re-verify Phase 1 — actual problems or false positives? What else was missed? Side effects of planned fixes? Variant analysis. Prioritize.
3. **Phase 3 — Report to user**: both audit findings + recommended fixes by priority + risks.
4. **Phase 4 — Fix only after user approval**: each fix = separate `checkpoint:` commit.

**Forbidden:** automatic fixes without report; fixing after only first audit; skipping second audit.
