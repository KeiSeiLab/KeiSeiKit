# DOMAIN — Scraper unified output invariant

All scrapers emit `UnifiedProfile` / `UnifiedContent` via `normalize()`. Provider-specific fields belong in `rawData`, nothing else.

**Schema (minimum fields):**
```
UnifiedProfile {
  platform: 'youtube' | 'linkedin' | 'instagram' | 'facebook' | 'xing' | 'telegram' | 'github' | 'twitter',
  external_id: string,              // platform-native stable ID (PRIMARY dedup key)
  name, username, avatar_url, bio, url,
  followers_count, following_count, posts_count,
  email, phone, website, location,
  company, job_title, industry,     // LinkedIn / XING
  consent: { lawful_basis, source, timestamp },   // GDPR — mandatory
  raw_data: Record<string, unknown>,               // untouched provider response
}
```

**BaseScraper pattern (all new scrapers inherit):**
- 1 scraper = 1 file = 1 platform (Constructor Pattern).
- `fetch()` → raw provider response; `normalize()` → `UnifiedProfile | UnifiedContent`.
- Normalizers live in `src/normalizers/<platform>.(ts|py|rs)` — one cube per platform.
- Never let provider-specific fields leak into DB queries, business logic, or UI. Business code reads ONLY `UnifiedProfile` keys.

**Deduplication:**
- Primary key: `(platform, external_id)` — platform-native stable ID.
- Secondary merge: normalized name + location + company — only when `external_id` missing.
- **Never dedup by email only** — email collisions (shared inboxes, typos, generic `info@`) merge distinct people into one profile.

**Consent flag (GDPR):**
- Every profile record a lawful-basis value (`legitimate_interest` / `consent` / `public_data`).
- Source (which scraper + when) logged per record.
- Right-to-erasure endpoint deletes by `(platform, external_id)` across all tables.

**Forbidden:** writing a scraper that skips `normalize()`; passing raw provider dicts into business logic / DB queries / UI components (breaks Single Source of Truth); deduplication by email alone; persisting a profile without `consent` field populated; putting platform-specific schema into `src/models/` top-level types (belongs in `raw_data` or provider-scoped module); mixing two platforms in one scraper file (Constructor Pattern — split per platform).
