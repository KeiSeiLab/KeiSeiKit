# STACK — Next.js 15/16 (App Router + TS + Server Components)

Use for browser/DOM work. TypeScript is the default for this stack; consider Rust→wasm where viable.

**Routing:** App Router (`app/`) — NOT Pages Router (`pages/`). Server Components by default; `"use client"` directive ONLY on components that need `useState` / `useEffect` / event handlers / browser APIs.

**Data flow:**
- Read: Server Components call DB/API directly. No client-side fetching for initial render.
- Mutate: Server Actions (`"use server"` functions) — NOT ad-hoc API routes unless a third party needs to call them.
- Cache: `fetch()` in Server Components uses Next's fetch cache; opt out with `cache: "no-store"` or `revalidate: N`.

**ORM:** Drizzle OR Prisma — pick ONE per project, never both. Drizzle preferred for edge-runtime compatibility (Cloudflare Workers).

**Env vars:**
- Server-only: `process.env.FOO` (never leaks to client bundle).
- Client-visible: `process.env.NEXT_PUBLIC_FOO` — everything else is redacted in the browser.
- Secrets: platform vars (Vercel / Cloudflare), `.env.local` locally, NEVER in `next.config.js` (ships to client).

**Typical paid-AI stack:** Next.js 16 + TypeScript + Drizzle/SQLite + Tailwind 4 + shadcn. Files > 200 LOC get split on-the-spot (Constructor Pattern). For paid AI calls, track cost in integer microdollars (1 USD = 1e6 μ$) — floats forbidden for money.

**Forbidden:** Pages Router for new routes, `"use client"` at the top of pages that don't need interactivity (ships 30-100kb extra JS), Drizzle + Prisma together, secrets in `next.config.js` or inside `NEXT_PUBLIC_*`.
