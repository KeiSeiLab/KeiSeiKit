# STACK — Astro 6 (Content + Marketing + Islands)

Use for marketing sites, content-heavy sites, docs, and landing pages. Zero-JS by default; interactivity is opt-in per component via islands.

**When to pick:** the page is >70% static content (marketing, blog, docs, portfolio). For app-like surfaces (dashboards, editors, long session state) prefer `stack-nextjs` or `stack-react-vite`.

**Routing:** file-based (`src/pages/`). `.astro` components render to HTML at build time. Dynamic routes via `[slug].astro` + `getStaticPaths`.

**Islands:** any framework component (React / Svelte / Vue / Solid) renders via an integration and takes a `client:*` directive:

- `client:load` — hydrate immediately (interactive from first paint)
- `client:idle` — hydrate when main thread idle
- `client:visible` — hydrate when visible (default for below-fold widgets)
- `client:media="(max-width: 768px)"` — hydrate only on matching viewport
- `client:only="react"` — skip SSR entirely (client-only components)

No directive = zero JS shipped. Never add one "just in case".

**React integration:** `npx astro add react` → installs `@astrojs/react`. Then import and use `.tsx` components inside `.astro` with a `client:*` directive where interactivity is needed.

**Deploy adapter (Cloudflare default):** `npx astro add cloudflare` → installs `@astrojs/cloudflare`. In `astro.config.mjs` set `output: "server"` (for per-request SSR) or `"hybrid"` (pre-render by default, SSR where opted in). Static-only builds need no adapter — `astro build` emits `dist/`.

**Content collections:** `src/content/<collection>/*.md(x)` + `src/content/config.ts` (Zod schema). Type-safe queries via `getCollection(name)`. Use for blog posts, case studies, docs.

**View Transitions:** `import { ViewTransitions } from "astro:transitions"` — 2 lines in the base layout, zero JS overhead. Pairs well with the `motion-design` skill.

**Env vars:**
- Build-time: `import.meta.env.FOO` (inlined). `PUBLIC_*` prefix is client-visible; everything else is build-host only.
- Runtime (server/SSR): via adapter runtime (`context.locals.runtime.env` on Cloudflare Workers).
- Secrets go in platform env (Cloudflare dashboard / `.dev.vars` locally). NEVER in `astro.config.mjs`.

**Images:** `<Image src={...} />` from `astro:assets` — automatic `srcset`, `sizes`, `width`/`height`, AVIF/WebP fallback. Pair with `web-assets` skill for pipeline details.

**Typical stack:** Astro 6 + TypeScript + Tailwind 4 (via `stack-tailwind`) + `@astrojs/react` for islands + `adapter-cloudflare` + Content Collections. Files > 200 LOC get split (Constructor Pattern).

**Forbidden:** `client:load` on static content, importing React at the top of `.astro` pages that don't render interactive components, secrets in `PUBLIC_*` vars, mixing two UI frameworks without a concrete reason (ships multiple hydration runtimes).
