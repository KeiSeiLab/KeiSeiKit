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
#     gh repo clone KeiSei84/KeiSeiKit-1.0
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
PROFILE="${KEISEIKIT_PROFILE:-cortex}"
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

case "$PROFILE" in
    minimal|core|frontend|ops|dev|mcp|cortex|full) ;;
    *) echo "[bootstrap] unknown profile: $PROFILE (valid: minimal core frontend ops dev mcp cortex full)" >&2; exit 2 ;;
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
    err "if not: gh repo clone KeiSei84/KeiSeiKit-1.0 && cd KeiSeiKit-1.0 && ./bootstrap.sh"
    exit 1
fi
log "checkout: $KIT_DIR"

# --- 5. run install ------------------------------------------------------
log "running ./install.sh --profile=$PROFILE $YES_FLAG ${EXTRA_FLAGS[*]:-}"
cd "$KIT_DIR"
./install.sh --profile="$PROFILE" $YES_FLAG "${EXTRA_FLAGS[@]:+${EXTRA_FLAGS[@]}}"

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
log ""
log "Next steps:"
log "  - Open a new shell so PATH picks up ~/.cargo/bin and the kei-* binaries."
log "  - Or source the rc file the installer wrote (Bash: ~/.bashrc, Zsh: ~/.zshrc)."
log "  - Run kei-doctor for a full health diagnostic."
log "  - For cortex profile: run /cortex-setup inside Claude Code."
log "  - For sleep layer: run /sleep-setup inside Claude Code."
