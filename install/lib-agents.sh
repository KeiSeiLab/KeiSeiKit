# shellcheck shell=bash
# lib-agents.sh — manifest copy + assembler build + .md agent generation.
#
# Generic manifests: overwrite-skip policy (never stomp user's existing
# manifests). Assembler source: always refreshed. Build: offline-first with
# online fallback. Agents: written in-place by the built assemble binary.
#
# Requires: say / err from lib-log.sh.
# Requires: backup_dir from lib-backup.sh.
# Reads globals: $KIT_DIR, $AGENTS_DIR.

# Copy _manifests/*.toml from the kit; skip any target file that already
# exists (user manifests are sacred). Also copies _templates/*.template
# when present.
install_manifests() {
  say "copying generic manifests -> $AGENTS_DIR/_manifests/ (skip if exists)"
  # Stack filter: when a stack profile is chosen, install only its agent set.
  # Empty allowlist (no stack / non-interactive) => install ALL (back-compat).
  local allow=""
  if command -v resolve_selected_agent_manifests >/dev/null 2>&1; then
    allow="$(resolve_selected_agent_manifests)"
  fi
  local copied=0 skipped=0 filtered=0 f name t has_templates=0
  for f in "$KIT_DIR/_manifests/"*.toml; do
    name="$(basename "$f")"
    if [ -n "$allow" ] && ! printf '%s\n' "$allow" | grep -qx "${name%.toml}"; then
      filtered=$((filtered+1)); continue
    fi
    if [[ -f "$AGENTS_DIR/_manifests/$name" ]]; then
      skipped=$((skipped+1))
    else
      cp "$f" "$AGENTS_DIR/_manifests/$name"
      copied=$((copied+1))
    fi
  done
  if [ -n "$allow" ]; then
    say "  copied $copied, skipped $skipped, stack-filtered $filtered"
  else
    say "  copied $copied, skipped $skipped (already present)"
  fi

  for t in "$KIT_DIR/_templates/"*.template; do
    [ -f "$t" ] && { has_templates=1; break; }
  done
  if [ "$has_templates" = "1" ]; then
    say "copying specialist template"
    backup_dir "$AGENTS_DIR/_templates"
    cp -f "$KIT_DIR/_templates/"*.template "$AGENTS_DIR/_templates/"
  fi
}

# Refresh _blocks/*.md — SSoT is the kit, always overwritten after backup.
install_blocks() {
  say "copying shared blocks -> $AGENTS_DIR/_blocks/"
  backup_dir "$AGENTS_DIR/_blocks"
  cp -f "$KIT_DIR/_blocks/"*.md "$AGENTS_DIR/_blocks/"
}

# Refresh _roles/*.toml — SSoT is the kit, always overwritten after backup.
# Manifests reference these via tool_role/agent_role keys; without them the
# assembler fails with "read role .../<name>.toml: No such file".
install_roles() {
  if ! compgen -G "$KIT_DIR/_roles/"*.toml > /dev/null; then
    return 0
  fi
  say "copying agent roles -> $AGENTS_DIR/_roles/"
  mkdir -p "$AGENTS_DIR/_roles"
  backup_dir "$AGENTS_DIR/_roles"
  cp -f "$KIT_DIR/_roles/"*.toml "$AGENTS_DIR/_roles/"
}

# Refresh _capabilities/<bucket>/<name>/text.md — capability snippets
# referenced by manifests' capabilities[] list. SSoT is the kit; full tree
# copy after backup. Without this the assembler fails with "read capability
# X::Y at .../_capabilities/X/Y/text.md: No such file".
install_capabilities() {
  if [ ! -d "$KIT_DIR/_capabilities" ]; then
    return 0
  fi
  say "copying capability snippets -> $AGENTS_DIR/_capabilities/"
  mkdir -p "$AGENTS_DIR/_capabilities"
  backup_dir "$AGENTS_DIR/_capabilities"
  cp -Rf "$KIT_DIR/_capabilities/"* "$AGENTS_DIR/_capabilities/"
}

# Copy the Rust assembler source (Cargo.toml + src/*.rs + .gitignore if any).
# Caller should run build_assembler afterwards.
copy_assembler_source() {
  say "copying assembler source"
  backup_dir "$AGENTS_DIR/_assembler"
  cp -f "$KIT_DIR/_assembler/Cargo.toml" "$AGENTS_DIR/_assembler/"
  cp -f "$KIT_DIR/_assembler/src/"*.rs "$AGENTS_DIR/_assembler/src/"
  if [[ -f "$KIT_DIR/_assembler/.gitignore" ]]; then
    cp -f "$KIT_DIR/_assembler/.gitignore" "$AGENTS_DIR/_assembler/"
  fi
}

# Build the assembler (release, offline-first, online fallback).
# Exits 2 if the binary is missing after a reported success (disk failure).
build_assembler() {
  copy_assembler_source
  say "building Rust assembler (cargo build --release, offline first)"
  if ! ( cd "$AGENTS_DIR/_assembler" && cargo build --release --offline ) 2>/tmp/keiseikit-cargo-offline.log; then
    say "offline build failed — fetching deps from crates.io"
    ( cd "$AGENTS_DIR/_assembler" && cargo build --release )
  fi
  if [[ ! -x "$AGENTS_DIR/_assembler/target/release/assemble" ]]; then
    err "build succeeded but binary not found at $AGENTS_DIR/_assembler/target/release/assemble"
    exit 2
  fi
}

# Run the built assembler in --in-place mode to write the agent .md files.
# Tolerant by design: a handful of stale/broken manifests (e.g. a handoff to
# an agent not shipped in this profile, or an un-substituted template field)
# must NOT abort the whole install — hooks, skills, settings still need to
# land. The assembler prints per-manifest FAIL lines for visibility, and the
# commit-time assembler-validate gate stays strict, so genuine breakage is
# still caught at authoring time.
generate_agents() {
  say "generating agent .md files (--in-place)"
  if ! AGENT_ROOT="$AGENTS_DIR" "$AGENTS_DIR/_assembler/target/release/assemble" --in-place; then
    warn "some agent manifests failed to assemble (see FAIL lines above) — continuing install"
  fi
}
