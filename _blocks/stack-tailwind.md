# STACK — Tailwind CSS 4 (compositional add-on)

This is a **compositional** block — it does NOT stand alone. Layer on top of `stack-nextjs`, `stack-react-vite`, `stack-astro`, or `stack-sveltekit`. Any of those + this = the canonical 2026 "Tailwind project" shape.

**Version:** Tailwind 4.x — ships as a Vite / PostCSS / CLI plugin, NOT as a dependency you import into your framework code. Config lives in CSS via `@theme` (not `tailwind.config.ts` — that is v3).

**Minimal setup (v4):**
```css
/* src/styles/app.css */
@import "tailwindcss";

@theme {
  --color-brand: oklch(0.6 0.2 250);
  --font-display: "Fraunces Variable", serif;
  --font-body: "Inter Variable", sans-serif;
  --radius-card: 0.75rem;
}
```

Any `--color-*`, `--font-*`, `--radius-*`, `--spacing-*`, `--breakpoint-*` declared in `@theme` auto-generates utilities (`bg-brand`, `font-display`, `rounded-card`, etc.).

**Design tokens are CSS custom properties**, not JS config. Same tokens reachable from runtime (`var(--color-brand)`) + Tailwind classes (`text-brand`). Single source of truth; no duplication.

**Dark mode:** `@custom-variant dark (&:where(.dark, .dark *))` (or `@media (prefers-color-scheme: dark)` for system-driven). Then `dark:bg-neutral-900` works as expected.

**Utilities forbidden in new code:** `@apply` in component CSS (makes purge harder, obscures which utilities render). Use the class attribute directly, or extract to a real component.

**Class composition:** use `clsx` or `tailwind-merge` (`cn()` helper pattern) for conditional classes. Never `className={"bg-red " + (active ? "opacity-100" : "opacity-0")}` — use `cn()`.

**Component libraries:** shadcn/ui (copy-paste, source-owned), Radix primitives, Headless UI — all compatible. Avoid UI kits that ship their own runtime CSS (MUI, Chakra) on top of Tailwind — the two design systems will fight.

**Forbidden:** `tailwind.config.js` for NEW v4 projects (use `@theme` in CSS), `@apply` beyond tiny one-offs, mixing Tailwind with MUI / Chakra / Bootstrap, hardcoded hex colors in `className` (`bg-[#ff0000]`) outside prototyping — those bypass the token system and drift.
