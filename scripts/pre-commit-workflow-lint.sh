#!/bin/sh
# pre-commit-workflow-lint.sh — pre-commit gate for workflow-file edits.
# Install: ln -sf ../../scripts/pre-commit-workflow-lint.sh .git/hooks/pre-commit
#
# Runs lint-workflows.sh + validate-workflow-shas.sh iff any staged file
# matches .github/workflows/*.y(a)ml or .github/dependabot.yml. No-op
# otherwise. Mirrors scripts/precommit-counts-check.sh in spirit.

set -eu

ROOT=$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)

STAGED=$(git diff --cached --name-only --diff-filter=ACMR 2>/dev/null || true)

# Match workflow-file edits via grep rather than a case-inside-subshell
# (macOS bash 3.2 mis-parses `;;` inside a $(... | while ... case ... esac)).
HIT_OUT=$(printf '%s\n' "${STAGED}" \
  | grep -E '^\.github/(workflows/.*\.(yml|yaml)|dependabot\.yml)$' \
  || true)

if [ -z "${HIT_OUT}" ]; then
  exit 0
fi

printf 'workflow files staged — running lint + SHA validation\n'
printf '%s\n' "${HIT_OUT}" | sed 's/^/  staged: /'

RC=0
"${ROOT}/scripts/lint-workflows.sh" || RC=$?
if [ "${RC}" -ne 0 ]; then
  cat >&2 <<EOF

actionlint reported findings. Fix them or unstage the workflow files, then retry.
EOF
  exit 1
fi

"${ROOT}/scripts/validate-workflow-shas.sh" || RC=$?
if [ "${RC}" -ne 0 ]; then
  cat >&2 <<EOF

validate-workflow-shas.sh reported MISSING SHAs. A pinned SHA does not
resolve at the upstream remote. Possible causes:

  - Fabricated SHA (hallucinated digits)
  - Force-pushed branch on upstream (rare, historical)
  - Typo

Fix the SHA or unstage the workflow file.
EOF
  exit 1
fi

exit 0
