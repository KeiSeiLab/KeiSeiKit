#!/bin/sh
# kei-docs-scaffold — detect project type, generate missing docs from templates.
# First-class primitive, POSIX sh (no bash-isms), ports to KeiSeiKit convention.
# Install path: $HOME/.claude/agents/_primitives/kei-docs-scaffold.sh
#
# Usage:
#   kei-docs-scaffold.sh [--type=all|claude|decisions|runbook|readme] [--force] [--dry-run] [DIR]
#
# Flags:
#   --type=TYPE   Which doc to scaffold. Default: all.
#                 Values: all | claude | decisions | runbook | readme
#   --force       Overwrite existing files. Default: skip if present.
#   --dry-run     Print actions, do not write.
#   DIR           Project directory. Default: $PWD.
#
# Detection: examines DIR for Cargo.toml / package.json / pyproject.toml /
# pubspec.yaml / go.mod / Package.swift / docker-compose.yml. Writes
# scaffolds pre-filled with the detected stack name.
# Safe to re-run: idempotent without --force.

set -eu

# ---- defaults -------------------------------------------------------------
TYPE="all"
FORCE=0
DRY_RUN=0
DIR=""

# ---- flag parsing (POSIX, no getopt_long) ---------------------------------
while [ $# -gt 0 ]; do
  case "$1" in
    --type=*)   TYPE="${1#--type=}" ;;
    --type)     shift; TYPE="$1" ;;
    --force)    FORCE=1 ;;
    --dry-run)  DRY_RUN=1 ;;
    -h|--help)
      sed -n '2,18p' "$0" | sed 's/^# \{0,1\}//'
      exit 0
      ;;
    -*)
      printf '[scaffold] unknown flag: %s\n' "$1" >&2
      exit 2
      ;;
    *)
      [ -z "$DIR" ] || { printf '[scaffold] multiple DIR args\n' >&2; exit 2; }
      DIR="$1"
      ;;
  esac
  shift
done

DIR="${DIR:-$PWD}"
[ -d "$DIR" ] || { printf '[scaffold] not a directory: %s\n' "$DIR" >&2; exit 2; }

case "$TYPE" in
  all|claude|decisions|runbook|readme) : ;;
  *) printf '[scaffold] invalid --type: %s\n' "$TYPE" >&2; exit 2 ;;
esac

# ---- stack detection ------------------------------------------------------
detect_stack() {
  if   [ -f "$DIR/Cargo.toml" ];        then echo "Rust (Cargo)"
  elif [ -f "$DIR/pubspec.yaml" ];      then echo "Flutter / Dart"
  elif [ -f "$DIR/package.json" ];      then echo "Node.js / TypeScript"
  elif [ -f "$DIR/pyproject.toml" ];    then echo "Python (pyproject)"
  elif [ -f "$DIR/requirements.txt" ];  then echo "Python (pip)"
  elif [ -f "$DIR/go.mod" ];            then echo "Go"
  elif [ -f "$DIR/Package.swift" ];     then echo "Swift (SPM)"
  elif [ -f "$DIR/docker-compose.yml" ]; then echo "Docker (compose)"
  else echo "Unknown"
  fi
}

detect_test_cmd() {
  case "$1" in
    "Rust (Cargo)")          echo "cargo test --release && cargo clippy -- -D warnings" ;;
    "Flutter / Dart")        echo "flutter test && flutter analyze" ;;
    "Node.js / TypeScript")  echo "npm test" ;;
    "Python"*)               echo "pytest -q" ;;
    "Go")                    echo "go test ./..." ;;
    "Swift (SPM)")           echo "swift test" ;;
    *)                       echo "# TODO: set test command" ;;
  esac
}

STACK=$(detect_stack)
TEST_CMD=$(detect_test_cmd "$STACK")
PROJECT_NAME=$(basename "$DIR")

printf '[scaffold] project: %s  stack: %s\n' "$PROJECT_NAME" "$STACK" >&2

# ---- write helpers --------------------------------------------------------
write_file() {
  target="$1"
  if [ -e "$target" ] && [ "$FORCE" -eq 0 ]; then
    printf '[scaffold]   skip (exists): %s\n' "$target" >&2
    return 0
  fi
  if [ "$DRY_RUN" -eq 1 ]; then
    printf '[scaffold]   would write: %s\n' "$target" >&2
    cat > /dev/null
    return 0
  fi
  cat > "$target"
  printf '[scaffold]   wrote: %s\n' "$target" >&2
}

# ---- doc generators (one function per file, ≤ 30 LOC) ---------------------
gen_claude() {
  write_file "$DIR/CLAUDE.md" <<EOF
# CLAUDE.md — $PROJECT_NAME

> Agent-facing project guide. Read FIRST at session start.

## Architecture

- [ ] Layer 1 — <describe>
- [ ] Layer 2 — <describe>
- [ ] Data flow — <describe>

## Stack

- **Language / framework:** $STACK
- **Package manager:** <pin>
- **Test runner:** \`$TEST_CMD\`

## Constraints

- Constructor Pattern: file < 200 LOC, function < 30 LOC
- No new dependency without Plan Mode
- No \`.unwrap()\` / \`.expect()\` in prod paths (Rust)

## Known issues

- [ ] <file:line> — <symptom>

## Test invariants

\`\`\`
$TEST_CMD
\`\`\`

## Hot paths (double-audit on touch)

- [ ] <file> — <why>

## References

- \`DECISIONS.md\` — architectural decisions
- \`docs/runbook.md\` — ops playbook (if deployed)
EOF
}

gen_decisions() {
  write_file "$DIR/DECISIONS.md" <<'EOF'
# DECISIONS.md — Architectural Decision Records (MADR 4.0)

> Append-only. One decision = one ADR entry. Never delete; supersede instead.

## ADR-001 — Adopt Constructor Pattern
- **Status:** accepted
- **Date:** TODO
- **Deciders:** TODO
- **Evidence grade:** E4

### Context and problem statement
Projects tend to grow monolithic files / DI containers / abstract factories.
Debugging and refactoring slow down.

### Decision drivers
- Every file readable in one screen (< 200 LOC)
- Every function fits in one mental buffer (< 30 LOC)
- Reproducibility: any cube swappable

### Considered options
1. **Constructor Pattern** — 1 file = 1 class = 1 responsibility
2. Classical layered OOP — DI containers, mixins
3. Free-form — no rules

### Decision outcome
Chosen: **Option 1** — Constructor Pattern.

### Consequences
- Good: easy audit, easy swap, cubes compose
- Bad: more files, more module boundaries
- Neutral: matches KeiSeiKit kit defaults

### Verification
File-size lint on every PR. See repo CI.
EOF
}

gen_runbook() {
  mkdir -p "$DIR/docs"
  write_file "$DIR/docs/runbook.md" <<'EOF'
# Runbook — Ops Playbook

> One symptom = one entry. Check → Fix → Escalation. Read-only before write.

## Service does not start

### Check (read-only, < 5 min)
- `<status command>` — expected: running
- Log path: `<path>` — grep for `panic|ERROR|fatal`
- Port open? `<nc / lsof command>`

### Fix (by likelihood, safest first)
1. **Config drift** — diff env vs template, reapply
2. **Port conflict** — kill conflicting process (NOT the service itself)
3. **Upstream outage** — check status page, fall back
4. **Data corruption** — restore from last checkpoint

### Escalation
- After 15 min failed fixes → escalate to on-call
- After 3 repeats in 24h → open an incident review

---

## Latency spike

### Check
- `<metrics command>`
- Grep slow queries in `<log path>`

### Fix
1. Restart worker (BENIGN first)
2. Check DB connection pool saturation
3. Scale up if sustained

### Escalation
- After 30 min sustained p99 > SLO → page on-call
EOF
}

gen_readme() {
  write_file "$DIR/README.md" <<EOF
# $PROJECT_NAME

> One-line pitch: <what this project does in ≤ 12 words>.

## Why

<One paragraph: the problem + how this differs from alternatives.>

## Install

\`\`\`
# TODO: paste copy-pasteable install command
\`\`\`

## Quickstart

\`\`\`
# TODO: minimal runnable example, ≤ 15 lines
\`\`\`

## Features

- [ ] Feature A — link to docs
- [ ] Feature B — link to docs

## Architecture

See \`CLAUDE.md\` for agent-facing details. Stack: **$STACK**.

## Status

Alpha. Versioning: SemVer 0.x.

## License

See \`LICENSE\`.
EOF
}

# ---- dispatch -------------------------------------------------------------
case "$TYPE" in
  all)       gen_claude; gen_decisions; gen_runbook; gen_readme ;;
  claude)    gen_claude ;;
  decisions) gen_decisions ;;
  runbook)   gen_runbook ;;
  readme)    gen_readme ;;
esac

printf '[scaffold] done\n' >&2
