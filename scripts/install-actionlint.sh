#!/bin/sh
# install-actionlint.sh — idempotent installer for rhysd/actionlint.
# Detects OS+arch, downloads the pinned release tarball to ~/.local/bin/actionlint.
# No-op if the binary is already on PATH. On macOS with Homebrew available and
# no local binary, suggests `brew install actionlint` as a faster alternative.
#
# Version pinned after WebFetch verification 2026-04-22.
# [VERIFIED: https://github.com/rhysd/actionlint/releases/tag/v1.7.12]
#
# v0.21.1 H1: SHA-256 verification added for every downloaded tarball.
# The hashes below were sourced from the upstream `checksums.txt` on the
# same release page. If your fork bumps the version, regenerate via:
#
#     curl -fsSL https://github.com/rhysd/actionlint/releases/download/v<N>/checksums.txt
#
# and paste the four darwin-amd64 / darwin-arm64 / linux-amd64 / linux-arm64
# rows into the SHA256_* variables below.
#
# If a hash is set to the literal string `SKIP`, the verification step
# prints a WARNING and proceeds — useful for local dev when the upstream
# checksums page is temporarily unreachable. CI should treat `SKIP` as a
# pre-commit failure (audit hygiene).
#
# [VERIFIED 2026-04-22 via curl https://github.com/rhysd/actionlint/releases/download/v1.7.12/actionlint_1.7.12_checksums.txt]
# The four SHA256_* values below are pinned to upstream checksums.txt rows.

set -eu

ACTIONLINT_VERSION="1.7.12"
INSTALL_DIR="${HOME}/.local/bin"
BIN="${INSTALL_DIR}/actionlint"

# Per (OS, ARCH) SHA-256 hashes. See comment block above.
# [VERIFIED: https://github.com/rhysd/actionlint/releases/download/v1.7.12/actionlint_1.7.12_checksums.txt]
SHA256_DARWIN_AMD64="5b44c3bc2255115c9b69e30efc0fecdf498fdb63c5d58e17084fd5f16324c644"
SHA256_DARWIN_ARM64="aba9ced2dee8d27fecca3dc7feb1a7f9a52caefa1eb46f3271ea66b6e0e6953f"
SHA256_LINUX_AMD64="8aca8db96f1b94770f1b0d72b6dddcb1ebb8123cb3712530b08cc387b349a3d8"
SHA256_LINUX_ARM64="325e971b6ba9bfa504672e29be93c24981eeb1c07576d730e9f7c8805afff0c6"

if command -v actionlint >/dev/null 2>&1; then
  printf 'actionlint already on PATH: %s\n' "$(command -v actionlint)"
  exit 0
fi

OS=$(uname -s | tr '[:upper:]' '[:lower:]')
ARCH_RAW=$(uname -m)
case "${ARCH_RAW}" in
  x86_64|amd64) ARCH="amd64" ;;
  arm64|aarch64) ARCH="arm64" ;;
  *) printf 'unsupported arch: %s\n' "${ARCH_RAW}" >&2; exit 2 ;;
esac

case "${OS}" in
  darwin|linux) : ;;
  *) printf 'unsupported os: %s\n' "${OS}" >&2; exit 2 ;;
esac

# Select the expected hash for this platform. Env override wins.
EXPECTED_SHA="${ACTIONLINT_SHA256_OVERRIDE:-}"
if [ -z "${EXPECTED_SHA}" ]; then
  case "${OS}_${ARCH}" in
    darwin_amd64) EXPECTED_SHA="${SHA256_DARWIN_AMD64}" ;;
    darwin_arm64) EXPECTED_SHA="${SHA256_DARWIN_ARM64}" ;;
    linux_amd64)  EXPECTED_SHA="${SHA256_LINUX_AMD64}" ;;
    linux_arm64)  EXPECTED_SHA="${SHA256_LINUX_ARM64}" ;;
    *) EXPECTED_SHA="SKIP" ;;
  esac
fi

# Homebrew fast-path on macOS.
if [ "${OS}" = "darwin" ] && command -v brew >/dev/null 2>&1; then
  printf 'Homebrew detected. Fast path:\n  brew install actionlint\n'
  printf 'Falling through to tarball install (~/.local/bin) anyway.\n'
fi

ASSET="actionlint_${ACTIONLINT_VERSION}_${OS}_${ARCH}.tar.gz"
URL="https://github.com/rhysd/actionlint/releases/download/v${ACTIONLINT_VERSION}/${ASSET}"

mkdir -p "${INSTALL_DIR}"
TMP=$(mktemp -d)
trap 'rm -rf "${TMP}"' EXIT INT TERM

printf 'downloading %s\n' "${URL}"
if command -v curl >/dev/null 2>&1; then
  curl -fsSL -o "${TMP}/${ASSET}" "${URL}"
elif command -v wget >/dev/null 2>&1; then
  wget -qO "${TMP}/${ASSET}" "${URL}"
else
  printf 'neither curl nor wget is installed\n' >&2
  exit 2
fi

# v0.21.1 H1 — sha256 verify step.
if [ "${EXPECTED_SHA}" = "SKIP" ]; then
  printf 'WARNING: no SHA-256 pinned for %s_%s — skipping integrity check.\n' \
    "${OS}" "${ARCH}" >&2
  printf '  Set ACTIONLINT_SHA256_OVERRIDE=<hash> or update %s.\n' "$0" >&2
else
  if command -v shasum >/dev/null 2>&1; then
    ACTUAL_SHA=$(shasum -a 256 "${TMP}/${ASSET}" | awk '{print $1}')
  elif command -v sha256sum >/dev/null 2>&1; then
    ACTUAL_SHA=$(sha256sum "${TMP}/${ASSET}" | awk '{print $1}')
  else
    printf 'neither shasum nor sha256sum is installed — refusing to install unverified binary\n' >&2
    exit 2
  fi
  if [ "${ACTUAL_SHA}" != "${EXPECTED_SHA}" ]; then
    printf 'SHA-256 MISMATCH for %s\n  expected: %s\n  actual:   %s\n' \
      "${ASSET}" "${EXPECTED_SHA}" "${ACTUAL_SHA}" >&2
    exit 2
  fi
  printf 'SHA-256 verified: %s\n' "${ACTUAL_SHA}"
fi

tar -xzf "${TMP}/${ASSET}" -C "${TMP}" actionlint
install -m 0755 "${TMP}/actionlint" "${BIN}"

printf 'installed: %s\n' "${BIN}"
case ":${PATH}:" in
  *:"${INSTALL_DIR}":*) : ;;
  *) printf 'note: %s is not on PATH — add it to your shell profile.\n' "${INSTALL_DIR}" ;;
esac
