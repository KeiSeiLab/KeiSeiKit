# DOMAIN — Paid APIs (Anthropic / OpenAI / fal.ai / Apify / Modal / AWS / GCP / ElevenLabs)

A real cost-overrun incident (a job estimated in tens of dollars that actually ran into triple digits on a GPU provider) motivates every rule below.

**MANDATORY pre-launch handoff to `kei-cost-guardian` before ANY paid run:**
1. Dashboard balance — state the current number, not "I think it's roughly".
2. Pricing page — fetch LIVE (WebFetch), not from memory. Rates change.
3. Running jobs — `modal app list` / provider dashboard → show user what's already billing.
4. Cost estimate — formula AND dollars. Example: `N_gpus × hours × $1.10/hr (A10G, verified <today>)`.
5. Single-variant verify — one run succeeds before fanning out to N variants (failed config × N = N billings).
6. Tell user the exact dollar cost BEFORE launch. Explicit GO required for anything > $5.
7. Monitor first 2 minutes of stdout — health check before fan-out.

**Cost tiers:**
- < $5 — AUTO (cost line in report, no confirmation needed)
- $5-$20 — WARN + daily-cap check ($20/day session cap)
- > $20 — STOP, explicit user "yes, launch" with the dollar number echoed back

**Batch ops (Apify, OpenAI batch, ElevenLabs bulk TTS):**
- Estimate whole-batch cost BEFORE first call
- Run 1-2 items to verify shape + per-item cost matches estimate
- THEN fan out; log per-call cost to `memory/{project}.md`

**Known rate ballparks (ALWAYS verify on the live pricing page before launch — rates change):**
- Apify YouTube ~$0.50/1K items · LinkedIn harvest ~$0.50-2/search · Instagram ~$2-3/1K · Telegram FREE via Telethon (direct API)
- Fal.ai Flux / Kling / others — per image or per video, varies by model
- Modal A10G ~$1.10/hr · H100 ~$4.50/hr · B200 ~$8/hr

**Forbidden:** launching without dashboard check; guessing prices; parallel variants without single-variant verify; skipping kei-cost-guardian handoff; running paid compute without logging actuals to `memory/{project}.md` after.
