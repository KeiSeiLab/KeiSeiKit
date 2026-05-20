# shellcheck shell=bash
# lib-args.sh — flag parsing + --help text.
#
# Sets globals: ACTIVATE_HOOKS, WITH_BRIDGES, WITH_SLEEP_SYNC,
# WITH_PATHWAY, NO_PATHWAY, PROFILE, ADD_LIST, REMOVE_NAME, LIST_MODE,
# ASSUME_YES, NO_EXECUTE, OUTCOME_DRY_RUN.
# --help exits 0 immediately.

ACTIVATE_HOOKS=0
WITH_BRIDGES=0
WITH_SLEEP_SYNC=0
WITH_PATHWAY=0
NO_PATHWAY=0
PROFILE=""
ADD_LIST=""
REMOVE_NAME=""
LIST_MODE=0
ASSUME_YES=0
NO_EXECUTE=0
SKIP_PREREQS=0
REBUILD_RUST_LIST=""
REBUILD_RUST_FLAG=0
OUTCOME_DRY_RUN=0

print_help() {
  cat <<EOF
Usage: ./install.sh [flags]

  NOTE: this classic installer is for power users (Rust primitives, custom
  profiles, full control). Most users should prefer the Claude Code plugin:
      /plugin marketplace add KeiSeiLab/KeiSeiKit-1.0
      /plugin install keisei@keisei-marketplace
  See README.md "Plugin install (v0.16+, recommended)" and PLUGIN.md for
  details. The classic installer and the plugin can coexist — use whichever
  fits.

  (no flags)                install profile=minimal — substrate baseline only
                            (37 agents + 67 skills + 39 hooks + 82 blocks +
                            16 caps + 7 roles + 11 bridges, NO primitives).
                            ~5s, no Rust compile.

  --profile=<name>          add primitive bundles on top of substrate baseline:
                            Outcome-tracking only (no substrate, no daemon):
                              outcome-only — 2 hooks + ledger.sqlite + 1 line
                                             in CLAUDE.md + (deferred) router.
                                             ~5 files, ~200 LOC. See
                                             docs/PROFILE-OUTCOME-ONLY.md
                            Standard:
                              minimal      — 0 primitives (~5s)
                              core         — 2 prims (tomd, kei-doctor)
                              frontend     — 8 site tools (mock-render, visual-diff, …)
                              ops          — 9 infra tools (provision, ssh-check, …)
                              dev          — 17 dev tools (kei-migrate, kei-memory, …)
                              mcp          — 10 MCP/LBM tools (kei-router, kei-sage, …)
                              cortex       — 11 cortex stack (kei-cortex daemon + cortex-ui)
                              full         — all 62 primitives (MANIFEST source of truth)
                            Dev hub (local-first dev environment, macOS arm64):
                              local-mirror — cortex + Forgejo + CI runner (13 prims)
                              dashboard    — local-mirror + projects-index + Datasette (16)
                              full-hub     — dashboard + zoekt + mdbook + restic + gdrive (20)

  --add=<a>[,<b>,...]       add one or more primitives on top of current install.
                            Name must match [primitive.<name>] in _primitives/MANIFEST.toml.

  --remove=<name>           remove a single primitive (shell file or rust crate dir +
                            scoped workspace Cargo.toml regenerated + rebuilt).

  --list                    list installed primitives from .installed state file.

  --with-bridges            render the 11 cross-tool bridge files into \$PWD
                            (Cursor / Copilot / Codex / Windsurf / Junie / Continue /
                            Aider / Replit / Antigravity / Warp / Zed).
                            Skipped if invoked inside the KeiSeiKit repo itself.

  --with-sleep-sync         after core install, run the v0.11 sleep-layer
                            setup helper (kei-sleep-setup.sh). TTY-only — no-op
                            on CI / non-interactive invocations. Print a
                            reminder to finish via /sleep-setup either way.

  --with-pathway            force PATH wiring (~/.bashrc / ~/.zshrc / fish
                            config) for the substrate target/release dir.
                            Default: auto-on for interactive TTY installs.

  --no-pathway              force-skip PATH wiring (do not modify any rc
                            file). Useful for CI or when the user manages
                            PATH via another mechanism (e.g. nix shell).

  --activate-hooks          jq-merge settings-snippet.json into ~/.claude/settings.json
                            non-interactively. Without this flag, a TTY prompt asks
                            at the end; non-TTY runs print manual instructions.

  --yes, -y                 skip the interactive confirm screen after the menu
                            (for automation). If no --profile was given the menu
                            still runs; --yes only auto-accepts the Install Plan.

  --no-execute              run flag parsing + menu + confirm, print the
                            resolved plan, then exit before copying/building
                            anything. Useful for dry-run / testing.

  --dry-run                 with --profile=outcome-only: print the list of
                            files that WOULD be touched in \$HOME, then exit
                            0 without writing. No-op for other profiles.

  --rebuild-rust            (dev-only) rebuild full Rust workspace + mirror
                            fresh binaries to ~/.claude/agents/_primitives/
                            _rust/target/release/. Closes the drift gap
                            between dev source edits and PATH-resolved
                            installed binaries. No-op for kit users.

  --rebuild-rust=<crate>    selective variant — rebuild only the named
                            crate (or comma-separated list) and mirror
                            those binaries. Faster than full workspace
                            on incremental edits.

  --help, -h                this help.
EOF
}

parse_args() {
  local arg
  for arg in "$@"; do
    case "$arg" in
      --activate-hooks)  ACTIVATE_HOOKS=1 ;;
      --with-bridges)    WITH_BRIDGES=1 ;;
      --with-sleep-sync) WITH_SLEEP_SYNC=1 ;;
      --with-pathway)    WITH_PATHWAY=1 ;;
      --no-pathway)      NO_PATHWAY=1 ;;
      --profile=*)       PROFILE="${arg#--profile=}" ;;
      --add=*)           ADD_LIST="${arg#--add=}" ;;
      --remove=*)        REMOVE_NAME="${arg#--remove=}" ;;
      --list)            LIST_MODE=1 ;;
      --yes|-y)          ASSUME_YES=1 ;;
      --no-execute)      NO_EXECUTE=1 ;;
      --skip-prereqs)    SKIP_PREREQS=1 ;;
      --rebuild-rust)    REBUILD_RUST_FLAG=1 ;;
      --rebuild-rust=*)  REBUILD_RUST_FLAG=1; REBUILD_RUST_LIST="${arg#--rebuild-rust=}" ;;
      --dry-run)         OUTCOME_DRY_RUN=1 ;;
      --help|-h)         print_help; exit 0 ;;
    esac
  done
}
