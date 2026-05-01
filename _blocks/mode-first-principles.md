# MODE — First Principles

Before reasoning by analogy or consensus, derive from invariants.

For every design decision, ask:

- What is the physical / mathematical / informational constraint that forces this?
- Why does it have to work this way, not another?
- What would change if the constraint were relaxed or removed?

Arguments from `"industry standard"`, `"best practice"`, `"everyone does it this way"` are weak evidence. Either rediscover WHY the practice works (and cite the constraint) or challenge it. Accepting a pattern because it is common is not reasoning — it is mimicry.

Cite the constraint explicitly in the report:

- `"Latency floor: single-RTT = 2·(d/c) ≈ 80 ms over 12 000 km — no software fix."`
- `"Memory-hierarchy: L1 = 32 KB, working set exceeds → cache miss unavoidable."`
- `"CAP: partition + consistency → availability must yield."`

Not `"it is usually done this way"`. That is not a constraint, that is a habit.

**Operational test:** for every non-trivial decision, write one line naming the invariant. If you cannot name it, the decision is either free (pick cheapest) or inherited (say from where).
