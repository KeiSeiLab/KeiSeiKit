# STACK — SvelteKit (Svelte 5 Runes + TS)

Use for animation-heavy sites, mobile-first interactive surfaces, and apps where smallest-possible runtime matters. Svelte 5 compiles to minimal JS; Runes replace the legacy reactive-label syntax.

**Scaffold:** `npm create svelte@latest <app>` → choose "SvelteKit", TypeScript strict, ESLint + Prettier.

**Routing:** file-based (`src/routes/`). Each route is a folder containing `+page.svelte` (UI) + optionally `+page.server.ts` (server load / actions) / `+page.ts` (universal load) / `+layout.svelte`. Dynamic routes via `[slug]/+page.svelte`.

**Runes (Svelte 5):**
- `$state(x)` — reactive value (replaces `let x = ...` + `$:` label)
- `$derived(expr)` — computed (replaces `$:` derivations)
- `$effect(() => {...})` — side effect (replaces `$:` statements with side effects)
- `$props()` — component props (replaces `export let`)
- `$bindable()` — two-way binding opt-in

No more legacy `export let` / `$: foo = bar` in new code. Runes are the canonical API from Svelte 5 onwards.

**Data flow:**
- `+page.server.ts` `load({ fetch, params })` — runs on server only, DB/secrets OK.
- `+page.ts` `load(...)` — runs both server (SSR) + client (navigation) — no secrets.
- Form actions: `export const actions = { default: async ({ request }) => {...} }` in `+page.server.ts`. Use `<form method="POST">` + progressive enhancement — works without JS.

**Env vars:**
- `$env/static/private` + `$env/dynamic/private` — server-only, secrets OK.
- `$env/static/public` + `$env/dynamic/public` — must be prefixed `PUBLIC_`, ships to client.
- SvelteKit refuses to build if a private env is imported into a client module — enforcement built in.

**Deploy adapter (Cloudflare default):** `npm i -D @sveltejs/adapter-cloudflare` and set it in `svelte.config.js`. Alternatives: `adapter-node`, `adapter-vercel`, `adapter-static`. Cloudflare adapter supports KV / R2 / D1 via `platform.env.*` inside load functions.

**Stores (legacy, still supported):** `writable`, `readable`, `derived` from `svelte/store`. Prefer `$state` in components; use stores only for cross-component shared state that truly needs it.

**Testing:** Vitest for unit + `@testing-library/svelte` for components + Playwright for E2E.

**Forbidden:** legacy `export let` + `$:` label syntax for NEW code (use Runes), `$env/static/private` imported into a client-reachable module, mixing runes + legacy reactivity in the same component, hardcoded secrets in `svelte.config.js` (ships to client bundle), adding React/Vue into a SvelteKit app without a very specific reason.
