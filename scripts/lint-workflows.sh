#!/bin/sh
# lint-workflows.sh — run actionlint over every workflow file.
# Advisory-only behaviour if actionlint is not installed: prints an install
# hint and exits 0 (mirrors the existing shellcheck step).
# Hard-fails (exit 1) only when actionlint itself reports findings.

set -eu

ROOT=$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)
WF_DIR="${ROOT}/.github/workflows"

if [ ! -d "${WF_DIR}" ]; then
  printf 'no workflows dir at %s — nothing to lint\n' "${WF_DIR}"
  exit 0
fi

if ! command -v actionlint >/dev/null 2>&1; then
  cat >&2 <<EOF
actionlint not found — install with:
  bash ${ROOT}/scripts/install-actionlint.sh
  # or: brew install actionlint    (macOS)
  # or: apt install actionlint      (Debian/Ubuntu >= 24.04)
Skipping workflow lint (advisory).
EOF
  exit 0
fi

set +e
# shellcheck disable=SC2046
actionlint $(ls "${WF_DIR}"/*.yml "${WF_DIR}"/*.yaml 2>/dev/null)
RC=$?
set -e

if [ "${RC}" -ne 0 ]; then
  printf 'actionlint reported findings (exit %d)\n' "${RC}" >&2
  exit 1
fi

printf 'actionlint: OK\n'
exit 0
