#!/usr/bin/env sh
# kei-ci-lint — validate GitHub Actions / Forgejo Actions workflow YAML.
# POSIX sh, requires yq (v4+, Go impl — mikefarah/yq).
#
# Checks (one rule per check, exits non-zero on any violation unless --warn):
#   R1  required fields present          (name, on, jobs)
#   R2  least-privilege permissions      (top-level permissions set, not write-all)
#   R3  OIDC vs long-lived token         (id-token:write → no AWS_*_KEY secrets)
#   R4  cache-hit hygiene                (keys use hashFiles, not branch)
#   R5  action pinning                   (uses: pinned by SHA, not mutable tag)
#   R6  deprecated actions               (set-output, save-state, node12/16)
#   R7  pwn-request pattern              (pull_request_target + checkout of head)
#
# Usage:
#   kei-ci-lint <file.yml> [file2.yml ...]
#   kei-ci-lint --dir .github/workflows
#   kei-ci-lint --dir .forgejo/workflows --warn
#
# Exit: 0 clean, 1 violation(s), 2 usage/missing-dep.

set -eu

WARN=0
FILES=""
FAIL=0

usage() {
  cat <<'EOF'
Usage: kei-ci-lint <file.yml> [file2.yml ...]
       kei-ci-lint --dir <workflows-dir> [--warn]
Validates GitHub / Forgejo Actions workflow YAML.
EOF
}

need() {
  command -v "$1" >/dev/null 2>&1 || { echo "kei-ci-lint: missing $1 (install: $2)" >&2; exit 2; }
}

need yq "brew install yq"

# Argument parse
if [ $# -eq 0 ]; then usage; exit 2; fi
while [ $# -gt 0 ]; do
  case "$1" in
    -h|--help) usage; exit 0 ;;
    --warn)    WARN=1; shift ;;
    --dir)     [ -d "${2:-}" ] || { echo "kei-ci-lint: not a dir: ${2:-}" >&2; exit 2; }
               FILES="$FILES $(find "$2" -maxdepth 2 -type f \( -name '*.yml' -o -name '*.yaml' \) 2>/dev/null)"
               shift 2 ;;
    *)         [ -f "$1" ] || { echo "kei-ci-lint: not a file: $1" >&2; exit 2; }
               FILES="$FILES $1"; shift ;;
  esac
done

report() {
  # $1=file  $2=rule  $3=message
  if [ "$WARN" = "1" ]; then
    printf "WARN  %s  %s  %s\n" "$1" "$2" "$3"
  else
    printf "FAIL  %s  %s  %s\n" "$1" "$2" "$3"
    FAIL=$((FAIL+1))
  fi
}

check_file() {
  F="$1"
  # R1 required fields
  for key in name on jobs; do
    yq -e ".$key" "$F" >/dev/null 2>&1 || report "$F" R1 "missing top-level: $key"
  done

  # R2 least-privilege
  TOP_PERMS=$(yq '.permissions' "$F" 2>/dev/null || echo "null")
  case "$TOP_PERMS" in
    null) report "$F" R2 "no top-level permissions — default is write-all on classic repos" ;;
    write-all|"'write-all'") report "$F" R2 "permissions: write-all at workflow level" ;;
  esac

  # R3 OIDC ↔ long-lived keys
  HAS_OIDC=$(yq '.permissions."id-token" // (.jobs.*.permissions."id-token" // "")' "$F" 2>/dev/null | grep -c write || true)
  HAS_AWS_KEY=$(grep -E 'secrets\.AWS_(ACCESS_KEY_ID|SECRET_ACCESS_KEY)' "$F" 2>/dev/null | wc -l || echo 0)
  if [ "$HAS_OIDC" -gt 0 ] && [ "$HAS_AWS_KEY" -gt 0 ]; then
    report "$F" R3 "OIDC enabled AND long-lived AWS secrets present — pick one"
  fi
  if [ "$HAS_OIDC" = "0" ] && [ "$HAS_AWS_KEY" -gt 0 ]; then
    report "$F" R3 "uses long-lived AWS_* secrets — prefer OIDC (id-token:write)"
  fi

  # R4 cache-hit hygiene
  BAD_CACHE=$(grep -nE 'key:\s*.*github\.ref(_name)?' "$F" 2>/dev/null || true)
  if [ -n "$BAD_CACHE" ]; then
    report "$F" R4 "cache key uses github.ref (branch-scoped) — use hashFiles() instead"
  fi

  # R5 action pinning by SHA
  # Extract "uses:" values and check for SHA (40-hex) vs tag.
  yq '.jobs.*.steps[].uses // empty' "$F" 2>/dev/null | while IFS= read -r USES; do
    [ -z "$USES" ] && continue
    case "$USES" in
      ./*|../*|docker://*) continue ;;  # local/docker refs
    esac
    REF="${USES##*@}"
    # SHA if 40 hex chars
    if ! echo "$REF" | grep -qE '^[0-9a-f]{40}$'; then
      report "$F" R5 "action pinned by tag, not SHA: $USES"
    fi
  done

  # R6 deprecated surface
  for PAT in '::set-output' '::save-state' 'node12' 'actions/checkout@v[12]' 'actions/cache@v[12]'; do
    if grep -qE "$PAT" "$F" 2>/dev/null; then
      report "$F" R6 "deprecated: matches /$PAT/"
    fi
  done

  # R7 pwn-request: pull_request_target + checkout of PR head
  if yq -e '.on.pull_request_target' "$F" >/dev/null 2>&1; then
    if grep -qE 'ref:\s*\$\{\{\s*github\.event\.pull_request\.head\.sha' "$F" 2>/dev/null; then
      report "$F" R7 "pull_request_target + checkout of PR head SHA (pwn-request surface)"
    fi
  fi
}

for f in $FILES; do check_file "$f"; done

if [ "$FAIL" -gt 0 ]; then
  echo "kei-ci-lint: $FAIL violation(s)" >&2
  exit 1
fi
echo "kei-ci-lint: OK"
exit 0
