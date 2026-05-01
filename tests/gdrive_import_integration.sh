#!/usr/bin/env bash
# gdrive_import_integration.sh — wave46-i4 integration tests (PLAN.md).
# Mocked rclone + Forgejo + gitleaks + git push. bash 3.2.
# SKIP=77 on missing prereqs; FAIL=1 on assertion miss; teardown via trap EXIT.
set -u
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
FIX="$ROOT/tests/fixtures/gdrive_import"
BASES="$ROOT ${HOME}/Projects/KeiSeiKit"
WIZ=""; GIM=""
for b in $BASES; do
    [ -z "$WIZ" ] && [ -f "$b/_templates/drive-import-wizard.sh.tmpl" ] && WIZ="$b/_templates/drive-import-wizard.sh.tmpl"
    [ -z "$GIM" ] && [ -f "$b/_templates/drive-import-gitignore-map.txt" ] && GIM="$b/_templates/drive-import-gitignore-map.txt"
done
TMPDIR=""; FORGEJO_PID=""; PASS=0; FAIL=0; FAILED_LIST=""

info()      { printf '%s\n' "$*"; }
fail_msg()  { printf 'FAIL: %s\n' "$*" >&2; FAIL=$((FAIL+1)); FAILED_LIST="${FAILED_LIST}${1}|"; }
ok_msg()    { printf 'PASS: %s\n' "$*"; PASS=$((PASS+1)); }
assert_eq() { [ "$1" = "$2" ] || { fail_msg "$3 (got '$1' want '$2')"; return 1; }; return 0; }
skip_all()  { printf 'SKIP: %s\n' "$*" >&2; exit 77; }

locate_binary() {
    command -v kei-gdrive-import >/dev/null 2>&1 && { printf 'kei-gdrive-import'; return 0; }
    local b f
    for b in $BASES; do
        for f in release debug; do
            [ -x "$b/_primitives/_rust/target/$f/kei-gdrive-import" ] \
                && { printf '%s' "$b/_primitives/_rust/target/$f/kei-gdrive-import"; return 0; }
        done
    done
    return 1
}

materialise_with_secret_fixture() {
    local t="$1"
    mkdir -p "$t"
    printf 'KEY="%s%s%s"\n' "AK" "IA" "0123456789ABCDEF" > "$t/leaked.py"
    printf '[package]\nname="with-secret"\nversion="0.1.0"\nedition="2021"\n' > "$t/Cargo.toml"
}
materialise_already_repo_fixture() {
    local t="$1/already-imported"
    mkdir -p "$t/.git"
    printf 'ref: refs/heads/main\n' > "$t/.git/HEAD"
}
materialise_fake_home() {
    local h="$1"
    mkdir -p "$h/.claude/secrets" "$h/.config/rclone"
    : > "$h/.config/rclone/rclone.conf"
    cat > "$h/.claude/secrets/.env" <<EOF
RCLONE_CONFIG=$h/.config/rclone/rclone.conf
KEI_DRIVE_REMOTE=mockdrive
KEI_FORGEJO_USER=tester
KEI_FORGEJO_TOKEN=fake-token-for-tests
KEI_FORGEJO_URL=http://127.0.0.1:${MOCK_FORGEJO_PORT:-3001}
EOF
}

stage_mock_bins() {
    mkdir -p "$TMPDIR/bin"
    cp "$FIX/mock-rclone" "$TMPDIR/bin/rclone"
    cp "$FIX/mock-gitleaks" "$TMPDIR/bin/gitleaks"
    cp "$FIX/mock-git" "$TMPDIR/bin/git"
    [ -x "$BIN" ] && cp "$BIN" "$TMPDIR/bin/kei-gdrive-import"
    chmod +x "$TMPDIR/bin/"*
}

setup() {
    TMPDIR="$(mktemp -d -t kei-gdrive-it.XXXXXX)" || skip_all "mktemp failed"
    cp -R "$FIX/projects" "$TMPDIR/projects"
    materialise_already_repo_fixture "$TMPDIR/projects"
    materialise_with_secret_fixture  "$TMPDIR/projects/with-secret"
    chmod +x "$FIX"/mock-* "$FIX/mock-forgejo-server.sh" 2>/dev/null || true
    stage_mock_bins
    materialise_fake_home "$TMPDIR/fake-home"
    export MOCK_RCLONE_FIXTURE_ROOT="$TMPDIR/projects"
    export MOCK_REAL_GIT="$(command -v git 2>/dev/null || echo /usr/bin/git)"
    export PATH="$TMPDIR/bin:$PATH"
    mkdir -p "$TMPDIR/_templates"
    [ -n "$GIM" ] && cp "$GIM" "$TMPDIR/_templates/drive-import-gitignore-map.txt"
    info "setup: TMPDIR=$TMPDIR"
}

start_mock_forgejo() {
    export MOCK_FORGEJO_PORT="${MOCK_FORGEJO_PORT:-3001}"
    if curl -sf --max-time 1 "http://127.0.0.1:$MOCK_FORGEJO_PORT/api/v1/version" >/dev/null 2>&1; then
        FORGEJO_PID="external"
        info "port $MOCK_FORGEJO_PORT serves Forgejo already; reusing (test8 OK-row may skip)"
        return 0
    fi
    command -v nc >/dev/null 2>&1 || { info "no netcat — wizard tests skip"; return 0; }
    export MOCK_FORGEJO_LOG="$TMPDIR/forgejo.log"  MOCK_FORGEJO_FIFO="$TMPDIR/forgejo.fifo"
    "$FIX/mock-forgejo-server.sh" >/dev/null 2>&1 &
    FORGEJO_PID=$!; sleep 0.4
}

teardown() {
    case "${FORGEJO_PID:-}" in [0-9]*) kill "$FORGEJO_PID" 2>/dev/null || true ;; esac
    [ -n "${KEI_DEBUG_KEEP:-}" ] || { [ -n "${TMPDIR:-}" ] && [ -d "$TMPDIR" ] && rm -rf "$TMPDIR"; }
    rm -rf /tmp/kei-gdrive-import/*with-secret_* /tmp/kei-gdrive-import/*rust-app_* 2>/dev/null || true
}
trap teardown EXIT

run_wizard() { HOME="$TMPDIR/fake-home" KIT_DIR="$TMPDIR" bash "$WIZ" "$@"; }
jq_field() { printf '%s' "$1" | jq -r "$2" 2>/dev/null || echo X; }

test1_classify_rust_app() {
    local out; out="$($BIN classify "$TMPDIR/projects/rust-app" 2>/dev/null)"
    local v; v="$(jq_field "$out" .verdict)"
    local l; l="$(jq_field "$out" .primary_lang)"
    local s; s="$(jq_field "$out" .score)"
    assert_eq "$v" "PROJECT" "test1: verdict" || return 1
    assert_eq "$l" "rust"    "test1: primary_lang" || return 1
    [ "$s" -ge 15 ] || { fail_msg "test1: score $s < 15"; return 1; }
    ok_msg "test1_classify_rust_app"
}

test2_classify_already_imported() {
    local out; out="$($BIN classify "$TMPDIR/projects/already-imported" 2>/dev/null)"
    local v; v="$(jq_field "$out" .verdict)"
    assert_eq "$v" "ALREADY-REPO" "test2: .git short-circuits Cargo.toml" || return 1
    ok_msg "test2_classify_already_imported"
}

test3_classify_photo_folder() {
    local out; out="$($BIN classify "$TMPDIR/projects/photo-folder" 2>/dev/null)"
    local v; v="$(jq_field "$out" .verdict)"
    assert_eq "$v" "NOT-A-PROJECT" "test3: verdict" || return 1
    ok_msg "test3_classify_photo_folder"
}

test4_scan_tree() {
    local out; out="$($BIN scan-tree "$TMPDIR/projects" 2>/dev/null)"
    local n; n="$(printf '%s' "$out" | jq 'length' 2>/dev/null || echo 0)"
    [ "$n" -ge 4 ] || { fail_msg "test4: scan-tree returned $n entries (<4)"; return 1; }
    ok_msg "test4_scan_tree (entries=$n)"
}

test5_classify_remote() {
    local out; out="$($BIN classify --remote "mockdrive:rust-app" 2>/dev/null)"
    local v; v="$(jq_field "$out" .verdict)"
    assert_eq "$v" "PROJECT" "test5: verdict via mock-rclone lsf" || return 1
    ok_msg "test5_classify_remote"
}

test6_wizard_secret_block() {
    [ -n "${FORGEJO_PID:-}" ] || { info "test6 SKIP: mock-forgejo unavailable"; return 0; }
    local rc=0
    run_wizard "mockdrive:with-secret" >"$TMPDIR/t6.out" 2>&1 || rc=$?
    [ "$rc" -ne 0 ] || { fail_msg "test6: wizard exit $rc, expected non-zero on secret"; return 1; }
    local L="$TMPDIR/var/kei-drive-import-ledger.csv"
    [ -f "$L" ] && grep -q 'BLOCKED-SECRETS' "$L" \
        || { fail_msg "test6: ledger missing BLOCKED-SECRETS row"; return 1; }
    ok_msg "test6_wizard_secret_block"
}

test7_wizard_remote_allowlist() {
    command -v git >/dev/null 2>&1 || { info "test7 SKIP: git missing"; return 0; }
    local s="$TMPDIR/staging-evil" rc=0
    mkdir -p "$s"
    ( cd "$s" && "$MOCK_REAL_GIT" init -q && "$MOCK_REAL_GIT" remote add origin "https://github.com/evil/repo.git"
      url="$("$MOCK_REAL_GIT" remote get-url origin)"
      printf '%s' "$url" | grep -qE '^http://127\.0\.0\.1:3001/' \
          || { echo "REJECTED: remote not allowlisted: $url" >&2; exit 1; } ) || rc=$?
    [ "$rc" -ne 0 ] || { fail_msg "test7: allowlist guard accepted github.com URL"; return 1; }
    ok_msg "test7_wizard_remote_allowlist"
}

test8_wizard_ledger_ok() {
    [ -n "${FORGEJO_PID:-}" ] || { info "test8 SKIP: mock-forgejo unavailable"; return 0; }
    [ "$FORGEJO_PID" = "external" ] && { info "test8 SKIP: external Forgejo (cannot fake OK auth)"; return 0; }
    command -v git >/dev/null 2>&1 || { info "test8 SKIP: git missing"; return 0; }
    local rc=0
    run_wizard "mockdrive:rust-app" >"$TMPDIR/t8.out" 2>&1 || rc=$?
    local L="$TMPDIR/var/kei-drive-import-ledger.csv"
    [ -f "$L" ] || { fail_msg "test8: ledger not created"; return 1; }
    grep -E ',OK,http://127\.0\.0\.1:3001/' "$L" >/dev/null \
        || { fail_msg "test8: ledger has no OK 127.0.0.1:3001 row (rc=$rc)"; return 1; }
    ok_msg "test8_wizard_ledger_ok"
}

main() {
    BIN="$(locate_binary)" || skip_all "kei-gdrive-import binary not in PATH or target/{release,debug}"
    command -v jq >/dev/null 2>&1 || skip_all "jq required (brew install jq)"
    setup
    start_mock_forgejo
    test1_classify_rust_app
    test2_classify_already_imported
    test3_classify_photo_folder
    test4_scan_tree
    test5_classify_remote
    test6_wizard_secret_block
    test7_wizard_remote_allowlist
    test8_wizard_ledger_ok
    info ""
    info "Tests: $PASS/8 PASS"
    [ "$FAIL" -eq 0 ] || { printf 'Failed: %s\n' "$FAILED_LIST" >&2; exit 1; }
    exit 0
}
main "$@"