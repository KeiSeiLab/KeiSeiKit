# shellcheck shell=bash
# lib-plan.sh — install-plan estimation, soft-dep status, confirm screen.
#
# Per-primitive time/disk estimates are hardcoded here (not in MANIFEST) to
# keep the manifest declarative + UX hints local. Shell primitives are
# ~1s / 5 KB; rust primitives vary by dep weight.
#
# Requires: primitive_field from lib-profile.sh.
# Requires: say / warn / err from lib-log.sh.
# Reads globals: ASSUME_YES, CONFIRM_TOTAL, CONFIRM_SECS, CONFIRM_MB (set by install.sh).

primitive_time_secs() {
  local name="$1" kind
  kind="$(primitive_field "$name" kind 2>/dev/null || true)"
  case "$kind" in
    shell) echo 1 ;;
    rust)
      case "$name" in
        mock-render|kei-migrate|kei-ledger) echo 20 ;;
        kei-changelog|firewall-diff)        echo 15 ;;
        visual-diff|tokens-sync|ssh-check)  echo 5  ;;
        *) echo 10 ;;
      esac
      ;;
    node) echo 2 ;;
    *) echo 0 ;;
  esac
}

primitive_disk_kb() {
  local name="$1" kind
  kind="$(primitive_field "$name" kind 2>/dev/null || true)"
  case "$kind" in
    shell) echo 5 ;;
    rust)
      case "$name" in
        mock-render|kei-migrate|kei-ledger) echo 30000 ;;
        kei-changelog|firewall-diff)        echo 10000 ;;
        visual-diff|tokens-sync|ssh-check)  echo 5000  ;;
        *) echo 8000 ;;
      esac
      ;;
    node) echo 12000 ;;
    *) echo 0 ;;
  esac
}

# estimate_install — reads newline-separated primitive names from stdin,
# prints "time_secs disk_kb" to stdout.
estimate_install() {
  local total_secs=0 total_kb=0 name s d
  while IFS= read -r name; do
    [ -z "$name" ] && continue
    s="$(primitive_time_secs "$name")"
    d="$(primitive_disk_kb "$name")"
    total_secs=$(( total_secs + s ))
    total_kb=$(( total_kb + d ))
  done
  echo "$total_secs $total_kb"
}

# Consumers-of-tool — list primitives (from $2..$N) whose deps mention $1.
_consumers_of() {
  local tool="$1"; shift
  local n deps_raw out=""
  for n in "$@"; do
    deps_raw="$(primitive_field "$n" deps 2>/dev/null || true)"
    echo "$deps_raw" | grep -qiE "(^|[^a-zA-Z])${tool}([^a-zA-Z]|$)" \
      && out="${out}${n},"
  done
  echo "${out%,}"
}

# check_soft_deps — reads newline-separated primitive names from stdin,
# prints one OK/MISS per unique soft-dep tool used by any listed primitive.
check_soft_deps() {
  local names_nl
  names_nl="$(cat)"
  [ -z "$names_nl" ] && return 0
  local -a tools=(jq pandoc playwright npx cargo hcloud vultr-cli yq sqlite3 curl)
  local -a names_arr=()
  local n tool consumers printed_header=0
  while IFS= read -r n; do [ -n "$n" ] && names_arr+=("$n"); done <<< "$names_nl"
  for tool in "${tools[@]}"; do
    consumers="$(_consumers_of "$tool" "${names_arr[@]}")"
    [ -z "$consumers" ] && continue
    [ "$printed_header" = "0" ] && echo "Soft-dep status:" && printed_header=1
    if command -v "$tool" >/dev/null 2>&1; then
      echo "  [OK]   $tool installed"
    else
      echo "  [MISS] $tool missing (needed for: $consumers)"
    fi
  done
}

# Per-primitive row (helper for print_plan_body). Stdin: newline names.
_print_primitive_rows() {
  local name kind extra
  while IFS= read -r name; do
    [ -z "$name" ] && continue
    kind="$(primitive_field "$name" kind 2>/dev/null || echo '?')"
    extra="$(primitive_time_secs "$name")s, $(( $(primitive_disk_kb "$name") / 1024 )) MB"
    printf '  + %s%-22s%s (%s, ~%s)\n' "${KEI_BLUE:-}" "$name" "${KEI_RST:-}" "$kind" "$extra"
  done
}

# print_plan_body — prints "Install Plan" block for given label + names.
# Args: $1 = label, stdin = newline-separated primitive names.
# Sets globals: CONFIRM_TOTAL, CONFIRM_SECS, CONFIRM_MB.
print_plan_body() {
  local profile_label="$1"
  local names total est_secs est_kb est_mb
  names="$(cat)"
  total="$(printf '%s\n' "$names" | grep -c . || true)"
  read -r est_secs est_kb <<< "$(printf '%s\n' "$names" | estimate_install)"
  est_mb=$(( est_kb / 1024 ))
  echo
  echo "================================"
  echo " Install Plan"
  echo "================================"
  echo
  echo "Profile:    $profile_label"
  echo "Primitives: ${total:-0} to add"
  [ "${total:-0}" -gt 0 ] && printf '%s\n' "$names" | _print_primitive_rows
  echo
  printf '%s\n' "$names" | check_soft_deps || true
  echo
  printf 'Estimated time: ~%ss\n'    "$est_secs"
  printf 'Estimated disk: ~%s MB\n' "$est_mb"
  echo
  CONFIRM_TOTAL="$total"; CONFIRM_SECS="$est_secs"; CONFIRM_MB="$est_mb"
}

# show_confirm_screen — prints plan body, then asks y/N (or whiptail --yesno).
# Stdin: newline-separated primitive names. Returns 0=confirmed, 1=declined.
show_confirm_screen() {
  local profile_label="$1"
  print_plan_body "$profile_label"
  [ "$ASSUME_YES" = "1" ] && { echo "(--yes: auto-confirming)"; return 0; }
  [ ! -t 0 ] && { echo "(non-TTY: auto-confirming)"; return 0; }
  if command -v whiptail >/dev/null 2>&1; then
    whiptail --yesno "Install ${CONFIRM_TOTAL:-0} primitive(s) for profile '$profile_label'?\n\nTime: ~${CONFIRM_SECS}s, disk: ~${CONFIRM_MB} MB" 14 70
    return $?
  fi
  local reply
  printf 'Proceed? [Y/n]: '
  read -r reply || return 1
  case "${reply:-Y}" in
    y|Y|yes|YES|'') return 0 ;;
    *) return 1 ;;
  esac
}
