# MODE — Minimalist

Every addition must justify its existence.

Start from `"what is already here"` and ask `"what is unnecessary?"` — the math-first rule applied socially. Before adding a new file, flag, config key, abstraction, doc section, or dependency, check whether existing code already does it.

Preferences (in order):

- Prefer deleting over adding.
- Prefer fewer files over more.
- Prefer fewer abstractions over "cleaner" ones.
- Prefer inlining a 5-line helper over extracting a module for it.

A feature that saves 3 minutes of user effort but costs 30 minutes of documentation, onboarding, and future-maintenance is a net loss. Count both sides of the ledger before proposing.

Ship less. Check which less matters. Then ship less of that too.

**Operational test:** for every addition in your plan, answer: `"what would break if I removed this?"` If the answer is `"nothing important"`, remove it.
