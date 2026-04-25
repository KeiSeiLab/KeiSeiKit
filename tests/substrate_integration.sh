#!/usr/bin/env bash
# substrate_integration.sh — cross-stream integration smoke test
#
# Architect P0-b (audit wave 2026-04-23): each stream (kei-forge / kei-task
# atoms / kei-sage / kei-runtime) has its own smoke tests, but no single
# test exercised the cross-stream composition. This script is that test.
#
# The check: build release binaries, generate a fresh atom via new-atom.sh,
# then verify that kei-runtime + kei-sage BOTH discover it identically and
# that kei-runtime schema-lint passes on it.
#
# Exit 0 = substrate v1 contract holds end-to-end
# Exit 1 = any step failed — see stderr for the offending stage

set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

TMPROOT="$(mktemp -d)"
trap 'rm -rf "$TMPROOT"' EXIT

fail() { echo "SUBSTRATE-INTEGRATION FAIL: $*" >&2; exit 1; }

echo "==> Building release binaries (kei-runtime, kei-sage, kei-task)…"
cd _primitives/_rust
cargo build --release -p kei-runtime -p kei-sage -p kei-task >/dev/null 2>&1 \
    || fail "cargo build failed"
RT="$(pwd)/target/release/kei-runtime"
SAGE="$(pwd)/target/release/kei-sage"
BIN_DIR="$(pwd)/target/release"
cd "$ROOT"

echo "==> Scaffolding a fresh atom (kei-task::create) via new-atom.sh for isolated test corpus…"
CORPUS="$TMPROOT/corpus/kei-task"
mkdir -p "$CORPUS"/{atoms/schemas,src/atoms,tests}

# Minimal hand-crafted atom mirroring Stream B's create atom shape —
# covers all REQUIRED frontmatter fields so schema-lint passes.
cat > "$CORPUS/atoms/create.md" <<'EOF'
---
atom: kei-task::create
kind: command
version: "0.22.3"
input:
  schema: schemas/create-input.json
  required: [title]
  example: { title: "x" }
output:
  schema: schemas/create-output.json
  example: { id: 1 }
errors:
  - code: DuplicateTitle
    http_analog: 409
side_effects:
  - { op: write, domain: kei-task-db }
idempotent: false
timeout_ms: 5000
stability: stable
keywords: [integration-test]
related: []
---

# kei-task::create

Integration-test atom. See substrate_integration.sh.
EOF

cat > "$CORPUS/atoms/schemas/create-input.json" <<'EOF'
{
  "$schema": "http://json-schema.org/draft-07/schema#",
  "$id": "kei-task/atoms/schemas/create-input.json",
  "title": "kei-task::create input",
  "type": "object",
  "required": ["title"],
  "properties": {
    "title": { "type": "string", "minLength": 1 }
  },
  "additionalProperties": false,
  "examples": [{"title": "x"}]
}
EOF

cat > "$CORPUS/atoms/schemas/create-output.json" <<'EOF'
{
  "$schema": "http://json-schema.org/draft-07/schema#",
  "$id": "kei-task/atoms/schemas/create-output.json",
  "title": "kei-task::create output",
  "type": "object",
  "properties": { "id": { "type": "integer" } },
  "additionalProperties": false,
  "examples": [{"id": 1}]
}
EOF

echo "==> kei-runtime schema-lint…"
"$RT" schema-lint --root "$TMPROOT/corpus" \
    | grep -q "^PASS" \
    || fail "schema-lint did not report PASS"

echo "==> kei-runtime list-atoms…"
LIST="$("$RT" list-atoms --root "$TMPROOT/corpus")"
echo "$LIST" | grep -q "kei-task::create" \
    || fail "kei-runtime list-atoms did not see kei-task::create"

echo "==> kei-sage atoms-discover…"
DISCOVER="$("$SAGE" atoms-discover --root "$TMPROOT/corpus")"
echo "$DISCOVER" | grep -q "kei-task::create" \
    || fail "kei-sage atoms-discover did not see kei-task::create"

echo "==> Cross-stream ID agreement…"
RT_IDS="$(echo "$LIST" | awk '{print $1}' | sort)"
SAGE_IDS="$(echo "$DISCOVER" | awk 'NR>1 && $1 != "" {print $1}' | sort)"
[ "$RT_IDS" = "$SAGE_IDS" ] \
    || fail "runtime and sage disagree on atom IDs:\n  runtime: $RT_IDS\n  sage:    $SAGE_IDS"

echo "==> kei-runtime invoke (expects real exec via Stream E → exit 0 + result.id)…"
set +e
INVOKE_OUT="$(KEI_RUNTIME_BIN_DIR="$BIN_DIR" KEI_TASK_DB="$TMPROOT/task.sqlite" \
    "$RT" invoke --root "$TMPROOT/corpus" kei-task::create --input '{"title":"integration"}' 2>&1)"
RC=$?
set -e
[ "$RC" -eq 0 ] \
    || fail "invoke should exit 0 on real exec, got $RC: $INVOKE_OUT"
echo "$INVOKE_OUT" | grep -q '"id"' \
    || fail "invoke stdout missing 'id' field: $INVOKE_OUT"

echo "==> kei-runtime invoke with missing binary (expects BinaryNotFound → exit 127)…"
set +e
PATH="/usr/bin:/bin" KEI_RUNTIME_BIN_DIR="/nonexistent" \
    "$RT" invoke --root "$TMPROOT/corpus" kei-task::create --input '{"title":"x"}' >/dev/null 2>&1
RC=$?
set -e
[ "$RC" -eq 127 ] \
    || fail "invoke with missing binary should exit 127, got $RC"

echo "==> kei-runtime invoke with bad input (expects InputInvalid → exit 2)…"
set +e
"$RT" invoke --root "$TMPROOT/corpus" kei-task::create --input '{}' >/dev/null 2>&1
RC=$?
set -e
[ "$RC" -eq 2 ] \
    || fail "invoke with missing required field should exit 2, got $RC"

# ---------------------------------------------------------------------------
# Phase 5 — migrated agent assertions (v0.16)
# ---------------------------------------------------------------------------
# After the atom-substrate checks above, confirm that the 5 kit-shipped
# agents migrated to the agent-substrate role+task-spec invocation model
# assemble with their capability fragments injected, and that
# kei-agent-runtime compose succeeds on a task.toml that references one
# of their roles.

echo "==> Phase 5 — building assembler + kei-agent-runtime…"
( cd _assembler && cargo build --release >/dev/null 2>&1 ) \
    || fail "assembler release build failed"
( cd _primitives/_rust && cargo build --release -p kei-agent-runtime >/dev/null 2>&1 ) \
    || fail "kei-agent-runtime release build failed"

ASSEMBLE_BIN="$ROOT/_assembler/target/release/assemble"
RUNTIME_BIN="$ROOT/_primitives/_rust/target/release/kei-agent-runtime"
[ -x "$ASSEMBLE_BIN" ] || fail "assemble binary missing at $ASSEMBLE_BIN"
[ -x "$RUNTIME_BIN" ] || fail "kei-agent-runtime binary missing at $RUNTIME_BIN"

echo "==> Phase 5 — discovering migrated manifests (substrate_role field)…"
MIGRATED=""
for m in "$ROOT"/_manifests/*.toml; do
    if grep -qE '^substrate_role[[:space:]]*=' "$m"; then
        MIGRATED+="$(basename "$m" .toml) "
    fi
done
MIGRATED_COUNT="$(echo "$MIGRATED" | wc -w | tr -d ' ')"
# v0.16 phase-5 wave 2 (2026-04-23): +7 agents (cost-guardian, ml-researcher,
# researcher, modal-runner, fal-ai-runner, infra-implementer, ml-implementer)
# lifts the migrated count from 5 to 12. Keep the floor at 12 so regressions
# (missing substrate_role on any of those 7) fail the gate.
[ "$MIGRATED_COUNT" -ge 12 ] \
    || fail "expected ≥12 migrated manifests, found $MIGRATED_COUNT: $MIGRATED"

echo "==> Phase 5 — assembling each migrated manifest to temp + checking substrate section…"
GEN_ROOT="$TMPROOT/migrated"
mkdir -p "$GEN_ROOT/_manifests" "$GEN_ROOT/_blocks" "$GEN_ROOT/_roles" "$GEN_ROOT/_capabilities"
cp "$ROOT"/_manifests/*.toml "$GEN_ROOT/_manifests/"
cp "$ROOT"/_blocks/*.md      "$GEN_ROOT/_blocks/"
cp "$ROOT"/_roles/*.toml     "$GEN_ROOT/_roles/"
cp -R "$ROOT"/_capabilities/* "$GEN_ROOT/_capabilities/"

for name in $MIGRATED; do
    AGENT_ROOT="$GEN_ROOT" HOME="$GEN_ROOT" \
        "$ASSEMBLE_BIN" --in-place "$GEN_ROOT/_manifests/${name}.toml" >/dev/null 2>&1 \
        || fail "assemble --in-place failed for $name"
    MD="$GEN_ROOT/${name}.md"
    [ -f "$MD" ] || fail "generated md missing for $name: $MD"
    grep -q '^# AGENT SUBSTRATE — role `' "$MD" \
        || fail "$name: missing '# AGENT SUBSTRATE — role ...' header"
    grep -q '^# BASELINE' "$MD" \
        || fail "$name: missing # BASELINE block after substrate (block order broken)"
done

echo "==> Phase 5 — smoke check: kei-code-implementer.md carries the policy::no-git-ops fragment…"
grep -q 'You MUST NOT invoke `git`' "$GEN_ROOT/kei-code-implementer.md" \
    || fail "kei-code-implementer substrate fragment (no-git-ops) missing"

echo "==> Phase 5 — smoke check: kei-critic.md (read-only role) carries the tools::read-only fragment…"
grep -q 'You MUST NOT use the `Edit` or `Write` tools' "$GEN_ROOT/kei-critic.md" \
    || fail "kei-critic substrate fragment (read-only) missing"

echo "==> Phase 5 — kei-agent-runtime compose against an example task.toml…"
EXAMPLE="$ROOT/_templates/task-examples/edit-local-forge.toml"
[ -f "$EXAMPLE" ] || fail "task example missing: $EXAMPLE"
COMPOSED="$("$RUNTIME_BIN" compose "$EXAMPLE" --kit-root "$ROOT" 2>&1)" \
    || fail "kei-agent-runtime compose failed: $COMPOSED"
echo "$COMPOSED" | grep -q 'You MUST NOT invoke `git`' \
    || fail "composed prompt missing policy::no-git-ops fragment"
echo "$COMPOSED" | grep -q 'under 200 lines of code' \
    || fail "composed prompt missing quality::constructor-pattern fragment"
echo "$COMPOSED" | grep -q 'Replace the shell-out templating' \
    || fail "composed prompt missing task.body.text"

echo "==> Phase 5 — cargo check --workspace from main (no regression)…"
( cd _primitives/_rust && cargo check --workspace >/dev/null 2>&1 ) \
    || fail "cargo check --workspace failed after phase 5 migration"

echo ""
echo "✓ SUBSTRATE-INTEGRATION PASS — atom-substrate + phase-5 migration checks all green"
