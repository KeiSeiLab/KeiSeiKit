# shellcheck shell=bash
# lib-scaffold.sh — directory scaffolding + MEMORY.md + primitives-meta copy
# + always-on sleep scripts + clean-slate primitive reset + profile install.
#
# These are phase orchestrators that glue lib-primitives + lib-profile together
# under the single top-level flow. Kept here (not in lib-primitives) so that
# cube stays <200 LOC and mono-concern (per-primitive ops + state + list).
#
# Requires: say / warn from lib-log.sh.
# Requires: primitive_field from lib-profile.sh.
# Requires: read_installed, install_primitives, regenerate_rust_workspace from lib-primitives.sh.
# Reads globals: $HOME_DIR, $AGENTS_DIR, $HOOKS_DIR, $SKILLS_DIR, $KIT_DIR,
#                $INSTALLED_FILE, $PROFILE_PRIMS.

# Create every directory we'll touch. Idempotent.
setup_target_dirs() {
  say "creating directories"
  mkdir -p \
    "$AGENTS_DIR/_blocks" \
    "$AGENTS_DIR/_manifests" \
    "$AGENTS_DIR/_primitives" \
    "$AGENTS_DIR/_templates" \
    "$AGENTS_DIR/_assembler/src" \
    "$AGENTS_DIR/_generated" \
    "$HOOKS_DIR" \
    "$SKILLS_DIR/new-agent" \
    "$HOME_DIR/.claude/memory" \
    "$HOME_DIR/.claude/scripts"
}

# Write a stub MEMORY.md if the user has no index yet. We never overwrite.
scaffold_memory_index() {
  local memory_index="$HOME_DIR/.claude/memory/MEMORY.md"
  [[ -f "$memory_index" ]] && return 0
  cat > "$memory_index" <<'EOF'
# Auto Memory — Index

> File-based memory index. Add entries as you save memory files under this directory.
> See `_blocks/memory-protocol.md` for format.
EOF
  say "scaffolded $memory_index"
}

# Copy MANIFEST.toml + README.md so --list works after install. Best-effort.
copy_primitives_meta() {
  mkdir -p "$AGENTS_DIR/_primitives"
  cp -f "$KIT_DIR/_primitives/MANIFEST.toml" "$AGENTS_DIR/_primitives/MANIFEST.toml" 2>/dev/null || true
  cp -f "$KIT_DIR/_primitives/README.md" "$AGENTS_DIR/_primitives/" 2>/dev/null || true
}

# v0.11 sleep-sync + v0.12 sleep-on-it queue scripts. Always available
# regardless of profile (zero binary deps); the user opts in at runtime
# via /sleep-setup + /sleep-on-it. Copy every install.
copy_sleep_scripts() {
  local sleep_sh src
  for sleep_sh in kei-sleep-setup.sh kei-sleep-sync.sh kei-sleep-queue.sh; do
    src="$KIT_DIR/_primitives/$sleep_sh"
    if [ -f "$src" ]; then
      cp -f "$src" "$AGENTS_DIR/_primitives/$sleep_sh"
      chmod +x "$AGENTS_DIR/_primitives/$sleep_sh"
    fi
  done
  if [ -d "$KIT_DIR/_primitives/templates" ]; then
    mkdir -p "$AGENTS_DIR/_primitives/templates"
    cp -f "$KIT_DIR/_primitives/templates/"*.md "$AGENTS_DIR/_primitives/templates/" 2>/dev/null || true
  fi
}

# KeiSei tamagotchi statusline — copy the renderer + state updater into
# ~/.claude/scripts/. Zero binary deps (pure bash, state under ~/.claude/pet/),
# always available regardless of profile. The statusLine + pet-update hooks
# are wired into settings.json by the settings-snippet merge (lib-hooks.sh).
copy_pet_scripts() {
  local pet_sh src dst="$HOME_DIR/.claude/scripts"
  [ -d "$KIT_DIR/scripts" ] || return 0
  mkdir -p "$dst"
  for pet_sh in keisei-pet.sh keisei-pet-update.sh; do
    src="$KIT_DIR/scripts/$pet_sh"
    if [ -f "$src" ]; then
      cp -f "$src" "$dst/$pet_sh"
      chmod +x "$dst/$pet_sh"
    fi
  done
}

# Clean slate: drop every shell .sh + rust crate dir from the installed set.
# FAST (no per-rust rebuild). A single regenerate_rust_workspace at the end
# of install_primitives handles the final state.
clean_slate_primitives() {
  local existing_installed n k f c
  existing_installed="$(read_installed)"
  [ -z "${existing_installed:-}" ] && return 0
  while IFS= read -r n; do
    [ -z "$n" ] && continue
    k="$(primitive_field "$n" kind 2>/dev/null || true)"
    case "$k" in
      shell) f="$(primitive_field "$n" file)";  [ -n "$f" ] && rm -f "$AGENTS_DIR/_primitives/$f" ;;
      rust)  c="$(primitive_field "$n" crate)"; [ -n "$c" ] && rm -rf "$AGENTS_DIR/_primitives/_rust/$c" ;;
    esac
  done <<< "$existing_installed"
  : > "$INSTALLED_FILE"
}

# Install fresh per profile. install_primitives rebuilds rust workspace once
# at the end if any rust crate was added; for minimal we still need to scrub
# any stale workspace Cargo.toml via regenerate_rust_workspace.
install_profile_primitives() {
  if [ -n "${PROFILE_PRIMS:-}" ]; then
    printf '%s\n' "$PROFILE_PRIMS" | tr ' ' '\n' | grep -v '^$' | install_primitives
  else
    regenerate_rust_workspace
    say "  (no primitives — minimal profile)"
  fi
}

# Top-level primitive phase: meta + sleep + clean + install.
run_primitives_phase() {
  copy_primitives_meta
  copy_sleep_scripts
  copy_pet_scripts
  say "resolving primitives for profile=$PROFILE"
  clean_slate_primitives
  install_profile_primitives
}

# Expand one --add=<tok> token into newline-separated primitive name(s):
# if <tok> is a known profile, emit its members; otherwise emit <tok> itself.
_expand_add_token() {
  local token="$1" local_members
  local_members="$(profile_members "$token" 2>/dev/null || true)"
  if [ -n "$local_members" ]; then
    printf '%s\n' "$local_members" | tr ' ' '\n'
  else
    printf '%s\n' "$token"
  fi
}

# Incremental --add/--remove short-circuit. Skips the full agent/hook/skills
# sync and just mutates the primitive set. Assumes a prior install already
# wrote _blocks etc. Reads $ADD_LIST / $REMOVE_NAME set by parse_args.
run_incremental_change() {
  [ -f "$MANIFEST" ] || { err "MANIFEST.toml missing: $MANIFEST"; exit 2; }
  mkdir -p "$AGENTS_DIR/_primitives"

  if [ -n "$REMOVE_NAME" ]; then
    say "removing primitive: $REMOVE_NAME"
    remove_primitive "$REMOVE_NAME"
  fi

  if [ -n "$ADD_LIST" ]; then
    local token
    {
      tr ',' '\n' <<< "$ADD_LIST" | grep -v '^$' | while IFS= read -r token; do
        _expand_add_token "$token"
      done
    } | grep -v '^$' | sort -u | install_primitives
    say "added: $ADD_LIST"
  fi

  echo
  say "incremental change complete"
  cmd_list
}
