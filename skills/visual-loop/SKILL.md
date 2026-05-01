---
name: visual-loop
description: Scaffold + run continuous visual / a11y / responsive regression checks via Playwright. One-command setup per project (config + spec template + npm script + baseline), then `npm run visual-check` produces diff report. Composes with /dev-guard frontend-validator and auto-dev-guard hook.
argument-hint: [project-root] (defaults to CWD)
---

# /visual-loop — Visual / A11y / Responsive Regression Loop

> Polish problem: button moved 4px right after font-weight change, color shifted 2 shades after Tailwind upgrade. Type-check + lint don't catch it. Visual diff does.

## When to use

- First time on a project → scaffolds Playwright config + spec template
- Subsequent runs → executes the existing checks, reports diff
- Triggered by `/dev-guard` 4th agent (frontend-validator) when `package.json` has `visual-check` script
- Triggered by `auto-dev-guard.sh` hook (advisory) when frontend file edited and Playwright present

## Phase 0 — Detect state

1. Walk up from `$ARG` (or CWD) to find project root (`package.json`, `pubspec.yaml`).
2. Stack-detect:
   - Next.js → `app/` route discovery, `next.config.*`
   - Vite + React → `src/routes/` or routes config
   - Vite + Vue → similar
   - SvelteKit → `src/routes/`
   - Astro → `src/pages/`
   - Flutter (web) → `lib/main.dart`, run via `flutter run -d web-server`
3. Check if Playwright is set up:
   - `playwright.config.{ts,js,mjs,cjs}` exists?
   - `node_modules/@playwright/test` installed?
   - npm script `visual-check` defined?

Branch:
- **All present** → skip to Phase 2 (run).
- **Missing pieces** → Phase 1 (scaffold), then Phase 2.

## Phase 1 — Scaffold (one AskUserQuestion batch)

Single click-only confirmation:

```
AskUserQuestion: Set up visual-loop in <project-name>?
Options:
  1. Yes — install Playwright + browsers, scaffold config + specs, add npm script
  2. Lite — config + specs only (assume Playwright + browsers already installed)
  3. Cancel
```

On `Yes`:

```bash
cd <project-root>
npm install --save-dev @playwright/test
npx playwright install --with-deps chromium
```

On `Yes` or `Lite`:

Write `playwright.config.ts`:

```ts
import { defineConfig, devices } from "@playwright/test";

export default defineConfig({
  testDir: "./e2e",
  outputDir: "./.playwright/output",
  snapshotDir: "./.playwright/snapshots",
  reporter: process.env.CI ? "github" : "list",
  use: {
    baseURL: process.env.BASE_URL ?? "http://localhost:3000",
    screenshot: "only-on-failure",
    trace: "retain-on-failure",
  },
  projects: [
    { name: "desktop-chrome",  use: { ...devices["Desktop Chrome"]  } },
    { name: "mobile-iphone",   use: { ...devices["iPhone 14"]       } },
    { name: "tablet-ipad",     use: { ...devices["iPad Pro 11"]     } },
  ],
  webServer: {
    command: "npm run dev",
    url: "http://localhost:3000",
    reuseExistingServer: !process.env.CI,
    timeout: 120_000,
  },
});
```

Write `e2e/visual.spec.ts` — auto-discovers routes via filesystem walk of `app/` (Next.js) or equivalent:

```ts
import { test, expect } from "@playwright/test";
import { readdirSync, statSync } from "node:fs";
import { join } from "node:path";

// Discover routes: Next.js app-router style. Adapt for other stacks.
function discoverRoutes(root = "app"): string[] {
  const out: string[] = [];
  const walk = (dir: string, prefix: string) => {
    let entries: string[];
    try { entries = readdirSync(join(process.cwd(), dir)); }
    catch { return; }
    for (const e of entries) {
      const full = join(dir, e);
      const st = statSync(join(process.cwd(), full));
      if (st.isDirectory()) {
        // Skip route groups, dynamic segments for now (need fixture data).
        if (e.startsWith("(") || e.startsWith("[")) continue;
        walk(full, `${prefix}/${e}`);
      } else if (e === "page.tsx" || e === "page.jsx") {
        out.push(prefix || "/");
      }
    }
  };
  walk(root, "");
  return Array.from(new Set(out));
}

const routes = discoverRoutes();

for (const route of routes) {
  test(`visual: ${route}`, async ({ page }) => {
    await page.goto(route);
    await page.waitForLoadState("networkidle");
    await expect(page).toHaveScreenshot(`${route.replace(/\//g, "_") || "root"}.png`, {
      maxDiffPixelRatio: 0.01,
      fullPage: true,
    });
  });
}
```

Write `e2e/a11y.spec.ts`:

```ts
import { test, expect } from "@playwright/test";
import AxeBuilder from "@axe-core/playwright";

const routes = ["/"]; // extend per project

for (const route of routes) {
  test(`a11y: ${route}`, async ({ page }) => {
    await page.goto(route);
    await page.waitForLoadState("networkidle");
    const results = await new AxeBuilder({ page })
      .withTags(["wcag2a", "wcag2aa", "best-practice"])
      .analyze();
    expect(results.violations).toEqual([]);
  });
}
```

Patch `package.json` scripts:

```json
{
  "scripts": {
    "visual-check": "playwright test e2e/visual.spec.ts",
    "a11y-check": "playwright test e2e/a11y.spec.ts",
    "visual-update": "playwright test e2e/visual.spec.ts --update-snapshots"
  }
}
```

Append to `.gitignore`:

```
.playwright/output/
.playwright/test-results/
```

Snapshot baseline (`.playwright/snapshots/`) **stays in git** — that's the baseline against which future diffs run.

Install `@axe-core/playwright`:

```bash
npm install --save-dev @axe-core/playwright
```

## Phase 2 — First run (establish baseline OR diff)

```bash
cd <project-root>
npm run visual-check
```

First time → creates baseline screenshots (no diff yet). Subsequent runs → diff against baseline. Failures = visual regression.

If baseline doesn't exist (first ever run on this snapshot), `--update-snapshots` runs implicitly. Subsequent failures show diff PNGs in `.playwright/output/`.

## Phase 3 — Triage diff (if any)

If `npm run visual-check` returned non-zero:

```
AskUserQuestion: Visual diff in N route(s):
Options:
  1. Approve as new baseline (npm run visual-update)
  2. Review diffs first (open .playwright/output/<route>/diff.png)
  3. Fix the regression in code (no baseline change)
  4. Cancel
```

Click drives action.

## Composition with other skills

- **`/dev-guard`** frontend-validator agent: step 5 — if `package.json` has `visual-check` script, invoke `npm run visual-check`. Severity: WARN on diff.
- **`/dev-ship`** `frontend-final-gate` agent (Wave 5): step — full visual+a11y+responsive+Lighthouse pass before merge.
- **`auto-dev-guard.sh`** hook: if Playwright present + edit affects routes, spawn background advisory.

## Rules

- **Click-only.** Free-text only for explicit baseline override scenarios.
- **Never delete .playwright/snapshots/** without explicit user approval — it's the baseline.
- **Don't over-fixture.** Phase 1 covers static routes; dynamic routes (`[id]`, route groups) need per-project fixture data — defer to user.
- **Bypass:** if user runs `KEI_DISABLED_HOOKS=auto-dev-guard ...` the hook skips visual check.

## Final report

```
=== VISUAL-LOOP REPORT ===
Project:    <name>
Stack:      <Next.js / Vite / ...>
Routes:     N discovered, M tested
Visual:     PASS / N diff(s)
A11y:       PASS / N violation(s)
Responsive: desktop-chrome / mobile-iphone / tablet-ipad — all PASS / FAIL on N

Baseline:   .playwright/snapshots/ (in git)
Diff path:  .playwright/output/ (gitignored)
```

## Out of scope

- Lighthouse perf — separate `/perf-audit` skill.
- Login-protected routes — caller must script auth fixture.
- E2E user-flow tests — separate from regression baseline.

## References

- Playwright docs: https://playwright.dev
- axe-core/playwright: https://github.com/dequelabs/axe-core-npm
