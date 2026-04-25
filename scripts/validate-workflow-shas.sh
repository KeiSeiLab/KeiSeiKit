#!/bin/sh
# validate-workflow-shas.sh — verify every `uses: <repo>@<sha40>` pin in the
# repo's workflow files resolves upstream. Closes v0.20.1 incident class.
# Hard-fails (exit 1) only on 404 / 422 from GitHub commits API.
# Trailing comment `# validate-workflow-shas: skip=<reason>` skips a line.
# Tag refs (@v4, @stable) are policy decisions and not checked.
# GITHUB_TOKEN (optional) raises the 60/hr anonymous rate limit.

set -eu

ROOT=$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)

SCAN_FILES=""
for f in "${ROOT}/.github/workflows"/*.yml \
         "${ROOT}/.github/workflows"/*.yaml \
         "${ROOT}/.github/dependabot.yml" ; do
  [ -f "${f}" ] && SCAN_FILES="${SCAN_FILES} ${f}"
done

[ -z "${SCAN_FILES}" ] && { printf 'no workflow files under %s/.github\n' "${ROOT}"; exit 0; }
command -v curl >/dev/null 2>&1 || { printf 'curl not found\n' >&2; exit 2; }

# shellcheck disable=SC2086
PINS=$(grep -hE '^[[:space:]]*(-[[:space:]]*)?uses:[[:space:]]*[a-zA-Z0-9_.-]+/[a-zA-Z0-9_.-]+@[a-f0-9]{40}' ${SCAN_FILES} 2>/dev/null || true)
[ -z "${PINS}" ] && { printf 'no SHA-pinned `uses:` lines\n'; exit 0; }

TMP=$(mktemp)
trap 'rm -f "${TMP}"' EXIT INT TERM

# Token sanity-probe: invalid token => unauthenticated fallback.
AUTH=""
if [ -n "${GITHUB_TOKEN:-}" ]; then
  P=$(curl -sS -o /dev/null -w '%{http_code}' \
    -H "Authorization: Bearer ${GITHUB_TOKEN}" \
    -H "Accept: application/vnd.github+json" \
    https://api.github.com/rate_limit 2>/dev/null || printf 000)
  if [ "${P}" = "200" ]; then AUTH="Authorization: Bearer ${GITHUB_TOKEN}"
  else printf '[info] GITHUB_TOKEN probe=%s — anonymous (60/hr)\n' "${P}" >&2; fi
fi

check_sha() {
  REPO=$1; SHA=$2; SHORT=$(printf '%s' "${SHA}" | cut -c1-7)
  URL="https://api.github.com/repos/${REPO}/commits/${SHA}"
  set +e
  if [ -n "${AUTH}" ]; then
    C=$(curl -sS -o /dev/null -w '%{http_code}' -H "${AUTH}" -H "Accept: application/vnd.github+json" "${URL}")
  else
    C=$(curl -sS -o /dev/null -w '%{http_code}' -H "Accept: application/vnd.github+json" "${URL}")
  fi
  RC=$?
  set -e
  if [ "${RC}" -ne 0 ]; then
    printf '[UNVERIFIED: %s@%s — curl rc=%d]\n' "${REPO}" "${SHORT}" "${RC}"; echo U >> "${TMP}"; return 0
  fi
  case "${C}" in
    200) printf 'SHA OK: %s@%s\n' "${REPO}" "${SHORT}"; echo K >> "${TMP}" ;;
    404) printf 'SHA MISSING: %s@%s — repo not found (404)\n' "${REPO}" "${SHA}" >&2; echo M >> "${TMP}" ;;
    422) printf 'SHA MISSING: %s@%s — no matching commit (422)\n' "${REPO}" "${SHA}" >&2; echo M >> "${TMP}" ;;
    403) printf '[UNVERIFIED: %s@%s — 403 (rate-limited)]\n' "${REPO}" "${SHORT}"; echo U >> "${TMP}" ;;
    *)   printf '[UNVERIFIED: %s@%s — HTTP %s]\n' "${REPO}" "${SHORT}" "${C}"; echo U >> "${TMP}" ;;
  esac
}

parse_line() {
  L=$1
  case "${L}" in
    *"validate-workflow-shas: skip="*)
      printf 'SKIP    %s\n' "$(printf '%s' "${L}" | sed 's/^[[:space:]]*//')"
      echo S >> "${TMP}"; return 0 ;;
  esac
  T=$(printf '%s' "${L}" | sed 's/^[[:space:]]*-\{0,1\}[[:space:]]*uses:[[:space:]]*//')
  REF=$(printf '%s' "${T}" | sed 's/[[:space:]]*#.*$//' | sed 's/[[:space:]]*$//')
  REPO=$(printf '%s' "${REF}" | sed 's/@.*$//')
  SHA=$(printf '%s' "${REF}" | sed 's/^[^@]*@//')
  if [ ${#SHA} -ne 40 ]; then
    printf 'SKIP-BADSHAPE %s (len=%d)\n' "${REF}" "${#SHA}"; echo U >> "${TMP}"; return 0
  fi
  check_sha "${REPO}" "${SHA}"
}

printf '%s\n' "${PINS}" | while IFS= read -r LINE; do
  [ -n "${LINE}" ] && parse_line "${LINE}"
done

count_tok() {
  C=$(grep -c "^$1\$" "${TMP}" 2>/dev/null || printf 0)
  C=$(printf '%s' "${C}" | tr -cd '0-9'); [ -z "${C}" ] && C=0
  printf '%s' "${C}"
}

OK_C=$(count_tok K); M_C=$(count_tok M); U_C=$(count_tok U); S_C=$(count_tok S)
T_C=$((OK_C + M_C + U_C + S_C))

printf '\nSummary: %d checked | %d OK | %d MISSING | %d UNVERIFIED | %d SKIPPED\n' \
  "${T_C}" "${OK_C}" "${M_C}" "${U_C}" "${S_C}"

[ "${M_C}" -gt 0 ] && exit 1

# v0.21.1 D3 — distinguish "all verified" from "rate-limited, we couldn't
# check". If there are UNVERIFIED pins AND we ran without GITHUB_TOKEN,
# treat this as a hard failure so CI surfaces the gap instead of silently
# returning green. If we DID have a token (even if rate-limited anyway),
# exit 0 — we tried, that's the best we can do.
if [ "${U_C}" -gt 0 ] && [ -z "${AUTH}" ]; then
  printf 'ERROR: %d pins UNVERIFIED without GITHUB_TOKEN. Re-run with\n' "${U_C}" >&2
  printf '       GITHUB_TOKEN=<pat> in env to complete verification.\n' >&2
  exit 2
fi

exit 0
