# shellcheck shell=bash
# lib-prereqs.sh — hard + soft prerequisite checks.
#
# HARD: cargo, jq. SOFT: deps based on the primitives that will be installed.
# A profile-aware soft-warn: only check deps for primitives actually in scope.
#
# Requires: err / warn / say from lib-log.sh.
# Requires: profile_members from lib-profile.sh.
# Reads globals: $PROFILE, $CUSTOM_PRIMS, $MANIFEST.
# Sets global:  $PROFILE_PRIMS (space-separated primitive names).

# Does the resolved profile contain at least one rust primitive? Only then
# is a functional cargo toolchain a HARD requirement. Profiles like minimal
# (0 primitives) and shell-only customs build nothing and must install
# without Rust — README promises "minimal — NO Rust compile". Requires
# PROFILE_PRIMS already resolved (resolve_profile_prims, called by install.sh
# and at the top of check_prereqs).
_profile_needs_cargo() {
  local p kind
  for p in ${PROFILE_PRIMS:-}; do
    kind="$(primitive_field "$p" kind 2>/dev/null || true)"
    [ "$kind" = "rust" ] && return 0
  done
  return 1
}

# Hard checks: cargo + jq, both always required.
#   cargo — the agent assembler (build_assembler) compiles a small rust binary
#           to generate the agent .md files on EVERY profile, so cargo is
#           non-negotiable. The heavy 105-crate substrate *workspace* build is
#           a separate concern: install.sh auto-sets KEI_SKIP_RUST for profiles
#           with no rust primitives so minimal stays fast (assembler only).
#   jq    — the installed hooks parse Claude Code JSON via jq and would abort
#           tool calls without it.
check_hard_prereqs() {
  say "checking prerequisites"
  if ! command -v cargo >/dev/null 2>&1; then
    err "cargo not found. Install Rust: https://rustup.rs/"
    exit 1
  fi
  if ! cargo --version >/dev/null 2>&1; then
    err "cargo is installed but not functional. Run: rustup default stable"
    exit 1
  fi
  if ! command -v jq >/dev/null 2>&1; then
    err "jq not found. jq is REQUIRED on any machine that will activate the"
    err "KeiSeiKit hooks — without it the hooks become dead weight and would"
    err "otherwise abort Claude Code's Edit/Write/Bash tool calls. Install it:"
    err "  brew install jq   (macOS)"
    err "  apt install jq    (Debian/Ubuntu)"
    exit 1
  fi
}

# Resolve primitive list for the current profile (or CUSTOM_PRIMS if custom)
# into PROFILE_PRIMS. Does not exit.
resolve_profile_prims() {
  if [ "$PROFILE" = "custom" ]; then
    PROFILE_PRIMS="$(echo "$CUSTOM_PRIMS" | tr ',' ' ')"
  else
    PROFILE_PRIMS="$(profile_members "$PROFILE" 2>/dev/null || true)"
  fi
}

# Scan PROFILE_PRIMS and echo a space-separated list of tool-need flags:
# pandoc playwright sqlite hcloud vultr yq python3 ffmpeg node pnpm.
_soft_dep_flags() {
  local needs_pandoc=0 needs_playwright=0 needs_sqlite=0
  local needs_hcloud=0 needs_vultr=0 needs_yq=0
  local needs_python=0 needs_ffmpeg=0 needs_node=0 needs_pnpm=0 p
  for p in $PROFILE_PRIMS; do
    case "$p" in
      tomd)                                   needs_pandoc=1 ;;
      design-scrape|live-preview|mock-render) needs_playwright=1 ;;
      kei-ledger|kei-migrate)                 needs_sqlite=1 ;;
      provision-hetzner)                      needs_hcloud=1 ;;
      provision-vultr)                        needs_vultr=1 ;;
      kei-ci-lint)                            needs_yq=1 ;;
      kei-cortex)                             needs_python=1; needs_ffmpeg=1 ;;
      cortex-ui)                              needs_node=1;   needs_pnpm=1   ;;
    esac
  done
  echo "$needs_pandoc $needs_playwright $needs_sqlite $needs_hcloud $needs_vultr $needs_yq $needs_python $needs_ffmpeg $needs_node $needs_pnpm"
}

# Soft checks: only warn for tools needed by primitives actually being installed.
check_soft_prereqs() {
  local n_pandoc n_playwright n_sqlite n_hcloud n_vultr n_yq
  local n_python n_ffmpeg n_node n_pnpm
  read -r n_pandoc n_playwright n_sqlite n_hcloud n_vultr n_yq n_python n_ffmpeg n_node n_pnpm <<< "$(_soft_dep_flags)"
  if [ "$n_pandoc" = "1" ] && ! command -v pandoc >/dev/null 2>&1; then
    warn "pandoc not found — tomd primitive will fail on .docx/.pptx. Install: brew install pandoc"
  fi
  if [ "$n_playwright" = "1" ] && ! command -v playwright >/dev/null 2>&1 && ! command -v npx >/dev/null 2>&1; then
    warn "playwright/npx not found — frontend primitives need them. Install: npm i -g playwright && playwright install chromium"
  fi
  if [ "$n_sqlite" = "1" ] && ! command -v sqlite3 >/dev/null 2>&1; then
    warn "sqlite3 CLI not found — kei-ledger/kei-migrate work without it (rusqlite embedded). Install for manual DB inspection: brew install sqlite"
  fi
  if [ "$n_hcloud" = "1" ] && ! command -v hcloud >/dev/null 2>&1; then
    warn "hcloud CLI not found — provision-hetzner requires it. Install: brew install hcloud"
  fi
  if [ "$n_vultr" = "1" ] && ! command -v vultr-cli >/dev/null 2>&1; then
    warn "vultr-cli not found — provision-vultr requires it. Install: brew install vultr/vultr-cli/vultr-cli"
  fi
  if [ "$n_yq" = "1" ] && ! command -v yq >/dev/null 2>&1; then
    warn "yq not found — kei-ci-lint requires yq v4+ (mikefarah/yq). Install: brew install yq"
  fi
  if [ "$n_python" = "1" ]; then
    if ! command -v python3 >/dev/null 2>&1; then
      warn "python3 not found — kei-cortex whisper_worker.py subprocess cannot launch. Install Python >=3.9: brew install python"
    elif ! command -v pip3 >/dev/null 2>&1; then
      warn "pip3 not found — needed for 'pip install -r scripts/requirements.txt' (faster-whisper). Install: python3 -m ensurepip --upgrade"
    fi
  fi
  if [ "$n_ffmpeg" = "1" ] && ! command -v ffmpeg >/dev/null 2>&1; then
    warn "ffmpeg not found on PATH — faster-whisper audio demux will fail. Install: brew install ffmpeg"
  fi
  if [ "$n_node" = "1" ] && ! command -v node >/dev/null 2>&1; then
    warn "node not found — cortex-ui (Svelte/Vite) build needs node>=18. Install: brew install node"
  fi
  if [ "$n_pnpm" = "1" ] && ! command -v pnpm >/dev/null 2>&1; then
    warn "pnpm not found — cortex-ui uses pnpm for install/build. Install: npm i -g pnpm"
  fi
}

# Top-level orchestrator: resolve profile first (idempotent — install.sh
# already resolved it before the no-execute/skip-prereqs branches), so the
# conditional cargo gate in check_hard_prereqs can see PROFILE_PRIMS; then
# hard checks (exit on miss); then soft warnings.
check_prereqs() {
  resolve_profile_prims
  check_hard_prereqs
  check_soft_prereqs
}
