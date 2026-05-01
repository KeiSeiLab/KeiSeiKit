# Architecture Decision Record — sample

## Decision

Use a single-trait Parser + Vec<Box<dyn>> registry for kei-decompose, NOT a HashMap.

## Context

- HashMap iteration order is non-deterministic and breaks tie-resolution.
- Parser registry needs ordered detection: first claim wins.

## Recommendations

1. Adopt the FormatParser trait + ordered registry pattern.
2. Add per-parser test fixtures under tests/fixtures/.
3. Document the registry invariants in the module-level docstring.

## Implementation

Land in Wave 52, ship as new primitive kei-decompose. kei-decision stays as one adapter.
