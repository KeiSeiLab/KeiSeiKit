# Phase 3 — Prior-art grep sweep (parallel)

For EACH component from Phase 2, run three independent searches in parallel
(single message, multiple Bash tool calls).

## 3a — KeiSeiKit reuse

```bash
# Replace <keywords> with the component's 3-5 distinctive keywords as an
# ERE alternation like (foo|bar|baz).
# The 7 paths cover: behavioral blocks, agent manifests, shell primitives,
# Rust primitive crates (source + Cargo.toml), all skills, cross-tool
# bridges, and enforced hooks. grep -r recurses, so _primitives/ catches
# both *.sh and _rust/<crate>/src/*.rs — but _primitives/_rust/ is listed
# explicitly for discoverability when someone reads this file.
grep -rinlE '<keywords>' \
  _blocks/ _manifests/ _primitives/ _primitives/_rust/ \
  skills/ _bridges/ hooks/ 2>/dev/null
```

## 3b — Personal bundle reuse (conditional, skip on missing)

If the environment variable `KEISEI_BUNDLE_PATH` is set and the directory
exists, grep prior art there. Otherwise skip Layer B. Do not hard-code
any path — the bundle is user-specific.

```bash
bundle="${KEISEI_BUNDLE_PATH:-}"
if [ -n "$bundle" ] && [ -d "$bundle" ]; then
  grep -rinlE '<keywords>' "$bundle" 2>/dev/null | head -20
else
  echo "personal bundle: absent (KEISEI_BUNDLE_PATH unset or missing) — skipping layer B"
fi
```

Document absence in the report — do NOT fabricate a hit.

## 3c — External docs (delegate)

For any component that involves an external API, framework, or third-party
library, delegate a tiny research task to a `kei-researcher` subagent: one
WebSearch call, one WebFetch of the top hit, one-paragraph summary. Skip if
the component is entirely internal.

## 3d — Classify + evidence-grade

For each component produce ONE row:

```
Component N: <one-line>
  Keywords:  (foo|bar|baz)
  3a reuse:  <path1>, <path2>   or  NONE
  3b reuse:  <path> (bundle)    or  ABSENT / NONE
  3c ext:    <URL summary>      or  INTERNAL
  Class:     [REUSE | ADAPT | CREATE | EXTERNAL]
  Evidence:  [E1-E6]
```

## Verify-criterion

- Every component has a classification.
- Every cited file path exists on disk (RULE 0.4 — no fabricated paths).
- If grep returns nothing, class is CREATE and the report says so — no
  phantom matches.
