# TEST — End-to-end (Playwright browser automation)

E2E tests drive a real browser against a real deployed stack and assert user-visible behaviour. Slow + flaky by nature — so discipline matters more than count. One reliable E2E beats ten flaky ones.

**Default tool:** `Playwright` (Microsoft, TS/JS/Python/.NET/Java bindings). Preferred over Cypress because: multi-browser (Chromium / Firefox / WebKit), parallel by default, trace viewer (time-travel debugger), auto-waiting for elements, network interception built-in. [E4, playwright.dev]

Cypress is the runner-up; use only if team already owns it. `Selenium` is legacy — avoid for new E2E.

**Scope:**
- E2E = **critical user journeys only** (login, checkout, primary CRUD flow, signup). Target ~5-15 tests, not 500.
- Everything else (form validation, error states, edge cases) → unit + integration + component tests.
- Rule: if a regression here would be a production incident, it's an E2E candidate.

**Page Object pattern (mandatory):**
```ts
class LoginPage {
  constructor(private page: Page) {}
  async goto() { await this.page.goto('/login'); }
  async login(user: string, pass: string) {
    await this.page.getByLabel('Email').fill(user);
    await this.page.getByLabel('Password').fill(pass);
    await this.page.getByRole('button', { name: 'Sign in' }).click();
  }
}
```
Selectors live in the page object, never in the test. When the UI changes, ONE file updates.

**Selector discipline:**
- Prefer `getByRole` / `getByLabel` / `getByText` (accessibility-anchored, survive CSS refactors).
- Fallback to `data-testid` attributes added purely for tests.
- AVOID CSS class selectors, XPath, nth-child — they break on every style change.

**Test isolation:**
- Each test gets a clean auth state via `storageState` fixtures (login once per project, reuse the cookie jar).
- Each test uses a fresh data scope — either a disposable test tenant, a UUID prefix, or DB truncation in a `beforeEach`.
- NEVER depend on test ordering. Parallel-safe by construction.

**CI headless + tracing:**
- Headless by default, headed only when debugging locally (`--headed --debug`).
- Enable trace on retry: `trace: 'on-first-retry'` — zero overhead on green runs, full forensic on flakes.
- Upload `test-results/` as CI artifact. Open traces with `npx playwright show-trace trace.zip`.
- Video + screenshots on failure: `video: 'retain-on-failure'`, `screenshot: 'only-on-failure'`.

**Flake policy:**
- Retry **at most twice** in CI. If a test retries often, it's a real bug — either in the SUT or the test.
- Quarantine flaky tests (`test.skip()` with a tracked ticket), never silently `retry: 5`.
- Root-cause flakes with the trace viewer, not by adding `waitForTimeout` (always a smell).

**Forbidden:**
- `page.waitForTimeout(ms)` — use auto-waiting locators or explicit `expect(...).toBeVisible()` polls.
- Running E2E against production without a dedicated test account and a rate limit.
- E2E-testing behaviour already covered by a unit/integration test (slow duplication).
- Hardcoded sleeps, hardcoded URLs, hardcoded user credentials in test files (use fixtures + env vars).
