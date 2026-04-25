#!/usr/bin/env bash
# tests/battle/battle-entry.sh — container ENTRYPOINT.
# Picks profile from $PROFILE env (default: minimal), runs installer, then
# verify.sh. Kept as a dedicated file (instead of a Dockerfile heredoc) so
# BuildKit isn't required and the script is editable post-image-build.
set -u

PROFILE="${PROFILE:-minimal}"
echo "=== battle-test: profile=$PROFILE ==="
echo "=== host: $(uname -a) ==="
echo "=== cargo: $(cargo --version) ==="
echo "=== jq:    $(jq --version) ==="
echo

cd /opt/keiseikit || { echo "kit missing at /opt/keiseikit"; exit 2; }
./install.sh --profile="$PROFILE" --yes 2>&1
INSTALL_EXIT=$?
echo
echo "=== install exit code: $INSTALL_EXIT ==="

if [ "$INSTALL_EXIT" -ne 0 ]; then
    echo "=== install failed; skipping verify ==="
    exit "$INSTALL_EXIT"
fi

echo
echo "=== running verify.sh ==="
/usr/local/bin/verify.sh
