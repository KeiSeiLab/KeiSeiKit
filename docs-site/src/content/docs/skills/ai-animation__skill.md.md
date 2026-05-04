---
title: skill
path: ai-animation/skill.md
dna_hash: sha256:24af9060ec25e415
language: markdown
size_loc: 843
generated: by-keidocs
---

# ai-animation/skill.md

## Public API

- `AI Animation Pipeline` — <!-- migrated from skills/ai-animation/skill.md (lowercase legacy filename) on 2026-05-02 -->
- `Pipeline Overview` — ```
- `Model Selection Matrix [E1-fal.ai docs]` — | Use Case | Model | Endpoint | Cost/sec | Duration | Why |
- `Step 1: Generate Keyframes (nano-banana)` — Generate reference images that will be animated into video.
- `Product floating in space — good for Kling rotation` — nano-banana "premium wireless headphones floating on dark background, studio lighting, centered composition, clean edges" -s 2K -a 16:9 -o keyframe-product
- `Scene for cinematic animation — good for Veo/Kling` — nano-banana "cyberpunk street at golden hour, rain puddles, neon reflections, cinematic composition" -s 2K -a 16:9 -o keyframe-scene
- `Character pose — good for Kling motion` — nano-banana "3D character mascot robot waving, isometric view, white background, Pixar style" -s 1K -a 1:1 -o keyframe-character
- `Hero background — good for Luma loop` — nano-banana "abstract flowing liquid metal, iridescent purple and gold, macro photography" -s 2K -a 16:9 -o keyframe-hero
- `Transparent asset for animation` — nano-banana "floating crystal sphere with inner glow" -t -s 1K -o keyframe-crystal
- `Option A: Upload to fal CDN (recommended for large files)` — image_url = fal_client.upload_file("keyframe-product.png")  # → https://fal.media/files/...
- `Option B: Encode as data URI (inline, faster for small files, no CDN hop)` — image_data = fal_client.encode_file("keyframe-product.png")
- `Then use in any video model` — result = fal_client.run("fal-ai/kling-video/v3/pro/image-to-video", arguments={
- `Step 2: Animate via fal.ai` — ### Prerequisites
- `SYNC — blocks until done (simplest, for single generations)` — result = fal_client.run("fal-ai/veo3", arguments={...})
- `ASYNC — non-blocking` — import asyncio
- `QUEUE with progress tracking — best for long jobs` — async def generate_with_progress():
- `FILE UPLOAD — local file → fal CDN URL` — url = fal_client.upload_file("local-image.png")  # returns https://fal.media/files/...
- `Sync (blocks until done, timeout risk for long generations)` — curl -X POST "https://fal.run/{endpoint}" \
- `Queue submit (non-blocking, recommended)` — REQUEST_ID=$(curl -s -X POST "https://queue.fal.run/{endpoint}" \
- `Check status` — curl -s "https://queue.fal.run/{endpoint}/requests/$REQUEST_ID/status" \
- `Get result` — curl -s "https://queue.fal.run/{endpoint}/requests/$REQUEST_ID" \
- `Webhook (fire and forget)` — curl -X POST "https://queue.fal.run/{endpoint}" \
- `Standard — best quality` — result = fal_client.run(
- `Fast I2V — 50% cheaper for image-to-video` — result = fal_client.run(
- `Text-to-video` — result = fal_client.run(
- `Image-to-video` — result = fal_client.run(
- `Extend existing video` — result = fal_client.run(
- `Step 3: Process for Web (FFmpeg)` — ### Download Generated Video
- `Download from fal.ai URL` — curl -o generated.mp4 "VIDEO_URL_FROM_FAL"
- `Extract frames at 30fps, 1920x1080` — mkdir -p frames/desktop
- `Convert to WebP` — for f in frames/desktop/*.png; do
- `Mobile version (fewer frames, smaller)` — mkdir -p frames/mobile
- `Web-optimized MP4 (H.264 for max compat)` — ffmpeg -i generated.mp4 \
- `WebM (VP9, smaller, modern browsers)` — ffmpeg -i generated.mp4 \
- `AV1 (smallest, newer browsers)` — ffmpeg -i generated.mp4 \
- `Every frame is keyframe = instant seeking` — ffmpeg -i generated.mp4 \
- `High quality GIF with palette optimization` — ffmpeg -i generated.mp4 \
- `Step 4: Integrate into Web` — ### Hero Video Background
- `Cost Calculator [E1-verified]` — ### Per-Video Costs (no audio, cheapest resolution)
- `Full Pipeline Examples` — ### Example 1: Apple-Style Product Scroll
- `1. Keyframe` — nano-banana "premium smart watch floating on dark background, studio rim lighting, centered" -s 2K -a 16:9 -o watch-keyframe
- `2. Animate (product rotation)` — python3 -c "
- `3. Download & extract frames` — curl -o watch-rotation.mp4 "VIDEO_URL"
- `Convert to WebP` — for dir in public/frames/watch/desktop public/frames/watch/mobile; do
- `4. → Use scroll-animation skill for GSAP canvas playback` — ```
- `1. Keyframe` — nano-banana "abstract flowing liquid metal, iridescent, macro photography, dark background" -s 2K -a 16:9 -o hero-bg
- `2. Animate (seamless loop)` — python3 -c "
- `3. Optimize for web autoplay` — curl -o hero-loop-raw.mp4 "VIDEO_URL"
- `1. Generate 5 keyframes` — for i in 1 2 3 4 5; do
- `2. Animate all with LTX (cheapest)` — python3 << 'EOF'
- `Prompting Guide for Video Models` — ### Motion Prompts (DO)
- `Integration with Other Skills` — | Skill | Role | When |
- `nano-banana via fal.ai API (Alternative to CLI)` — If the nano-banana CLI is not available, use the fal.ai endpoint directly:
- `Generate keyframe via fal.ai nano-banana (Gemini image gen)` — result = fal_client.run("fal-ai/nano-banana", arguments={
- `Cost: $0.039 per image` — 
- `Now chain directly into video generation (no upload needed — already a URL!)` — video_result = fal_client.run("fal-ai/kling-video/v3/pro/image-to-video", arguments={
- `curl version` — KEYFRAME_URL=$(curl -s -X POST "https://fal.run/fal-ai/nano-banana" \
- `Workflow Summary` — 1. **Define goal** — what animation, where on page, what triggers it?

## Related

- parent: `ai-animation`

## Discussion

<script src="https://giscus.app/client.js"
        data-repo="KeiSei84/KeiSeiKit-1.0"
        data-repo-id="PLACEHOLDER_REPO_ID"
        data-category="wiki-comments"
        data-category-id="PLACEHOLDER_CATEGORY_ID"
        data-mapping="pathname"
        data-strict="0"
        data-reactions-enabled="1"
        data-emit-metadata="0"
        data-input-position="bottom"
        data-theme="preferred_color_scheme"
        data-lang="en"
        data-loading="lazy"
        crossorigin="anonymous"
        async></script>
