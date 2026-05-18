#!/usr/bin/env bash
# [SUPERSEDED v0.24] Prefer the unified `kei-provision hetzner <cmd>` Rust
# binary (_primitives/_rust/kei-provision). This shell remains for deployed
# scripts that haven't migrated yet; functionally identical, retained only
# so existing call sites keep working until the migration sweep lands.
#
# provision-hetzner.sh — idempotent Hetzner Cloud server provisioning.
# Wraps the `hcloud` CLI. Install path:
#   $HOME/.claude/agents/_primitives/provision-hetzner.sh
#
# USAGE
#   provision-hetzner.sh create <name> [--type cx22|cax11] [--location fsn1] \
#                                       [--image debian-12] [--ssh-key <id>] \
#                                       [--firewall <name>] [--user-data <file>]
#   provision-hetzner.sh status <name>
#   provision-hetzner.sh destroy <name> [--force]
#   provision-hetzner.sh list
#
# ENV (RULE 0.8 — secrets single source)
#   HCLOUD_TOKEN           — Hetzner API token (REQUIRED). Source:
#                            $(grep ^HCLOUD_TOKEN ~/.claude/secrets/.env | cut -d= -f2)
#
# EXIT
#   0 ok
#   1 usage / missing args / missing deps / unknown command
#   2 hcloud API error (non-idempotent path — inspect stderr)
#
# IDEMPOTENCY
#   `create <name>` on an existing server prints its IP + exits 0.
#   `destroy <name>` on a missing server exits 0 (nothing to do).

set -euo pipefail

log()  { printf '[%s] [provision-hetzner] %s\n' "$(date '+%H:%M:%S')" "$*" >&2; }
die()  { log "ERROR: $*"; exit "${2:-2}"; }

check_deps() {
  command -v hcloud >/dev/null 2>&1 || \
    die "hcloud CLI missing. Install: brew install hcloud (macOS) | https://github.com/hetznercloud/cli/releases" 1
  command -v jq    >/dev/null 2>&1 || die "jq missing. brew install jq" 1
  [ -n "${HCLOUD_TOKEN:-}" ] || die "HCLOUD_TOKEN not set. Source ~/.claude/secrets/.env first." 1
}

# Print server JSON if it exists, empty string otherwise. Never fails.
server_json() {
  local name="$1"
  hcloud server describe "$name" -o json 2>/dev/null || true
}

cmd_list() {
  check_deps
  hcloud server list -o 'columns=id,name,status,ipv4,location,server_type,created'
}

cmd_status() {
  check_deps
  local name="${1:-}"; [ -n "$name" ] || die "status: <name> required" 1
  local json; json=$(server_json "$name")
  if [ -z "$json" ]; then
    echo "absent"
    return 0
  fi
  printf 'name=%s\nstatus=%s\nipv4=%s\nlocation=%s\ntype=%s\n' \
    "$(jq -r .name            <<<"$json")" \
    "$(jq -r .status          <<<"$json")" \
    "$(jq -r '.public_net.ipv4.ip // "-"' <<<"$json")" \
    "$(jq -r .datacenter.location.name <<<"$json")" \
    "$(jq -r .server_type.name <<<"$json")"
}

cmd_create() {
  check_deps
  local name="${1:-}"; shift || true
  [ -n "$name" ] || die "create: <name> required" 1

  local type="cx22" location="fsn1" image="debian-12"
  local ssh_key="" firewall="" user_data=""
  while [ $# -gt 0 ]; do
    case "$1" in
      --type)      type="$2"; shift 2 ;;
      --location)  location="$2"; shift 2 ;;
      --image)     image="$2"; shift 2 ;;
      --ssh-key)   ssh_key="$2"; shift 2 ;;
      --firewall)  firewall="$2"; shift 2 ;;
      --user-data) user_data="$2"; shift 2 ;;
      *) die "create: unknown flag '$1'" 1 ;;
    esac
  done

  # Idempotent fast-path: if the server already exists, just print its IP.
  local existing; existing=$(server_json "$name")
  if [ -n "$existing" ]; then
    local ip; ip=$(jq -r '.public_net.ipv4.ip // "-"' <<<"$existing")
    log "server '$name' already exists → $ip (no-op)"
    echo "$ip"
    return 0
  fi

  local args=(server create
    --name "$name"
    --type "$type"
    --image "$image"
    --location "$location"
    --label "project=kei"
  )
  [ -n "$ssh_key"   ] && args+=(--ssh-key "$ssh_key")
  [ -n "$firewall"  ] && args+=(--firewall "$firewall")
  [ -n "$user_data" ] && { [ -r "$user_data" ] || die "user-data not readable: $user_data" 1; args+=(--user-data-from-file "$user_data"); }

  log "creating '$name' ($type @ $location, image=$image)…"
  # mktemp вместо /tmp/$$ — устраняет symlink-race TOCTOU (security MEDIUM
  # audit 2026-05-18).
  local tmpf; tmpf=$(mktemp /tmp/provision-hetzner.XXXXXX.json) || return 1
  hcloud "${args[@]}" -o json >"$tmpf"
  local ip; ip=$(jq -r '.server.public_net.ipv4.ip' "$tmpf")
  rm -f "$tmpf"
  [ "$ip" != "null" ] && [ -n "$ip" ] || die "create returned no IPv4 (check stderr)"
  log "created '$name' → $ip"
  echo "$ip"
}

cmd_destroy() {
  check_deps
  local name="${1:-}"; shift || true
  [ -n "$name" ] || die "destroy: <name> required" 1
  local force=""
  [ "${1:-}" = "--force" ] && force=1

  local existing; existing=$(server_json "$name")
  if [ -z "$existing" ]; then
    log "server '$name' absent (no-op)"
    return 0
  fi

  if [ -z "$force" ]; then
    printf 'Destroy server "%s"? [y/N] ' "$name" >&2
    read -r ans
    [ "$ans" = "y" ] || [ "$ans" = "Y" ] || { log "aborted"; return 1; }
  fi

  log "deleting '$name'…"
  hcloud server delete "$name" >&2
  log "deleted '$name'"
}

main() {
  local cmd="${1:-}"; shift || true
  case "$cmd" in
    create)  cmd_create  "$@" ;;
    destroy) cmd_destroy "$@" ;;
    status)  cmd_status  "$@" ;;
    list)    cmd_list    "$@" ;;
    -h|--help|"") cat <<EOF >&2
provision-hetzner.sh — idempotent Hetzner Cloud server provisioning.
USAGE
  provision-hetzner.sh create <name> [--type cx22|cax11] [--location fsn1] \\
                                     [--image debian-12] [--ssh-key <id>]  \\
                                     [--firewall <name>] [--user-data <file>]
  provision-hetzner.sh status  <name>
  provision-hetzner.sh destroy <name> [--force]
  provision-hetzner.sh list

ENV
  HCLOUD_TOKEN (required) — load via: source ~/.claude/secrets/.env
EOF
      [ "$cmd" = "-h" ] || [ "$cmd" = "--help" ] && exit 0 || exit 1
      ;;
    *) die "unknown command '$cmd'. Run --help." 1 ;;
  esac
}

main "$@"
