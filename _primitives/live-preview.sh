#!/usr/bin/env sh
# live-preview — start / stop / status for a project's dev server.
# Detects framework from package.json; stores PID in .keisei/dev-server.pid.
#
# USAGE
#   live-preview start <dir>
#   live-preview stop  [pid]        # default: reads .keisei/dev-server.pid
#   live-preview status

set -eu

CMD="${1:-}"

usage() {
  cat <<'EOF'
Usage: live-preview start <dir>   — start `npm run dev` in <dir>, record PID
       live-preview stop [pid]    — stop running server (default: recorded PID)
       live-preview status        — show whether a server is running
EOF
}

PID_FILE() {
  dir="${1:-.}"
  mkdir -p "$dir/.keisei"
  printf '%s/.keisei/dev-server.pid\n' "$dir"
}

detect_script() {
  pkg="$1/package.json"
  [ -f "$pkg" ] || { echo "dev"; return; }
  if command -v jq >/dev/null 2>&1; then
    jq -r '.scripts.dev // .scripts.start // "dev"' "$pkg"
  else
    echo "dev"
  fi
}

case "$CMD" in
  start)
    DIR="${2:-}"
    [ -z "$DIR" ] && { usage; exit 1; }
    [ -d "$DIR" ] || { echo "live-preview: $DIR not a directory" >&2; exit 1; }

    PID_F="$(PID_FILE "$DIR")"
    if [ -f "$PID_F" ]; then
      OLD="$(cat "$PID_F" 2>/dev/null || true)"
      if [ -n "$OLD" ] && kill -0 "$OLD" 2>/dev/null; then
        echo "live-preview: server already running pid=$OLD (pidfile $PID_F)" >&2
        exit 1
      fi
    fi

    SCRIPT="$(detect_script "$DIR")"
    echo "[live-preview] starting 'npm run $SCRIPT' in $DIR" >&2
    (
      cd "$DIR"
      nohup npm run "$SCRIPT" >.keisei/dev-server.log 2>&1 &
      echo $! > ".keisei/dev-server.pid"
    )
    NEW="$(cat "$PID_F")"
    echo "live-preview: started pid=$NEW log=$DIR/.keisei/dev-server.log"
    ;;
  stop)
    TARGET="${2:-}"
    if [ -z "$TARGET" ]; then
      PID_F="$(PID_FILE ".")"
      [ -f "$PID_F" ] || { echo "live-preview: no pidfile at $PID_F" >&2; exit 1; }
      TARGET="$(cat "$PID_F")"
    fi
    if kill -0 "$TARGET" 2>/dev/null; then
      kill "$TARGET"
      echo "live-preview: stopped pid=$TARGET"
      [ -f ".keisei/dev-server.pid" ] && rm -f ".keisei/dev-server.pid"
    else
      echo "live-preview: pid=$TARGET not running (cleaning pidfile)" >&2
      [ -f ".keisei/dev-server.pid" ] && rm -f ".keisei/dev-server.pid"
      exit 1
    fi
    ;;
  status)
    PID_F="$(PID_FILE ".")"
    if [ ! -f "$PID_F" ]; then
      echo "live-preview: no pidfile (not running from $(pwd))"
      exit 0
    fi
    PID="$(cat "$PID_F")"
    if kill -0 "$PID" 2>/dev/null; then
      echo "live-preview: running pid=$PID"
    else
      echo "live-preview: stale pidfile (pid=$PID exited)"
      rm -f "$PID_F"
    fi
    ;;
  -h|--help|help|"")
    usage
    ;;
  *)
    echo "live-preview: unknown command '$CMD'" >&2
    usage
    exit 1
    ;;
esac
