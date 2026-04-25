# DOMAIN — Scrapers Tier 3 (Apify / Bright Data paid)

**MANDATORY handoff to `kei-cost-guardian` before ANY paid scraping run.** Tier 3 = fallback, not default. Prove Tier 1 insufficient first.

**Known rates (verify on provider pricing page before launch — rates change):**
- **Apify YouTube** `apidojo/youtube-scraper` — $0.50/1K (free API v3 preferred when quota allows)
- **Apify LinkedIn** `harvestapi/linkedin-profile-scraper` — $4/1K (no email) / $10/1K (with email) — **HIGH LEGAL RISK** (BGH Germany Nov 2024: scraping = 100 EUR GDPR compensation per user)
- **Apify Instagram** `apify/instagram-scraper` — $2.30-2.60/1K · `apidojo/instagram-scraper` — $0.50/1K (cheaper, residential proxies mandatory)
- **Apify Facebook** `apify/facebook-posts-scraper` — $5-8/1K · Bright Data ~$1/1K (10x cheaper at scale)
- **Apify TikTok** — `[VERIFY: https://apify.com/store?search=tiktok]` (report lacks current rate)
- **Apify Telegram** — $1-3/1K, **DON'T USE** — Telethon (Tier 1, FREE) gives 100% functionality
- **Bright Data residential proxies** — ~$7-8/GB (Apify residential add-on same tier)

**Pre-run checklist (hand off to `kei-cost-guardian`):**
1. Dashboard balance — state current Apify credits / Bright Data balance.
2. Pricing page fetched LIVE (WebFetch) — quote rate + timestamp.
3. Running actors — Apify dashboard: show what's already billing.
4. Cost estimate — `N_items × $rate/1K + proxy_GB × $8`. Echo dollars to user BEFORE launch.
5. **1-2 item smoke run first** — verify shape + per-item cost; only then scale.
6. Monitor first 2 min stdout — kill on anomaly, don't let a broken actor burn the run.
7. Log actuals to `memory/{project}.md` after.

**GDPR residential-proxy ban (EU targets):**
- Residential proxies for GDPR-protected data (EU individual profiles) → **DPO sign-off required**.
- Default to datacenter proxies unless the actor mandates residential (Instagram, Facebook).
- XING = DACH = strictest GDPR jurisdiction — prefer XingZap (5 EUR/query, GDPR-compliant) over raw Apify actors.

**Cost tiers (inherit from `domain-paid-apis`):**
- < $5 AUTO · $5-$20 WARN · > $20 STOP + explicit user "yes, launch $N.NN" echo.

**Forbidden:** launching LinkedIn paid scrape without legal-review sign-off in `DECISIONS.md`; cookie-based LinkedIn actors with user's main account (`curious_coder/*` bans accounts); residential proxies on EU individual profiles without DPO approval; batch >100 items without `kei-cost-guardian` estimate; skipping 1-2 item smoke run (failed actor config × N items = N billings); running paid scraper when Tier 1 (YouTube API v3, Telethon, GitHub GraphQL) covers the data; hardcoding Apify tokens in source (use `secrets/*.env`).
