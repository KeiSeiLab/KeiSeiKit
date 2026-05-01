# API — ElevenLabs (voice)

Live pricing: WebFetch https://elevenlabs.io/pricing before any bulk run [VERIFY: character pricing tier varies by plan].

**MANDATORY 3-step Voice Design flow (order is fixed):**
1. **`designVoice`** — describe voice characteristics (gender, age, accent, style) → returns preview audio + `generated_voice_id` (ephemeral).
2. **`createVoice`** — accept the preview → permanent `voice_id` added to library.
3. **TTS** — synthesize text using the permanent `voice_id`.

Skipping or reordering any step = API error. Ephemeral preview IDs expire — cannot TTS directly from `designVoice` output.

**Models:**
| Model | Use case | Latency | Quality |
|------|---------|---------|---------|
| `eleven_flash_v2_5` | Real-time, low latency (~75ms) | Fastest | Good |
| `eleven_multilingual_v2` | Production, 29 languages | Slower | Best |
| `eleven_turbo_v2_5` | Balanced | Fast | High |

**Pricing [VERIFY: check live pricing page]** — billed per character, plan-gated character quota:
- Free: ~10K chars/mo
- Starter: ~30K chars/mo
- Creator / Pro / Scale — higher quotas, character overage rates vary per plan.
- Voice Design calls also consume characters (preview audio counts).

**TTS params (sane defaults):**
- `stability: 0.5` — higher = more monotone, lower = more expressive (range 0-1)
- `similarity_boost: 0.75` — higher = closer to reference voice
- `style: 0-1` — emotional exaggeration; set 0 for Flash v2 (not supported)
- `use_speaker_boost: true` for Multilingual v2

**Voice ID caching:** once `createVoice` returns a `voice_id`, store it in `memory/{project}.md` or DB. Reuse across TTS calls — re-designing the same voice = wasted characters + non-deterministic result.

**Video integration (if pairing with a video model that supports voice):** `voice_id` flows into the video model's `voice_ids` payload. Per-speaker markers in prompts ONLY when `voice_ids` actually sent.

**Cost tracking:** log per-call `characters_used` + cumulative month-to-date → `memory/{project}.md`. Hand off to `kei-cost-guardian` on any batch expected to exceed 50% of monthly quota.

**Forbidden:** calling TTS without prior `createVoice` (ephemeral preview IDs fail); exceeding plan character quota without `kei-cost-guardian` check (overage billing surprise); committing `voice_id` values into git when they reference private/cloned voices (storage convention — see `domain-has-secrets.md`); re-designing the same voice per-scene instead of caching `voice_id`; skipping the 3-step flow with direct TTS on `generated_voice_id`.
