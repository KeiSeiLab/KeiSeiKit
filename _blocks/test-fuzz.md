# TEST — Fuzzing (input-space exploration)

Fuzzing throws semi-random inputs at a target to find crashes, panics, hangs, and undefined behaviour the unit-test author never imagined. Complements `test-gen` (happy/edge/error) — fuzz owns the unknown-unknown surface.

**When to fuzz:** parsers, deserializers, protocol handlers, auth/crypto boundaries, any function that accepts untrusted bytes or strings. NOT business logic with well-defined inputs (use property tests instead).

**Per-language tool (default):**
- **Rust:** `cargo-fuzz` (libfuzzer-sys backend) — `cargo fuzz init`, then `fuzz_target!(|data: &[u8]| { my_parser(data); })`. Requires nightly. Harness lives in `fuzz/fuzz_targets/`. [E4, official: https://rust-fuzz.github.io/book/]
- **Python:** `hypothesis` in fuzz mode (`@given` + `HealthCheck.too_slow` disabled) for structured inputs; `atheris` (Google, libfuzzer bindings) for bytes-in fuzzing. [E4, hypothesis.readthedocs.io / github.com/google/atheris]
- **JS/TS:** `fast-check` with `fc.assert` using `numRuns: 10_000+` for fuzz-volume runs; `jsfuzz` for libFuzzer-style bytes fuzzing. [E4, fast-check.dev]

**Corpus management:**
- Seed corpus = hand-picked valid inputs (1-10 files). Place under `fuzz/corpus/<target>/`.
- Fuzzer mutates corpus → keeps inputs that hit new coverage → corpus grows.
- Commit corpus to git (gitignore `fuzz/artifacts/`). Treat as test fixture.

**Crash triage:**
1. Fuzzer dumps crash input under `fuzz/artifacts/<target>/crash-<hash>`.
2. Reproduce: `cargo fuzz run <target> fuzz/artifacts/<target>/crash-<hash>`.
3. Minimize: `cargo fuzz tmin <target> <input>` — shrinks to minimal reproducer.
4. Write a regression unit test using the minimized input BEFORE fixing the bug. Regression test is permanent; fuzz corpus is ephemeral.

**CI integration (budget-aware):**
- Short CI run: 60s per target on every PR. Catches regressions, not deep bugs.
- Nightly run: 1-4h per target on schedule. Upload crashes as artifacts.
- OSS-Fuzz (free for OSS): submit a `project.yaml` + Dockerfile + build script; Google runs fuzzing on their infra. [E4, google.github.io/oss-fuzz]

**Forbidden:**
- Fuzzing without a crash-reproducer harness (crashes become irreproducible).
- Running fuzzer without `cargo fuzz tmin` / equivalent — full-size crashes waste reviewer time.
- Committing `fuzz/artifacts/` (binary crash bodies, repo bloat).
- Treating a fuzz hit as "flaky" — every crash is a bug until minimized + explained.
