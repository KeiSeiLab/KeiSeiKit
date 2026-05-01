# API — Apify (web scraping platform)

Live pricing: WebFetch https://apify.com/pricing before any run >$5. Treat the table below as a starting sketch and always re-verify on the live pricing page.

**Platform plans (sample — re-verify on live pricing page):**

| Plan | $/mo | Credits | CU cost | Max RAM | Retention |
|------|-----:|--------:|--------:|--------:|----------:|
| Free | $0 | $5 | $0.30 | 4-8 GB | 7d |
| Starter | $49 | $49 | $0.30 | 32 GB | 14d |
| Scale | $199 | $199 | $0.25 | 128 GB | 21d |
| Business | $999 | $999 | $0.20 | 256+ GB | 31d |

**CU (Compute Unit) formula:** `CU = Memory(GB) × Duration(hours)`. Browser scraper ≈ 300 pages/CU; HTTP scraper ≈ 3000 pages/CU. Most actors 0.1-5 CU/run.

**Per-actor rates (sample — re-check pricing page before any batch):**
| Platform | Best actor | $/1K | Risk | Free alternative |
|----------|-----------|-----:|------|-----------------|
| YouTube | `apidojo/youtube-scraper` | $0.50 | LOW | **YouTube Data API v3 (FREE, 10K units/day)** |
| LinkedIn | `harvestapi/linkedin-profile-scraper` | $4 (no email) / $10 (email) | **HIGH** | linkedin_scraper (Python) |
| Instagram | `apify/instagram-scraper` (official) | $2.30-2.60 | VERY HIGH | Instaloader |
| Instagram | `apidojo/instagram-scraper` (3rd party) | $0.50 | VERY HIGH | — |
| Facebook | `apify/facebook-posts-scraper` | $5-8 | VERY HIGH | facebook-scraper |
| Telegram | via Apify | $1-3 | LOW | **Telethon/Pyrogram (FREE, MTProto)** |

Prefer free path when available — Telethon (Telegram) and YouTube Data API v3 are 100% FREE and fully featured.

**Proxies:**
- Datacenter — included in plan; $0.6-1.0/IP overage. Blocked by IG/FB on first hit.
- Residential — **$7-8/GB**. Required for Instagram/Facebook. **GDPR risk** for EU targets (BGH Germany Nov 2024: €100/user scraping compensation).
- SERP — $2.50/1K.

**Webhooks:** POST on `ACTOR.RUN.SUCCEEDED` / `.FAILED` → your endpoint receives `runId`, `datasetId`. Use for pipelines; poll only for manual one-offs.

**Input schema validation:** every actor has a JSON schema (`input_schema.json`). Validate inputs client-side before POST — failed inputs still eat CU in the startup phase.

**Legal landscape:** hiQ v. LinkedIn (2022) CFAA ≠ public data; Meta v. Bright Data (2024) Meta lost; **BGH Germany Nov 2024: GDPR Art. 82 → €100 per scraped user**. All 6 major platforms' ToS prohibit scraping (contractual, not criminal).

**LinkedIn HIGH RISK:** `harvestapi` no-cookie actors are safer ($4-10/1K). Cookie-based (`curious_coder`) = ban + ToS exposure. Max 500 profiles/day deep. **Always legal review before EU LinkedIn runs.**

**Forbidden:** LinkedIn batch without legal sign-off (GDPR + ToS); residential proxies against EU targets without documented consent basis; batch runs without per-item cost estimate to `kei-cost-guardian`; using main personal account for any cookie-based actor (curious_coder line); launching an actor before validating input against its `input_schema.json`; paying Apify for Telegram when Telethon is free.
