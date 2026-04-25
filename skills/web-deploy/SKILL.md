---
name: web-deploy
description: Use when deploying websites — Cloudflare Pages, Vercel, edge functions, caching strategy, Core Web Vitals, CI/CD pipeline, DNS setup. Triggers on "deploy", "hosting", "cloudflare pages", "web vitals", "caching strategy".
arguments:
  - name: command
    description: "Command: init, deploy, perf, cache, dns, ci, compare"
    required: false
  - name: framework
    description: "Framework: astro, next, sveltekit, react-router (auto-detect if omitted)"
    required: false
---

# Web Deployment & Performance

Default target: Cloudflare Pages. Default framework: Astro 6.

## Platform Decision

| Platform | Free Tier | Pro Price | Best For |
|----------|-----------|-----------|----------|
| **Cloudflare Pages** | Unlimited BW, 500 builds/mo | $5/mo | Content sites, marketing (DEFAULT) |
| Vercel | 100GB BW, 100 deploys/day | $20/user/mo | Next.js full-stack apps |
| Netlify | 100GB BW, 300 build min | $19/user/mo | Static + built-in forms |

Cloudflare ecosystem: Workers, D1, R2, KV, Turnstile, Analytics — all free tier.

## Framework Decision

| Framework | Zero JS | Islands | Best For |
|-----------|---------|---------|----------|
| **Astro 6** | Yes | Yes | Content/marketing (DEFAULT) |
| Next.js 16 | No | No | Full-stack React apps |
| SvelteKit | Compiles | No | Animation-heavy, mobile-first |

Astro 6 static output: typical LCP <500ms on CF Pages.

## CDN Caching Strategy

| Asset Type | Cache-Control |
|-----------|---------------|
| Hashed JS/CSS/fonts | `public, max-age=31536000, immutable` |
| HTML pages | `public, max-age=0, s-maxage=3600, stale-while-revalidate=86400` |
| API/dynamic | `public, s-maxage=60, stale-while-revalidate=300` |
| Images | `public, max-age=86400, s-maxage=604800` |

## Core Web Vitals

| Metric | Good | Key Fix |
|--------|------|---------|
| LCP | <2.5s | Preload hero: `fetchpriority="high"`, inline critical CSS, preload fonts |
| INP | <200ms | Break tasks >50ms, `requestIdleCallback`, defer 3rd-party |
| CLS | <0.1 | Width/height on all images/video, `aspect-ratio`, font size-adjust |

## GitHub Actions CI/CD

```yaml
name: Deploy
on: { push: { branches: [main] } }
jobs:
  deploy:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: actions/setup-node@v4
        with: { node-version: 22, cache: npm }
      - run: npm ci && npm run build && npm test
      - uses: cloudflare/wrangler-action@v3
        with:
          apiToken: ${{ secrets.CLOUDFLARE_API_TOKEN }}
          accountId: ${{ secrets.CLOUDFLARE_ACCOUNT_ID }}
          command: pages deploy dist --project-name=my-site
```

Secrets: `CLOUDFLARE_API_TOKEN` + `CLOUDFLARE_ACCOUNT_ID`.

## Cloudflare DNS + SSL

1. Add domain, change nameservers
2. `A @ <ip> Proxied` + `CNAME www @ Proxied`
3. SSL: Full (Strict), Always HTTPS, HSTS
4. www→apex redirect rule (301)

## Edge Functions

| Feature | CF Workers | Vercel Edge |
|---------|-----------|-------------|
| Locations | 330+ | 30+ |
| Cold start | <1ms | <50ms |
| Free | 100K req/day | 1M/month |

## Deploy Checklist

- [ ] Build succeeds, tests pass
- [ ] Lighthouse Performance >90
- [ ] Core Web Vitals green
- [ ] Caching headers per asset type
- [ ] SSL/HTTPS enforced
- [ ] www/apex redirect
- [ ] Error pages (404, 500) configured
- [ ] Security headers: CSP, X-Frame-Options, Referrer-Policy
- [ ] Environment variables in dashboard
