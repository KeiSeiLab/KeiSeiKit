# TEST — Property-based testing (invariants + shrinking)

A property test asserts an invariant — a statement true for every valid input — and the framework generates hundreds of inputs automatically. On failure, it shrinks the input to the minimal reproducer. Complements unit tests (which assert on hand-picked examples) and fuzz (which throws bytes at a boundary).

**When to use:** pure functions with stable contracts — parsers (`encode ∘ decode = id`), data structures (insert-then-lookup = hit), serializers, math, state machines with invariants. NOT for side-effectful handlers (use integration tests).

**Per-language tool (default):**
- **Rust:** `proptest` — `proptest! { fn roundtrip(s in "\\PC*") { assert_eq!(decode(encode(&s)), s); } }`. Supports stateful tests via `proptest-state-machine`. Prefer over `quickcheck` (proptest has better shrinking + regression file). [E4, proptest.rs]
- **Python:** `hypothesis` — `@given(st.integers())` / `@given(st.text())`. Stateful: `hypothesis.stateful.RuleBasedStateMachine`. Regression examples auto-saved under `.hypothesis/`. [E4, hypothesis.readthedocs.io]
- **JS/TS:** `fast-check` — `fc.assert(fc.property(fc.string(), s => decode(encode(s)) === s))`. Stateful: `fc.commands`. [E4, fast-check.dev]

**Writing a good property:**
1. **Round-trip:** `f⁻¹(f(x)) == x` (encode/decode, parse/print, serialize/deserialize).
2. **Idempotence:** `f(f(x)) == f(x)` (normalize, sort, dedupe).
3. **Invariant:** `op(x)` preserves property P (insert preserves size+1; sort preserves multiset).
4. **Metamorphic:** `f(g(x)) == h(f(x))` (commute operations).
5. **Comparison with oracle:** `my_fast_impl(x) == simple_slow_impl(x)` for all x.

**Shrinking:**
- When a test fails, framework automatically shrinks the counterexample to the smallest input reproducing the failure.
- Commit the shrunk example as a regression unit test. Do NOT rely on the `.proptest-regressions` / `.hypothesis/examples` cache alone — commit it, but also pin the hit in a normal test.

**Stateful tests:**
- Model a state machine: commands + preconditions + postconditions + model state.
- Framework generates valid command sequences, applies to SUT and model, asserts equality.
- Use for data structures, caches, stateful APIs, small DSLs.

**Config discipline:**
- `cases = 1024` default; bump to 10_000 for CI; lower to 64 for quick local iteration.
- Seed explicitly for reproducibility in CI logs (`PROPTEST_CASES=10000 PROPTEST_SEED=42`).

**Forbidden:**
- Property assertions that just restate the implementation (`f(x) == f(x)`).
- Disabling shrinking ("it took too long") — shrunk output is the whole point.
- Ignoring a single failing case as "flaky" — properties don't flake; the input found a bug.
- Mixing property tests with external services (DB, network) — properties must be deterministic.
