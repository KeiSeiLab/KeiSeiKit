# shellcheck shell=bash
# lib-args.sh — flag parsing + --help text.
#
# Sets globals: ACTIVATE_HOOKS, WITH_BRIDGES, WITH_SLEEP_SYNC,
# WITH_PATHWAY, NO_PATHWAY, PROFILE, ADD_LIST, REMOVE_NAME, LIST_MODE,
# ASSUME_YES, NO_EXECUTE.
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

print_help() {
  cat <<EOF
Usage: ./install.sh [flags]

  NOTE: this classic installer is for power users (Rust primitives, custom
  profiles, full control). Most users should prefer the Claude Code plugin:
      /plugin marketplace add KeiSei84/KeiSeiKit
      /plugin install keisei@keisei-marketplace
  See README.md "Plugin install (v0.16+, recommended)" and PLUGIN.md for
  details. The classic installer and the plugin can coexist — use whichever
  fits.

  (no flags)                install profile=minimal (agents + hooks + skills + bridges,
                            no primitives). ~5s, no Rust compile for primitives.

  --profile=<name>          set installed-primitive set to one of:
                              minimal   (no primitives)
                              core      (tomd, kei-doctor)
                              frontend  (8 site tools: mock-render / visual-diff / ...)
                              ops       (8 infra tools: kei-ledger / ssh-check / ...)
                              dev       (9 dev tools: kei-migrate / kei-memory / deep-sleep quartet / ...)
                              mcp       (10 LBM-port tools: kei-router / kei-sage / kei-auth / ...)
                              cortex    (6: kei-cortex daemon + cortex-ui Svelte + kei-pet / kei-shared / kei-ledger / kei-memory)
                              full      (all primitives — MANIFEST source of truth)

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
      --help|-h)         print_help; exit 0 ;;
    esac
  done
}
