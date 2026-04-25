# STACK — Vite + React 19 + TypeScript (SPA)

Use for single-page applications, internal dashboards, editors, design tools — surfaces where the page IS the app and SEO / zero-JS don't matter. For marketing use `stack-astro`. For full-stack React with Server Components use `stack-nextjs`.

**Scaffold:** `npm create vite@latest <app> -- --template react-ts`. Dev server via `vite`; production via `vite build` → `dist/` (static files).

**Routing:** `react-router-dom` v7 (data routers, `createBrowserRouter`). One file per route under `src/routes/`. For file-based routing prefer TanStack Router (first-class TS inference).

**Data:**
- Server state → TanStack Query v5 (`useQuery` / `useMutation`). Never `useEffect + fetch`.
- Client state → Zustand or `useState` — pick one per feature, don't layer Redux unless the team already uses it.
- Form state → React Hook Form v7 + Zod resolver (single schema client + server).

**Rendering:** React 19's `useActionState`, `useOptimistic`, and the `use()` hook for promise unwrapping. `Suspense` + `ErrorBoundary` on every route boundary. No conditional rendering that hides suspense errors.

**Types first:** Props/interfaces declared BEFORE the component. Discriminated unions for variant props. `as const` for finite-set string unions. No `any` in new code — use `unknown` + type-guards.

**Env vars:** `import.meta.env.VITE_*` — anything NOT prefixed with `VITE_` is stripped at build time. Secrets → backend, never in `VITE_*` (ships to browser).

**Styling:** Tailwind 4 (via `stack-tailwind`) OR CSS Modules — never both in the same project. `className` + token classes; no inline `style={{}}` except for dynamic CSS custom properties.

**Testing:** Vitest + React Testing Library + Playwright for E2E. Tests co-located next to source (`Component.test.tsx`).

**Deploy target:** the SPA is a static bundle — Cloudflare Pages, Vercel, S3+CloudFront, or any static host. No adapter needed.

**Forbidden:** `create-react-app` (deprecated), `fetch` inside `useEffect` (use TanStack Query), `any` in new code, secret env vars without the `VITE_` prefix (shipped to client), mixing Redux + Zustand in the same feature, CSS-in-JS runtimes (ships extra KB — use Tailwind or CSS Modules).
