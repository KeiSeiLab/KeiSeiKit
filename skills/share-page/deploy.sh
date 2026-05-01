#!/bin/bash
# Deploy an HTML file to GitHub Pages (KeiSei84/shares)
# Usage: deploy.sh <html-file> [custom-slug]
# Returns: shareable URL

set -e

RED='\033[0;31m'
GREEN='\033[0;32m'
CYAN='\033[0;36m'
NC='\033[0m'

HTML_FILE="$1"
CUSTOM_SLUG="$2"
REPO_DIR="/tmp/shares"
REPO_URL="https://github.com/KeiSei84/shares.git"
PAGES_BASE="https://keisei84.github.io/shares"

# Validate input
if [ -z "$HTML_FILE" ]; then
    echo -e "${RED}Error: provide an HTML file path${NC}" >&2
    echo "Usage: $0 <html-file> [custom-slug]" >&2
    exit 1
fi

if [ ! -f "$HTML_FILE" ]; then
    echo -e "${RED}Error: file not found: $HTML_FILE${NC}" >&2
    exit 1
fi

# Generate slug from filename or use custom
if [ -n "$CUSTOM_SLUG" ]; then
    SLUG="$CUSTOM_SLUG"
else
    SLUG=$(basename "$HTML_FILE" .html | tr '[:upper:]' '[:lower:]' | sed 's/[^a-z0-9-]/-/g' | sed 's/--*/-/g' | sed 's/^-//;s/-$//')
fi

FILENAME="${SLUG}.html"

# Clone or pull repo
if [ -d "$REPO_DIR/.git" ]; then
    echo -e "${CYAN}Pulling latest...${NC}" >&2
    cd "$REPO_DIR" && git pull --quiet origin main 2>/dev/null || true
else
    echo -e "${CYAN}Cloning repo...${NC}" >&2
    rm -rf "$REPO_DIR"
    git clone --quiet "$REPO_URL" "$REPO_DIR"
fi

cd "$REPO_DIR"

# Copy file
cp "$HTML_FILE" "$FILENAME"

# Check if anything changed
if git diff --quiet -- "$FILENAME" 2>/dev/null && git ls-files --error-unmatch "$FILENAME" >/dev/null 2>&1; then
    echo -e "${CYAN}No changes detected, file already deployed${NC}" >&2
    LIVE_URL="${PAGES_BASE}/${FILENAME}"
    echo -e "${GREEN}URL: ${LIVE_URL}${NC}" >&2
    echo "$LIVE_URL"
    exit 0
fi

# Deploy
git add "$FILENAME"
git commit --quiet -m "share: ${SLUG}"
git push --quiet origin main

LIVE_URL="${PAGES_BASE}/${FILENAME}"

echo "" >&2
echo -e "${GREEN}Deployed successfully!${NC}" >&2
echo -e "${GREEN}URL: ${LIVE_URL}${NC}" >&2
echo -e "${CYAN}(GitHub Pages propagation: 30-60 sec)${NC}" >&2
echo "" >&2

# Output URL for programmatic use
echo "$LIVE_URL"
