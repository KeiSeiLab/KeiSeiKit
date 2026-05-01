#!/usr/bin/env sh
# design-scrape — Playwright-based scrape of a live website into tokens,
# section map, and full-page screenshots. Output: one directory per URL.
#
# USAGE
#   design-scrape <url> [--out <dir>]
#
# OUTPUT
#   <out>/desktop.png
#   <out>/mobile.png
#   <out>/tokens.json
#   <out>/structure.json
#
# Requires: npx (Node), Playwright (`npx playwright install chromium` once).

set -eu

URL="${1:-}"
OUT="${OUT:-./design-scrape}"

usage() {
  cat <<'EOF'
Usage: design-scrape <url> [--out <dir>]

Captures two full-page screenshots (desktop 1280x900, mobile 375x812),
extracts computed tokens + DOM structure via Playwright. Writes all
artefacts under <dir> (default: ./design-scrape).
EOF
}

[ -z "$URL" ] || [ "$URL" = "-h" ] || [ "$URL" = "--help" ] && { usage; [ -z "$URL" ] && exit 1; exit 0; }

# --out arg parsing (positional URL first)
shift
while [ $# -gt 0 ]; do
  case "$1" in
    --out) OUT="$2"; shift 2 ;;
    *) echo "design-scrape: unknown arg: $1" >&2; exit 1 ;;
  esac
done

if ! command -v npx >/dev/null 2>&1; then
  echo "design-scrape: npx not found. Install Node 20+." >&2
  exit 1
fi
if ! command -v jq >/dev/null 2>&1; then
  echo "design-scrape: jq not found. brew install jq / apt install jq" >&2
  exit 1
fi

mkdir -p "$OUT"

# Inline Playwright script piped on stdin. We generate a self-contained .mjs
# so npx resolves `playwright` from the nearest node_modules OR installs ephemeral.
SCRIPT="$OUT/.scrape.mjs"
cat > "$SCRIPT" <<'MJS'
import { chromium } from "playwright";
import { writeFileSync } from "node:fs";
import { argv } from "node:process";

const url = argv[2];
const outDir = argv[3];

const browser = await chromium.launch();

async function shot(viewport, name) {
  const ctx = await browser.newContext({ viewport });
  const page = await ctx.newPage();
  await page.goto(url, { waitUntil: "networkidle", timeout: 45000 });
  await page.screenshot({ path: `${outDir}/${name}.png`, fullPage: true });
  await ctx.close();
}

await shot({ width: 1280, height: 900 }, "desktop");
await shot({ width: 375, height: 812 }, "mobile");

const ctx = await browser.newContext({ viewport: { width: 1280, height: 900 } });
const page = await ctx.newPage();
await page.goto(url, { waitUntil: "networkidle", timeout: 45000 });

const tokens = await page.evaluate(() => {
  const g = (sel) => { const el = document.querySelector(sel); return el ? getComputedStyle(el) : null; };
  const body = g("body"); const h1 = g("h1");
  return {
    colors: { background: body?.backgroundColor, text: body?.color, heading: h1?.color },
    typography: { bodyFont: body?.fontFamily, bodySize: body?.fontSize, h1Font: h1?.fontFamily, h1Size: h1?.fontSize },
  };
});

const structure = await page.evaluate(() => ({
  title: document.title,
  sections: document.querySelectorAll("section, [class*='section']").length,
  headings: Array.from(document.querySelectorAll("h1,h2,h3")).map(h => ({ t: h.tagName, x: h.textContent.trim().slice(0,80) })),
}));

writeFileSync(`${outDir}/tokens.json`, JSON.stringify(tokens, null, 2));
writeFileSync(`${outDir}/structure.json`, JSON.stringify(structure, null, 2));

await browser.close();
MJS

echo "[design-scrape] capturing $URL -> $OUT" >&2
if ! npx --yes playwright --version >/dev/null 2>&1; then
  echo "[design-scrape] note: Playwright not yet installed — first run will download Chromium" >&2
fi

node "$SCRIPT" "$URL" "$OUT"
rm -f "$SCRIPT"

echo "[design-scrape] done:"
ls -1 "$OUT"
