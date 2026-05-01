#!/usr/bin/env bash
# Build a minimal KeiSei.app bundle that opens Terminal with `kei`.
#
# Output: $OUT_DIR/KeiSei.app (default OUT_DIR=$HOME/Applications)
# Usage:
#   ./tools/mac-app/build.sh                  # → ~/Applications/KeiSei.app
#   OUT_DIR=/tmp ./tools/mac-app/build.sh    # → /tmp/KeiSei.app
#
# Requires: macOS (uses osascript). On Linux/Windows this is a no-op.

set -euo pipefail

OUT_DIR="${OUT_DIR:-$HOME/Applications}"
APP="$OUT_DIR/KeiSei.app"

if [ "$(uname -s)" != "Darwin" ]; then
  echo "skip: macOS only (current OS: $(uname -s))" >&2
  exit 0
fi

mkdir -p "$APP/Contents/MacOS" "$APP/Contents/Resources"

cat > "$APP/Contents/MacOS/KeiSei" <<'LAUNCHER'
#!/bin/bash
# Open Terminal with `kei` (the KeiSeiKit launcher).
# Falls back to `keisei` (Rust binary) help if `kei` not on PATH.
osascript <<APPLESCRIPT
tell application "Terminal"
    activate
    if (count of windows) = 0 then
        do script "kei || keisei || echo 'kei/keisei not found; install KeiSeiKit first'"
    else
        do script "kei || keisei || echo 'kei/keisei not found; install KeiSeiKit first'" in window 1
    end if
end tell
APPLESCRIPT
LAUNCHER
chmod +x "$APP/Contents/MacOS/KeiSei"

cat > "$APP/Contents/Info.plist" <<PLIST
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>CFBundleExecutable</key>
    <string>KeiSei</string>
    <key>CFBundleIdentifier</key>
    <string>com.keisei.launcher</string>
    <key>CFBundleName</key>
    <string>KeiSei</string>
    <key>CFBundleDisplayName</key>
    <string>KeiSei</string>
    <key>CFBundlePackageType</key>
    <string>APPL</string>
    <key>CFBundleVersion</key>
    <string>0.16</string>
    <key>CFBundleShortVersionString</key>
    <string>0.16</string>
    <key>NSHighResolutionCapable</key>
    <true/>
    <key>LSUIElement</key>
    <false/>
</dict>
</plist>
PLIST

echo "✓ Built: $APP"
echo "  Drag to /Applications or run from Spotlight: 'KeiSei'"
