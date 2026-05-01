#!/usr/bin/env bash
# [SUPERSEDED v0.24] Prefer the unified `kei-provision vultr <cmd>` Rust
# binary (_primitives/_rust/kei-provision). This shell remains for deployed
# scripts that haven't migrated yet; functionally identical, retained only
# so existing call sites keep working until the migration sweep lands.
#
# provision-vultr.sh — idempotent Vultr VPS provisioning.
# Wraps the `vultr-cli` v3. Install path:
#   $HOME/.claude/agents/_primitives/provision-vultr.sh
#
# USAGE
#   provision-vultr.sh create <label>  [--plan vc2-1c-1gb] [--region ams] \
#                                      [--os-id 2136] [--ssh-key <id>] \
#                                      [--firewall <group-id>] [--user-data <file>]
#   provision-vultr.sh status  <label>
#   provision-vultr.sh destroy <label> [--force]
#   provision-vultr.sh list
#
# ENV (RULE 0.8 — secrets single source)
#   VULTR_API_KEY          — Vultr API key (REQUIRED). Source:
#                            $(grep ^VULTR_API_KEY ~/.claude/secrets/.env | cut -d= -f2)
#
# NOTES
#   * vultr-cli v3: `vultr-cli instance create …` (not `server`).
#   * --os-id 2136 = Debian 12 x86_64 (subject to change; verify via
#     `vultr-cli os list | grep Debian`). We do NOT hard-code the ID.
#   * Vultr identifies instances by UUID; we use the human-friendly `label`
#     field for idempotency. Labels must be unique within the account.
#
# EXIT
#   0 ok
#   1 usage / missing args / missing deps / unknown command
#   2 vultr API error

set -euo pipefail

log()  { printf '[%s] [provision-vultr] %s\n' "$(date '+%H:%M:%S')" "$*" >&2; }
die()  { log "ERROR: $*"; exit "${2:-2}"; }

check_deps() {
  command -v vultr-cli >/dev/null 2>&1 || \
    die "vultr-cli missing. Install: brew install vultr/vultr-cli/vultr-cli | https://github.com/vultr/vultr-cli" 1
  command -v jq        >/dev/null 2>&1 || die "jq missing. brew install jq" 1
  [ -n "${VULTR_API_KEY:-}" ] || die "VULTR_API_KEY not set. Source ~/.claude/secrets/.env first." 1
}

# Return JSON of instance with matching label, or empty string.
instance_json_by_label() {
  local label="$1"
  vultr-cli instance list -o json 2>/dev/null \
    | jq -c --arg l "$label" '.instances[] | select(.label == $l)' \
    | head -n1
}

cmd_list() {
  check_deps
  vultr-cli instance list -o json \
    | jq -r '.instances[] | [.label, .region, .plan, .status, .main_ip] | @tsv'
}

cmd_status() {
  check_deps
  local label="${1:-}"; [ -n "$label" ] || die "status: <label> required" 1
  local json; json=$(instance_json_by_label "$label")
  if [ -z "$json" ]; then
    echo "absent"
    return 0
  fi
  printf 'label=%s\nid=%s\nstatus=%s\npower=%s\nip=%s\nregion=%s\nplan=%s\n' \
    "$(jq -r .label    <<<"$json")" \
    "$(jq -r .id       <<<"$json")" \
    "$(jq -r .status   <<<"$json")" \
    "$(jq -r .power_status <<<"$json")" \
    "$(jq -r .main_ip  <<<"$json")" \
    "$(jq -r .region   <<<"$json")" \
    "$(jq -r .plan     <<<"$json")"
}

resolve_debian_12_os() {
  # Return the OS id for "Debian 12 x64" (subject to Vultr catalog updates).
  vultr-cli os list -o json \
    | jq -r '.os[] | select(.name | test("Debian 12.*x64"; "i")) | .id' \
    | head -n1
}

cmd_create() {
  check_deps
  local label="${1:-}"; shift || true
  [ -n "$label" ] || die "create: <label> required" 1

  local plan="vc2-1c-1gb" region="ams" os_id=""
  local ssh_key="" firewall="" user_data=""
  while [ $# -gt 0 ]; do
    case "$1" in
      --plan)      plan="$2"; shift 2 ;;
      --region)    region="$2"; shift 2 ;;
      --os-id)     os_id="$2"; shift 2 ;;
      --ssh-key)   ssh_key="$2"; shift 2 ;;
      --firewall)  firewall="$2"; shift 2 ;;
      --user-data) user_data="$2"; shift 2 ;;
      *) die "create: unknown flag '$1'" 1 ;;
    esac
  done

  # Idempotency.
  local existing; existing=$(instance_json_by_label "$label")
  if [ -n "$existing" ]; then
    local ip; ip=$(jq -r '.main_ip // "-"' <<<"$existing")
    log "instance '$label' already exists → $ip (no-op)"
    echo "$ip"
    return 0
  fi

  [ -z "$os_id" ] && { os_id=$(resolve_debian_12_os) || true; }
  [ -n "$os_id" ] || die "cannot resolve Debian 12 OS id. Pass --os-id explicitly." 1

  local args=(instance create
    --region "$region"
    --plan   "$plan"
    --os     "$os_id"
    --label  "$label"
    --tags   "project=kei"
  )
  [ -n "$ssh_key"  ] && args+=(--ssh-keys "$ssh_key")
  [ -n "$firewall" ] && args+=(--firewall-group-id "$firewall")
  if [ -n "$user_data" ]; then
    [ -r "$user_data" ] || die "user-data not readable: $user_data" 1
    # vultr-cli expects base64 for userdata.
    args+=(--userdata "$(base64 < "$user_data" | tr -d '\n')")
  fi

  log "creating '$label' ($plan @ $region, os=$os_id)…"
  vultr-cli "${args[@]}" -o json >/tmp/provision-vultr-$$.json
  local ip; ip=$(jq -r '.instance.main_ip' /tmp/provision-vultr-$$.json)
  rm -f /tmp/provision-vultr-$$.json
  # Vultr assigns IP asynchronously — re-poll if empty.
  if [ "$ip" = "" ] || [ "$ip" = "null" ] || [ "$ip" = "0.0.0.0" ]; then
    log "IP pending — polling instance status up to 60s…"
    for _ in $(seq 1 30); do
      sleep 2
      ip=$(instance_json_by_label "$label" | jq -r '.main_ip // ""')
      [ -n "$ip" ] && [ "$ip" != "0.0.0.0" ] && break
    done
  fi
  [ -n "$ip" ] && [ "$ip" != "0.0.0.0" ] || die "create: no IPv4 after 60s poll"
  log "created '$label' → $ip"
  echo "$ip"
}

cmd_destroy() {
  check_deps
  local label="${1:-}"; shift || true
  [ -n "$label" ] || die "destroy: <label> required" 1
  local force=""
  [ "${1:-}" = "--force" ] && force=1

  local existing; existing=$(instance_json_by_label "$label")
  if [ -z "$existing" ]; then
    log "instance '$label' absent (no-op)"
    return 0
  fi
  local id; id=$(jq -r .id <<<"$existing")

  if [ -z "$force" ]; then
    printf 'Destroy instance "%s" (%s)? [y/N] ' "$label" "$id" >&2
    read -r ans
    [ "$ans" = "y" ] || [ "$ans" = "Y" ] || { log "aborted"; return 1; }
  fi

  log "deleting '$label' ($id)…"
  vultr-cli instance delete "$id" >&2
  log "deleted '$label'"
}

main() {
  local cmd="${1:-}"; shift || true
  case "$cmd" in
    create)  cmd_create  "$@" ;;
    destroy) cmd_destroy "$@" ;;
    status)  cmd_status  "$@" ;;
    list)    cmd_list    "$@" ;;
    -h|--help|"") cat <<EOF >&2
provision-vultr.sh — idempotent Vultr VPS provisioning.
USAGE
  provision-vultr.sh create <label>  [--plan vc2-1c-1gb] [--region ams] \\
                                     [--os-id <id>] [--ssh-key <id>]   \\
                                     [--firewall <group-id>] [--user-data <file>]
  provision-vultr.sh status  <label>
  provision-vultr.sh destroy <label> [--force]
  provision-vultr.sh list

ENV
  VULTR_API_KEY (required) — load via: source ~/.claude/secrets/.env
EOF
      [ "$cmd" = "-h" ] || [ "$cmd" = "--help" ] && exit 0 || exit 1
      ;;
    *) die "unknown command '$cmd'. Run --help." 1 ;;
  esac
}

main "$@"
