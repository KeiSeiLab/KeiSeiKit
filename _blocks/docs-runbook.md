# DOCS — Operational runbook template

A runbook tells on-call (or a future agent) exactly what to do when an alert fires. Every production system needs one per failure class. Format: *symptoms → checks → fixes → escalation*.

**File path:** `docs/runbooks/<component>-<alert-name>.md`. Index in `docs/runbooks/README.md` (or link from `HOTPATHS.md`).

**Template (copy as-is):**

```markdown
# Runbook — <component>: <alert or symptom name>

## Metadata
- **Severity:** SEV1 (page now) | SEV2 (work hours) | SEV3 (next day)
- **On-call rotation:** <team / pagerduty schedule / single handle>
- **Last rehearsed:** YYYY-MM-DD  (stale > 90d → re-rehearse)
- **Linked ADRs:** ADR-NNNN

## Symptoms
- Observable signal: <metric name> > <threshold> for <duration>
- User impact: <what breaks for end users>
- Typical dashboards: <URLs>

## Diagnostic checks (in order)
1. Check dashboard X — if metric Y is flat, skip to step 4
2. Tail logs: `<exact command>`
3. Inspect dependency Z status page: <URL>
4. Reproduce locally if unclear: `<command>`

## Fixes (try in order; STOP at first that works)
### Fix A — restart (lowest risk)
```bash
<exact command>
```
Verify: <metric returns to <threshold> within <time>>

### Fix B — rollback
```bash
<exact command>
```
Verify: <...>

### Fix C — scale up (if load-related)
```bash
<exact command>
```

## Escalation
- 15 min without recovery → page <secondary on-call>
- Data loss suspected → page <eng-lead> AND <security>
- Customer-visible > 30 min → post to <status-page-url>

## Post-incident
- File incident report at `docs/incidents/YYYY-MM-DD-<slug>.md`
- If root cause new → new ADR in `DECISIONS.md`
- If runbook step failed → update this file (date the edit)

## Known non-issues
- Symptom X that looks scary but is benign (e.g. queue lag < 5s during deploy)
```

**Rules:**
- One alert = one runbook. Do not bundle.
- Every command is copy-pasteable. No placeholders `<...>` in the live fixes section.
- Rehearse quarterly. Mark the date.

**Source:** Google SRE Book Ch. 11 "Being On-Call" and Ch. 14 "Managing Incidents" [E4]; PagerDuty Incident Response Documentation [E4].
