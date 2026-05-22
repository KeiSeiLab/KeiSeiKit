# shellcheck shell=bash
# lib-menu.sh — interactive menu (option C hybrid).
#
# Hierarchy: whiptail > dialog > plain-text bash-select. Stdout contract:
#   - one-line output = profile name OR comma-separated custom primitive list
#   - empty stdout + exit 1 = user cancelled
# Menu is ONLY triggered from the top-level flow: never from --add/--remove/--list.
#
# Requires: all_primitive_names, primitive_field from lib-profile.sh.
# Requires: err from lib-log.sh.
# Reads globals: PROFILE, ADD_LIST, REMOVE_NAME, LIST_MODE (set by install.sh).

# menu_should_skip — return 0 if menu should be skipped, 1 if it should run.
# Skip reasons: any selection flag was passed, or stdin/stdout is not a TTY.
menu_should_skip() {
  [ -n "$PROFILE" ]      && return 0
  [ -n "$ADD_LIST" ]     && return 0
  [ -n "$REMOVE_NAME" ]  && return 0
  [ "$LIST_MODE" = "1" ] && return 0
  [ ! -t 0 ]             && return 0   # interactive stdin only; not -t 1 (curl|bash tees stdout)
  return 1
}

# whiptail/dialog radiolist → profile name. Exits 1 on cancel.
#
# Substrate baseline (ALWAYS installed regardless of profile):
#   37 agents + 67 skills + 39 hooks + 82 blocks + 16 capabilities
#   + 7 roles + 11 cross-tool bridges. ~5s.
# Profile choice = how many ADDITIONAL primitive binaries to add on top.
menu_whiptail_profile() {
  local tool="$1"
  "$tool" --title "${STR_MENU_TITLE:-KeiSeiKit Installer} — ${STR_MENU_SUBSTRATE:-substrate always installed; profile = primitives ADDED on top}" --radiolist \
    "${STR_MENU_PROFILE_PROMPT:-Choose install profile (SPACE to select, ENTER to confirm):}" 28 86 12 \
    "minimal"      "substrate only — 0 primitives (~5s)"                       ON  \
    "core"         "+ 2 primitives (tomd, kei-doctor) (~5s)"                   OFF \
    "frontend"     "+ 8 site tools — mock-render, visual-diff, figma-tokens"   OFF \
    "ops"          "+ 9 infra tools — provision, ssh-check, firewall-diff"     OFF \
    "dev"          "+ 17 dev tools — kei-migrate, kei-memory, deep-sleep"      OFF \
    "mcp"          "+ 10 MCP tools — kei-router, kei-sage, kei-auth, kei-pet"  OFF \
    "cortex"       "+ 11 cortex stack — kei-cortex daemon + cortex-ui"         OFF \
    "full"         "+ all 62 primitives (~5 min, 380 MB)"                      OFF \
    "local-mirror" "dev hub: cortex + Forgejo + CI runner (+ 13 prims)"        OFF \
    "dashboard"    "local-mirror + projects-index + Datasette (+ 16 prims)"    OFF \
    "full-hub"     "dashboard + zoekt + mdbook + restic + gdrive (+ 20)"       OFF \
    "custom"       "pick individual primitives from MANIFEST (64 available)"   OFF \
    3>&1 1>&2 2>&3
}

# whiptail/dialog checklist → comma-separated primitive names. Exits 1 on cancel.
menu_whiptail_custom() {
  local tool="$1"
  local args=() name desc
  while IFS= read -r name; do
    [ -z "$name" ] && continue
    desc="$(primitive_field "$name" desc 2>/dev/null || echo '')"
    # truncate long descs so whiptail doesn't wrap awkwardly
    desc="${desc:0:48}"
    args+=("$name" "$desc" "OFF")
  done < <(all_primitive_names)
  local picked
  picked="$("$tool" --title "Custom — pick primitives" --checklist \
    "SPACE to toggle, ENTER to confirm:" 24 78 16 \
    "${args[@]}" 3>&1 1>&2 2>&3)" || return 1
  # whiptail emits quoted names separated by spaces; normalize to csv
  echo "$picked" | tr -d '"' | tr ' ' ',' | sed 's/^,//;s/,$//'
}

# plain-text profile picker → profile name. Exits 1 on cancel.
menu_plain_profile() {
  echo "============================================================"                  >&2
  echo " ${STR_MENU_TITLE:-KeiSeiKit Installer}"                                       >&2
  echo "============================================================"                  >&2
  echo                                                                                  >&2
  echo " ${STR_MENU_SUBSTRATE:-Substrate baseline (ALWAYS installed):}"                >&2
  echo "   • 37 agent manifests   • 67 skills    • 39 hooks"                           >&2
  echo "   • 82 blocks            • 16 caps      •  7 roles"                           >&2
  echo "   • 11 cross-tool bridges (Cursor / Copilot / Codex / Aider / …)"             >&2
  echo                                                                                  >&2
  echo " ${STR_MENU_PROFILE_PROMPT:-Profile = primitive binaries ADDED on top of substrate.}"                      >&2
  echo "------------------------------------------------------------"                  >&2
  echo                                                                                  >&2
  echo "  Standard:"                                                                    >&2
  echo "    1) minimal      — substrate only, 0 primitives (~5s)"                      >&2
  echo "    2) core         — + 2 prims (tomd, kei-doctor) (~5s)"                      >&2
  echo "    3) frontend     — + 8 site tools (~60s, 80 MB)"                            >&2
  echo "    4) ops          — + 9 infra tools (~90s, 50 MB)"                           >&2
  echo "    5) dev          — + 17 dev tools (~120s, 80 MB)"                           >&2
  echo "    6) mcp          — + 10 MCP/LBM tools (~90s, 50 MB)"                        >&2
  echo "    7) cortex       — + 11 cortex stack (~90s, 60 MB)"                         >&2
  echo "    8) full         — + all 62 primitives (~5 min, 380 MB)"                    >&2
  echo                                                                                  >&2
  echo "  Dev hub (local-first development environment, macOS arm64):"                  >&2
  echo "   10) local-mirror — cortex + Forgejo + CI runner (+ 13 prims)"                >&2
  echo "   11) dashboard    — local-mirror + projects-index + Datasette (+ 16)"         >&2
  echo "   12) full-hub     — dashboard + zoekt + mdbook + restic + gdrive (+ 20)"      >&2
  echo                                                                                  >&2
  echo "    9) custom       — pick individual primitives (64 available)"                >&2
  echo                                                                                  >&2
  local reply
  printf 'Enter choice [1-12] (default 1): ' >&2
  read -r reply || return 1
  case "${reply:-1}" in
    1)  echo minimal      ;;
    2)  echo core         ;;
    3)  echo frontend     ;;
    4)  echo ops          ;;
    5)  echo dev          ;;
    6)  echo mcp          ;;
    7)  echo cortex       ;;
    8)  echo full         ;;
    9)  echo custom       ;;
    10) echo local-mirror ;;
    11) echo dashboard    ;;
    12) echo full-hub     ;;
    *) err "invalid choice: $reply"; return 1 ;;
  esac
}

# Print the numbered primitive list to stderr (helper for plain custom picker).
_print_primitive_list() {
  local -a names=("$@")
  local i desc
  echo                                         >&2
  echo "Select primitives (space-separated numbers, 'a' for all, 'n' for none):" >&2
  echo                                         >&2
  for (( i=0; i<${#names[@]}; i++ )); do
    desc="$(primitive_field "${names[$i]}" desc 2>/dev/null || echo '')"
    printf "  %2d) [ ] %-20s — %s\n" "$((i+1))" "${names[$i]}" "$desc" >&2
  done
  echo                                         >&2
}

# plain-text custom picker → comma-separated primitive names.
menu_plain_custom() {
  local -a names=() picked=()
  local name reply tok
  while IFS= read -r name; do
    [ -z "$name" ] && continue
    names+=("$name")
  done < <(all_primitive_names)
  _print_primitive_list "${names[@]}"
  printf 'Selection: ' >&2
  read -r reply || return 1
  case "$reply" in
    a|A|all) picked=("${names[@]}") ;;
    n|N|none|'') picked=() ;;
    *)
      for tok in $reply; do
        [[ "$tok" =~ ^[0-9]+$ ]] && (( tok >= 1 && tok <= ${#names[@]} )) \
          && picked+=("${names[$((tok-1))]}")
      done
      ;;
  esac
  local IFS=,; echo "${picked[*]}"
}

# Run the menu and parse its output into PROFILE / CUSTOM_PRIMS globals.
# Returns 0 on success (incl. menu_should_skip), 1 on user cancel.
run_menu_if_needed() {
  CUSTOM_PRIMS=""
  CONFIRM_TOTAL=0
  CONFIRM_SECS=0
  CONFIRM_MB=0
  menu_should_skip && return 0
  [ -f "$MANIFEST" ] || { err "MANIFEST.toml missing: $MANIFEST"; exit 2; }
  local menu_out
  menu_out="$(show_interactive_menu)" || { say "menu cancelled — aborting"; return 1; }
  if [ -z "$menu_out" ]; then
    say "no selection — aborting"
    return 1
  fi
  if echo "$menu_out" | grep -q ','; then
    CUSTOM_PRIMS="$menu_out"
    PROFILE="custom"
  elif echo "$menu_out" | grep -qE '^(minimal|core|frontend|ops|dev|mcp|cortex|full|local-mirror|dashboard|full-hub)$'; then
    PROFILE="$menu_out"
  else
    # Single name from custom-with-one-item — treat as CUSTOM_PRIMS
    CUSTOM_PRIMS="$menu_out"
    PROFILE="custom"
  fi
  return 0
}

# show_interactive_menu — master dispatcher. Echoes profile name OR csv list.
show_interactive_menu() {
  local tool=""
  if command -v whiptail >/dev/null 2>&1; then
    tool="whiptail"
  elif command -v dialog >/dev/null 2>&1; then
    tool="dialog"
  fi
  local choice
  if [ -n "$tool" ]; then
    choice="$(menu_whiptail_profile "$tool")" || return 1
    if [ "$choice" = "custom" ]; then
      menu_whiptail_custom "$tool" || return 1
    else
      echo "$choice"
    fi
  else
    choice="$(menu_plain_profile)" || return 1
    if [ "$choice" = "custom" ]; then
      menu_plain_custom
    else
      echo "$choice"
    fi
  fi
}
