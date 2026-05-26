#!/usr/bin/env bash
# bootstrap.sh — zero-to-installed KeiSeiKit on a fresh machine.
#
# Usage from inside an already-cloned repo:
#     ./bootstrap.sh                    # interactive — picks cortex profile
#     ./bootstrap.sh --profile=core     # explicit profile
#     ./bootstrap.sh --yes              # non-interactive (CI / scripts)
#
# Usage from a fresh machine (private repo, gh CLI required for clone):
#     gh auth login
#     gh repo clone KeiSeiLab/KeiSeiKit-1.0
#     cd KeiSeiKit-1.0 && ./bootstrap.sh
#
# What it does (idempotent — re-running is safe):
#     1. Detects OS (macOS / Linux)
#     2. Installs jq + rustup if missing (uses brew / apt / dnf / pacman)
#     3. Sources cargo env so a fresh shell sees PATH=$HOME/.cargo/bin
#     4. Verifies we're in a KeiSeiKit checkout (presence of install.sh)
#     5. Runs install.sh with the chosen profile
#     6. Health-checks the install via kei-doctor (best-effort)
#
# What it does NOT do (these are still YOUR responsibility):
#     - Set up SSH keys for github (use `gh auth login` first)
#     - Configure secrets per RULE 0.8 (~/.claude/secrets/.env)
#     - Activate Claude Code hooks (re-run with --activate-hooks if needed)

set -euo pipefail

# --- defaults ------------------------------------------------------------
PROFILE="${KEISEIKIT_PROFILE:-}"   # empty → wizard prompts on TTY (Wave 45)
YES_FLAG=""
EXTRA_FLAGS=()
SKIP_PREREQS=0

while [ $# -gt 0 ]; do
    case "$1" in
        --profile=*) PROFILE="${1#*=}"; shift ;;
        --profile)   PROFILE="$2"; shift 2 ;;
        --yes|-y)    YES_FLAG="--yes"; shift ;;
        --skip-prereqs) SKIP_PREREQS=1; shift ;;
        --*)         EXTRA_FLAGS+=("$1"); shift ;;
        *)           echo "unknown arg: $1" >&2; exit 2 ;;
    esac
done

# --- wizard (Wave 45) ----------------------------------------------------
# If no --profile given AND we're on a TTY, ask. Non-TTY (CI / pipe) →
# fallback to cortex for compat with v0.16 default behaviour.
prompt_profile() {
    if [ -n "$PROFILE" ]; then return 0; fi
    # Interactive iff stdin is a terminal. NOT stdout: web-install.sh tees stdout
    # to a logfile (pipe), so -t 1 is false even in an interactive curl|bash.
    # Prompts print to the terminal via tee; the menu reads from stdin.
    # Non-interactive (CI / piped, no controlling terminal) → minimal: fast,
    # no 105-crate compile, can't half-fail. Matches install.sh's own default
    # (was "cortex" here → divergent install vs direct install.sh). Opt up with
    # --profile=cortex/full-hub.
    if [ ! -t 0 ]; then PROFILE="minimal"; return 0; fi
    cat <<'WIZARD'

╔═══════════════════════════════════════════════════════════════════╗
║  KeiSeiKit Installation Wizard                                    ║
╠═══════════════════════════════════════════════════════════════════╣
║                                                                   ║
║   [1] minimal       — agents+hooks+skills only                    ║
║                       50 MB · 5 sec install                       ║
║                                                                   ║
║   [2] cortex        — + kei-cortex daemon + UI + 8 Rust crates    ║
║                       540 MB · 5 min install                      ║
║                                                                   ║
║   [3] local-mirror  — cortex + Local Forgejo + Forgejo Runner CI  ║
║                       800 MB · 10 min · push without VPN          ║
║                                                                   ║
║   [4] dashboard     — local-mirror + Project Dashboard + DBs UI   ║
║                       1 GB · 15 min · single pane of glass        ║
║                                                                   ║
║   [5] full-hub      — dashboard + Search + Docs + Backup          ║
║                       1.3 GB · 25 min · everything                ║
║                                                                   ║
║   [6] full          — every primitive in MANIFEST (53 tools)      ║
║                       1.5 GB · 15 min · power user                ║
║                                                                   ║
╚═══════════════════════════════════════════════════════════════════╝

WIZARD
    local choice=""
    while true; do
        read -r -p "Pick a profile [1-6, default=2]: " choice
        choice="${choice:-2}"
        case "$choice" in
            1) PROFILE="minimal";      break ;;
            2) PROFILE="cortex";       break ;;
            3) PROFILE="local-mirror"; break ;;
            4) PROFILE="dashboard";    break ;;
            5) PROFILE="full-hub";     break ;;
            6) PROFILE="full";         break ;;
            *) echo "invalid — pick 1, 2, 3, 4, 5, or 6" ;;
        esac
    done
    echo "[bootstrap] profile selected: $PROFILE"
    echo
}

prompt_profile

case "$PROFILE" in
    minimal|core|frontend|ops|dev|mcp|cortex|full|local-mirror|dashboard|full-hub) ;;
    *) echo "[bootstrap] unknown profile: $PROFILE" >&2
       echo "   valid: minimal cortex local-mirror dashboard full-hub full (also core/frontend/ops/dev/mcp)" >&2
       exit 2 ;;
esac

# --- helpers -------------------------------------------------------------
log()  { echo "[bootstrap] $*"; }
err()  { echo "[bootstrap] ERROR: $*" >&2; }
have() { command -v "$1" >/dev/null 2>&1; }

OS="$(uname -s)"

# --- 1. OS detection -----------------------------------------------------
case "$OS" in
    Darwin|Linux) ;;
    *) err "unsupported OS: $OS (only Darwin / Linux for now)"; exit 1 ;;
esac
log "OS: $OS"

# --- 2. install jq -------------------------------------------------------
install_jq() {
    if have jq; then return 0; fi
    log "installing jq"
    case "$OS" in
        Darwin)
            if ! have brew; then
                log "homebrew missing — installing it first (10-15 min)"
                NONINTERACTIVE=1 /bin/bash -c \
                    "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
                # macOS Apple-Silicon: add brew to PATH for this session
                if [ -d /opt/homebrew/bin ]; then export PATH="/opt/homebrew/bin:$PATH"; fi
            fi
            brew install jq
            ;;
        Linux)
            if   have apt-get;  then sudo apt-get update && sudo apt-get install -y jq
            elif have dnf;      then sudo dnf install -y jq
            elif have pacman;   then sudo pacman -S --noconfirm jq
            elif have apk;      then sudo apk add jq
            else err "no supported package manager — install jq manually"; exit 1
            fi
            ;;
    esac
}

# --- 3. install rustup ---------------------------------------------------
install_rust() {
    if have cargo && cargo --version >/dev/null 2>&1; then return 0; fi
    log "installing rustup (default toolchain: stable)"
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | \
        sh -s -- -y --default-toolchain stable --profile minimal
    # shellcheck disable=SC1091
    source "$HOME/.cargo/env"
}

if [ "$SKIP_PREREQS" = "0" ]; then
    install_jq
    install_rust
else
    log "--skip-prereqs: assuming jq + cargo already installed"
fi

# --- 4. checkout sanity check --------------------------------------------
KIT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
if [ ! -f "$KIT_DIR/install.sh" ]; then
    err "install.sh not found in $KIT_DIR — am I inside a KeiSeiKit checkout?"
    err "if not: gh repo clone KeiSeiLab/KeiSeiKit-1.0 && cd KeiSeiKit-1.0 && ./bootstrap.sh"
    exit 1
fi
log "checkout: $KIT_DIR"

# --- 5. run install ------------------------------------------------------
log "running install.sh --profile=$PROFILE $YES_FLAG ${EXTRA_FLAGS[*]:-}"
cd "$KIT_DIR"
# Defensive: invoke via `bash` not `./install.sh` because GitHub's contents
# API does NOT preserve the executable bit on `gh api -X PUT` updates
# (only the git Data API does). Older clones may have install.sh with
# mode 644 even though the source repo has it 755. `bash <file>` works
# regardless of file mode. Verified incident 2026-05-26 prod-curl test.
bash ./install.sh --profile="$PROFILE" $YES_FLAG "${EXTRA_FLAGS[@]:+${EXTRA_FLAGS[@]}}"

# --- 6. post-install verification ----------------------------------------
KEI_BIN="$HOME/.claude/agents/_primitives/_rust/target/release"
log "==========================================================================="
log "post-install health check"
log "==========================================================================="

if [ -x "$KEI_BIN/kei-doctor" ]; then
    "$KEI_BIN/kei-doctor" 2>&1 | head -25 || true
elif [ -x "$HOME/.claude/agents/_primitives/kei-doctor.sh" ]; then
    "$HOME/.claude/agents/_primitives/kei-doctor.sh" 2>&1 | head -25 || true
else
    log "(kei-doctor not installed — pick a profile that includes it: core/cortex/full)"
fi

log ""
log "==========================================================================="
log "DONE — KeiSeiKit installed (profile: $PROFILE)"
log "==========================================================================="

# v0.45: post-install onboarding wizard.
# Auto-triggers if stdin is a TTY (real terminal). Wizard itself re-checks
# and exits cleanly if non-interactive — so curl|bash one-liner runs work too.
ONBOARD_SH="$HOME/.claude/scripts/kei-onboard.sh"
if [ -x "$ONBOARD_SH" ] && [ -t 0 ] && [ "${KEI_NO_ONBOARD:-0}" != "1" ]; then
  log ""
  log "Starting post-install onboarding (pick primary CLI + wire MCP)..."
  log "Skip with KEI_NO_ONBOARD=1; re-run anytime with 'kei onboard'."
  log ""
  "$ONBOARD_SH" || log "(onboarding exited non-zero; re-run with 'kei onboard')"
else
  log ""
  log "Post-install wizard skipped (no TTY or KEI_NO_ONBOARD=1)."
  log "Run interactively to configure primary CLI:"
  log "  kei onboard           # full wizard"
  log "  kei pick              # just pick primary"
  log "  kei mcp-wire          # wire MCP into installed CLIs"
fi
log ""
log "Next steps:"
log "  - Open a new shell so PATH picks up ~/.cargo/bin and the kei-* binaries."
log "  - Or source the rc file the installer wrote (Bash: ~/.bashrc, Zsh: ~/.zshrc)."
log "  - Run kei-doctor for a full health diagnostic."
log "  - For cortex profile: run /cortex-setup inside Claude Code."
log "  - For sleep layer: run /sleep-setup inside Claude Code."
