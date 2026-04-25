# DOMAIN — Scrapers Tier 1 (free APIs + open-source)

**Default to Tier 1. Paid tier only after Tier 1 is proven insufficient** (e.g. GitHub GraphQL FREE covers most dev-profile needs before anything paid).

**Tier 1 providers (FREE, with quota ceilings):**
- **YouTube Data API v3** — 10K units/day, search=100 units (≈100 searches/day), video details=1 unit. Cache aggressively, reuse IDs.
- **Telegram Telethon** (Python, MTProto) — user-account session, `get_participants` capped 200/call, FLOOD_WAIT adaptive. Pyrogram = alt.
- **GitHub GraphQL API v4** — 5K requests/hour authenticated; unauthenticated = 60/hr only.
- **Twitter twscrape** — unofficial, account-pool based, shadowban risk per account. Rotate accounts; never use main.

**GDPR — consent-first pipeline:**
- Discover → normalize → dedup → enrich → save, with explicit consent flag per profile.
- Scraped profile = personal data under GDPR; `lawful basis` recorded per source.
- Right-to-erasure: delete by (platform, external_id) must work.

**Rate & quota hygiene:**
- Persist quota counters per provider per day to `memory/{project}.md` or DB.
- Exponential backoff on 429/rate-limit; never hammer.
- Telethon/twscrape sessions stored in `secrets/` (see `domain-has-secrets`).

**Forbidden:** scraping Telegram with a user account without the user's explicit consent (account ban + ToS); hammering YouTube API quota without caching (10K units burns in minutes); unauthenticated GitHub calls (60/hr = instant lockout on any real job); committing Telethon `.session` files; using your personal Twitter account as the twscrape pool seed; scraping profiles without recording consent/lawful-basis flag.
