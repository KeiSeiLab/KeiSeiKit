# API — fal.ai (image / video / 3D)

Live pricing: WebFetch https://fal.ai/pricing before any batch >$2. Maintain your own model snapshot in your memory dir to avoid re-verifying every call.

**Model catalog (verify before launch — model IDs and prices change):**

| Asset | Model | Endpoint | Price |
|------|------|----------|-------|
| Hero premium | FLUX.2 Pro | `fal-ai/flux-2-pro` | $0.03-0.045/MP |
| Hero budget | FLUX.1 Dev | `fal-ai/flux/dev` | $0.025/MP |
| 3D icons | Recraft V3 handmade_3d | `fal-ai/recraft/v3/text-to-image` | $0.04 |
| SVG | Recraft V4 Vector | `fal-ai/recraft/v4/text-to-vector` | $0.08 |
| BG removal | Bria RMBG 2.0 | `fal-ai/bria/background/remove` | $0.018 |
| Video budget | LTX 2.0 Fast | `fal-ai/ltx-2/text-to-video/fast` | $0.04/sec |
| Video hero loop | Luma Ray 2 I2V | `fal-ai/luma-dream-machine/ray-2/image-to-video` | $0.50/5sec@540p |
| Video Kling | Kling v3 Pro I2V | `fal-ai/kling-video/v3/pro/image-to-video` | $0.224/sec |
| Video Veo 3 | Veo 3 | `fal-ai/veo3` | $0.20-0.40/sec |
| 3D GLB | Trellis | `fal-ai/trellis` | $0.02 |

**Hard-learned per-model gotchas:**
- **FLUX.2 Pro ZERO-CONFIG** — NO `guidance_scale` (API rejects), `safety_tolerance: "5"`, `enable_prompt_expansion: false`, `image_urls[]` always array (even for 1 ref).
- **Kling O3** — prompt hard limit **2500 chars**; `image_url` NOT `start_image_url` (V3 legacy); `elements` + `voice_ids` can be sent **together on O3 only**; `generate_audio: true` ALWAYS (else silent video).
- **Luma Ray 2** — `loop: true` for hero sections (seamless loop, same first/last frame).
- **Async flow:** POST → `request_id` → poll status → fetch `response_url`. Don't expect sync result.

**NSFW filter:** default ON for Flux/Recraft. `safety_tolerance` raises threshold (higher = more permissive); `"5"` is the documented max. Failed content returns a flagged error, still billed.

**Webhook vs poll:** webhooks need a public HTTPS URL (tunnel with ngrok/CF for local). Poll is fine for <30-min batches.

**Cost discipline:** 1-2 smoke samples before fanning out to ≥5 generations. Full-site budget template: 20 icons + 5 hero + 10 bg + 35 bg-removal + 35 upscale × 2 iterations ≈ $4-8. Hand off to `kei-cost-guardian` on any batch >$5.

**API key:** `FAL_KEY` in `<repo>/.env`. Never in chat, source, curl examples, or git (see `domain-has-secrets.md`).

**Forbidden:** adding `guidance_scale` to FLUX.2 Pro; Kling O3 prompts >2500 chars; launching any batch without kei-cost-guardian handoff; quoting prices from memory for session total >$2 (re-verify via WebFetch); FLUX.2 Pro for plain backgrounds when FLUX.1 Dev does the job (pick cheapest-that-matches-brief); hard-coding `FAL_KEY` in source.
