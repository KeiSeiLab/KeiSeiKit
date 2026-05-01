#!/usr/bin/env bash
# kei-sleep-setup.sh — KeiSeiKit v0.11 sleep-sync first-time wizard.
# Generates deploy key, scaffolds sync-repo, writes env refs (RULE 0.8).
# Idempotent. Invoked by `/sleep-setup` skill or `install.sh --with-sleep-sync`.
set -eu

KEI_HOME="${HOME}/.claude"
SECRETS_FILE="${KEI_HOME}/secrets/.env"
SSH_KEY="${HOME}/.ssh/keisei-memory-sync"
REPO_PATH="${KEI_HOME}/memory/sync-repo"

say()  { printf '[sleep-setup] %s\n' "$*"; }
warn() { printf '[sleep-setup] warn: %s\n' "$*" >&2; }
err()  { printf '[sleep-setup] error: %s\n' "$*" >&2; }

validate_ssh_url() {
    printf '%s' "$1" | grep -Eq '^git@[A-Za-z0-9._-]+:[A-Za-z0-9._/-]+\.git$'
}

url_host() {
    printf '%s' "$1" | sed -E 's/^git@([^:]+):.*/\1/'
}

prompt_repo_url() {
    local url="${KEI_MEMORY_REPO_URL:-}"
    if [ -n "$url" ]; then
        say "using KEI_MEMORY_REPO_URL from environment"
    elif [ -t 0 ]; then
        printf '\nMemory repo SSH URL (e.g. git@github.com:you/kei-memory.git):\n  > '
        read -r url
    else
        err "no repo URL provided and no TTY to prompt; set KEI_MEMORY_REPO_URL"
        exit 1
    fi
    if ! validate_ssh_url "$url"; then
        err "invalid SSH URL format: $url"
        err "expected: git@<host>:<org>/<repo>.git"
        exit 1
    fi
    printf '%s' "$url"
}

ensure_ssh_key() {
    mkdir -p "$(dirname "$SSH_KEY")"
    chmod 700 "$(dirname "$SSH_KEY")" 2>/dev/null || true
    if [ -f "$SSH_KEY" ] && [ -f "${SSH_KEY}.pub" ]; then
        say "deploy key already exists at $SSH_KEY (skipping)"
        return 0
    fi
    say "generating ed25519 deploy key at $SSH_KEY"
    ssh-keygen -t ed25519 -f "$SSH_KEY" -N '' -C 'keisei-memory-sync' >/dev/null
    chmod 600 "$SSH_KEY"
}

show_deploy_key_instructions() {
    local url="$1"
    printf '\n==============================================================\n'
    printf ' ADD THIS AS A DEPLOY KEY (WRITE ACCESS) TO: %s\n' "$url"
    printf '==============================================================\n\n'
    cat "${SSH_KEY}.pub"
    printf '\nFingerprint: '
    ssh-keygen -lf "${SSH_KEY}.pub" 2>/dev/null || true
    printf '\nGitHub:    Settings -> Deploy keys -> Add ("Allow write access")\n'
    printf 'GitLab:    Settings -> Repository -> Deploy keys -> Enable write\n'
    printf 'Bitbucket: Repository settings -> Access keys -> Add (write)\n'
    printf 'Forgejo:   Settings -> Deploy Keys -> Add (allow write)\n'
    printf '==============================================================\n\n'
}

init_sync_repo() {
    local url="$1"
    mkdir -p "$REPO_PATH"
    if [ -d "${REPO_PATH}/.git" ]; then
        say "sync-repo already initialized at $REPO_PATH"
        return 0
    fi
    say "cloning $url → $REPO_PATH (shallow, may fail if repo empty — will init instead)"
    if GIT_SSH_COMMAND="ssh -i $SSH_KEY -o StrictHostKeyChecking=accept-new" \
       git clone --depth 1 "$url" "$REPO_PATH" 2>/dev/null; then
        say "cloned existing repo"
    else
        say "clone failed (empty repo?) — initializing local and setting remote"
        rm -rf "$REPO_PATH"
        mkdir -p "$REPO_PATH"
        ( cd "$REPO_PATH" && git init -q -b main && git remote add origin "$url" )
    fi
}

scaffold_repo_structure() {
    local url="$1"
    cd "$REPO_PATH"
    mkdir -p traces reports
    [ -f README.md ]        || write_readme
    [ -f .gitignore ]       || printf 'target/\nnode_modules/\n.DS_Store\n*.swp\n*.swo\n' > .gitignore
    [ -f backlog.md ]       || write_backlog
    [ -f .keisei-sync.toml ] || write_sync_config "$url"
}

write_readme() {
    cat > README.md <<'EOF'
# KeiSeiKit memory store

Append-only store for KeiSeiKit session traces + nightly REM reports.
Managed by kei-sleep-sync; do not hand-edit.

- traces/  — session JSONL pushed after each Claude Code session
- reports/ — nightly reports written by a cloud agent on /schedule
- backlog.md — recurring patterns flagged for your review
EOF
}

write_backlog() {
    cat > backlog.md <<'EOF'
# REM backlog — recurring patterns

Nightly consolidation prepends dated blocks when >=3 patterns recur.
Pop entries manually after review.

<!-- populated by the cloud agent -->
EOF
}

write_sync_config() {
    cat > .keisei-sync.toml <<EOF
# KeiSeiKit sleep-sync config (per-repo)
repo_url = "$1"
push_on_session_end = true
branch = "main"
commit_prefix = "memory"
EOF
}

write_env_refs() {
    local url="$1"
    mkdir -p "$(dirname "$SECRETS_FILE")"
    chmod 700 "$(dirname "$SECRETS_FILE")" 2>/dev/null || true
    touch "$SECRETS_FILE"
    chmod 600 "$SECRETS_FILE"
    # Remove any prior KEI_MEMORY_* lines (idempotent update).
    local tmp
    tmp="$(mktemp)"
    grep -vE '^(KEI_MEMORY_REPO_URL|KEI_MEMORY_REPO_PATH|KEI_MEMORY_SSH_KEY)=' \
        "$SECRETS_FILE" > "$tmp" 2>/dev/null || true
    cat >> "$tmp" <<EOF
KEI_MEMORY_REPO_URL=$url
KEI_MEMORY_REPO_PATH=$REPO_PATH
KEI_MEMORY_SSH_KEY=$SSH_KEY
EOF
    mv "$tmp" "$SECRETS_FILE"
    chmod 600 "$SECRETS_FILE"
    say "wrote env refs to $SECRETS_FILE"
}

test_ssh_auth() {
    local url="$1"
    local host
    host="$(url_host "$url")"
    say "testing SSH auth to $host"
    # ssh -T on git hosts returns non-zero even on success; grep the banner.
    local out
    out="$(ssh -i "$SSH_KEY" -o StrictHostKeyChecking=accept-new \
              -o BatchMode=yes -T "git@$host" 2>&1 || true)"
    if printf '%s' "$out" | grep -Eiq '(successfully authenticated|welcome|you.?ve|does not provide shell access)'; then
        say "SSH auth OK ($host)"
        return 0
    fi
    warn "SSH auth to $host did not return a known success banner"
    warn "server said: $(printf '%s' "$out" | head -n1)"
    warn "if the deploy key was just added it may need 30-60s to propagate"
    return 1
}

main() {
    say "KeiSeiKit v0.11 sleep-sync setup"
    local url
    url="$(prompt_repo_url)"
    ensure_ssh_key
    show_deploy_key_instructions "$url"
    if [ -t 0 ]; then
        printf 'Deploy key added to the repo? Press ENTER to continue, Ctrl-C to abort.\n'
        read -r _ || true
    fi
    init_sync_repo "$url"
    scaffold_repo_structure "$url"
    write_env_refs "$url"
    test_ssh_auth "$url" || warn "continuing despite auth test warning"
    echo
    say "setup complete"
    say "next: run /sleep-setup in Claude Code to register a nightly /schedule trigger"
}

main "$@"
