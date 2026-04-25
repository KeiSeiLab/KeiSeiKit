# DOCS — `DECISIONS.md` / ADR template (MADR 4.0)

Architecture Decision Records capture *why* a non-trivial choice was made, so future maintainers (including agents) don't re-litigate. Format: **MADR 4.0** (Markdown Any Decision Records, 2024). Nygard originated the practice in 2011.

**One ADR per non-trivial decision.** File path convention:
- Single-file log: append to `DECISIONS.md` (top-of-file = newest).
- Per-decision files: `docs/adr/NNNN-kebab-case-title.md` (NNNN = zero-padded int).

**MADR 4.0 template (copy as-is):**

```markdown
# ADR-NNNN: <short imperative title>

- **Status:** Proposed | Accepted | Rejected | Superseded-by-ADR-NNNN | Deprecated
- **Date:** YYYY-MM-DD
- **Deciders:** @handle, @handle
- **Evidence grade:** E1-E6 (see `_blocks/evidence-grading.md`)

## Context and Problem Statement
<1-3 sentences: what forces us to decide? What breaks if we don't?>

## Decision Drivers
- Driver 1 (e.g. cost < $X/mo)
- Driver 2 (e.g. must run offline)
- Driver 3 (e.g. team knows language Y)

## Considered Options
1. **Option A** — one-line summary
2. **Option B** — one-line summary
3. **Option C** — one-line summary

## Decision Outcome
Chosen: **Option <letter>**, because <1-2 sentences tying back to drivers>.

### Consequences
- Positive: <what improves>
- Negative: <what we give up, tech debt incurred>
- Neutral: <noteworthy but not directional>

## Pros and Cons of the Options
### Option A
- Pro: ...
- Con: ...
### Option B
- Pro: ...
- Con: ...

## Links
- Supersedes: ADR-NNNN
- Related: `HOTPATHS.md#section`, external URL
- Evidence source: [VERIFIED: url] or [UNVERIFIED]
```

**Rules:**
- Status `Accepted` = implemented or actively being implemented. `Proposed` = under review. `Rejected` stays as an ADR (the record of why we said no).
- Never delete a past ADR. Supersede with a new ADR that references the old number.
- Evidence grade mandatory (RULE 0.4). No grade → the ADR is unreviewable.

**Source:** MADR 4.0 spec — [adr/madr](https://adr.github.io/madr/) [E4]. Nygard 2011 original post `cognitect.com/blog/2011/11/15/documenting-architecture-decisions` [E4].
