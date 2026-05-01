---
name: ai-animation
description: Use when creating AI-powered animations and videos — nano-banana keyframes → Kling/Luma/Veo video generation via fal.ai → web-ready output. Full pipeline from concept to scroll-driven animation or video embed. Triggers on "AI animation", "animate image", "AI video", "Kling", "image to video", "анимируй", "сделай видео из картинки".
arguments:
  - name: pipeline
    description: "Pipeline: keyframe-to-video, image-to-video, text-to-video, video-to-web (default: keyframe-to-video)"
    required: false
  - name: model
    description: "Model: kling, luma, veo, ltx, pixverse (default: auto-select by use case)"
    required: false
  - name: style
    description: "Visual style or reference for generation"
    required: false
---

# AI Animation Pipeline

> nano-banana (keyframes) → fal.ai (video gen) → FFmpeg (web-ready) → scroll/embed
> Prices verified 2026-03-13 [E1-fal.ai API docs]

## Pipeline Overview

```
Concept / Prompt
  │
  ├─→ [1. Keyframes] nano-banana → reference images (poses, angles, scenes)
  │
  ├─→ [2. Animate] fal.ai → Kling / Luma / Veo → MP4 video
  │
  ├─→ [3. Process] FFmpeg → frame sequence / optimized video / GIF
  │
  └─→ [4. Integrate] scroll-animation / video embed / motion-design
```

---

## Model Selection Matrix [E1-fal.ai docs]

| Use Case | Model | Endpoint | Cost/sec | Duration | Why |
|----------|-------|----------|----------|----------|-----|
| **Product reveal** | Kling v3 Pro | `fal-ai/kling-video/v3/pro/image-to-video` | $0.112 (no audio) / $0.168 (audio) | 3-15s | Best motion, multi-prompt, elements |
| **Hero loop** | Luma Ray 2 | `fal-ai/luma-dream-machine/ray-2/image-to-video` | $0.10/sec @540p | 5s, 9s | `loop: true`, start+end keyframes |
| **Cinematic** | Veo 3 | `fal-ai/veo3` | $0.20 (no audio) / $0.40 (audio) | 4-8s | Best quality + audio, 1080p |
| **Cinematic (fast)** | Veo 3 Fast | `fal-ai/veo3/fast/image-to-video` | $0.10 (no audio) / $0.15 (audio) | 4-8s | 50% cheaper I2V |
| **Bulk/cheap** | LTX 2.0 | `fal-ai/ltx-2/text-to-video/fast` | $0.04 @1080p | 6-20s | Cheapest, up to 4K, extend/retake |
| **Stylized** | PixVerse v4.5 | `fal-ai/pixverse/v4.5/image-to-video` | $0.03-0.08/sec | 5-8s | anime/3d/clay/comic/cyberpunk + camera presets |

### Quick Decision

- Loop for hero section? → **Luma Ray 2** (loop: true)
- Animate a product/design comp? → **Kling v3 Pro**
- Maximum quality, budget OK? → **Veo 3**
- Need 10+ videos cheap? → **LTX 2.0**
- Specific art style? → **PixVerse v4.5**

---

## Step 1: Generate Keyframes (nano-banana)

Generate reference images that will be animated into video.

### Best Practices for Animation Keyframes

```bash
# Product floating in space — good for Kling rotation
nano-banana "premium wireless headphones floating on dark background, studio lighting, centered composition, clean edges" -s 2K -a 16:9 -o keyframe-product

# Scene for cinematic animation — good for Veo/Kling
nano-banana "cyberpunk street at golden hour, rain puddles, neon reflections, cinematic composition" -s 2K -a 16:9 -o keyframe-scene

# Character pose — good for Kling motion
nano-banana "3D character mascot robot waving, isometric view, white background, Pixar style" -s 1K -a 1:1 -o keyframe-character

# Hero background — good for Luma loop
nano-banana "abstract flowing liquid metal, iridescent purple and gold, macro photography" -s 2K -a 16:9 -o keyframe-hero

# Transparent asset for animation
nano-banana "floating crystal sphere with inner glow" -t -s 1K -o keyframe-crystal
```

### nano-banana → fal.ai Upload Workflow

nano-banana outputs local PNG files. To use as input for fal.ai video models:

```python
import fal_client

# Option A: Upload to fal CDN (recommended for large files)
image_url = fal_client.upload_file("keyframe-product.png")  # → https://fal.media/files/...

# Option B: Encode as data URI (inline, faster for small files, no CDN hop)
image_data = fal_client.encode_file("keyframe-product.png")

# Then use in any video model
result = fal_client.run("fal-ai/kling-video/v3/pro/image-to-video", arguments={
    "start_image_url": image_url,  # or image_data
    "prompt": "the headphones slowly rotate",
    "duration": "5"
})
```

**Optimal nano-banana settings per video model:**

| Video Model | nano-banana Size | Aspect | Notes |
|------------|-----------------|--------|-------|
| Kling v3 Pro | `-s 2K` | `-a 16:9` or `-a 9:16` or `-a 1:1` | Match `aspect_ratio` param |
| Luma Ray 2 | `-s 2K` | `-a 16:9` or any of 6 ratios | 540p output, 2K input still better |
| Veo 3 | `-s 2K` | `-a 16:9` or `-a 9:16` only | **Must be 720p+ and <8MB** |
| LTX 2.0 | `-s 2K` | `-a 16:9` | Flexible, handles most inputs |
| PixVerse v4.5 | `-s 2K` | match target | Resolution matches input |

### Keyframe Prompting Rules

1. **Clean edges** — AI video models struggle with busy/cluttered compositions
2. **Centered subject** — off-center subjects may drift during animation
3. **Consistent lighting** — dramatic lighting gives video models more to work with
4. **Simple backgrounds** — solid/gradient backgrounds → cleaner motion
5. **Describe potential motion** — "floating", "flowing", "spinning" primes the composition
6. **16:9 for video** — match the target video aspect ratio in the keyframe
7. **High resolution** — use `-s 2K` minimum, video models downsample better than upscale

---

## Step 2: Animate via fal.ai

### Prerequisites

```bash
pip install fal-client
export FAL_KEY="your-key"
```

### fal_client SDK Patterns (Python)

```python
import fal_client

# SYNC — blocks until done (simplest, for single generations)
result = fal_client.run("fal-ai/veo3", arguments={...})
video_url = result["video"]["url"]

# ASYNC — non-blocking
import asyncio
async def generate():
    result = await fal_client.run_async("fal-ai/veo3", arguments={...})
    return result["video"]["url"]

# QUEUE with progress tracking — best for long jobs
async def generate_with_progress():
    handler = await fal_client.submit_async("fal-ai/veo3", arguments={...})
    logs_index = 0
    async for event in handler.iter_events(with_logs=True):
        if isinstance(event, fal_client.Queued):
            print(f"Queue position: {event.position}")
        elif isinstance(event, (fal_client.InProgress, fal_client.Completed)):
            for log in event.logs[logs_index:]:
                print(log["message"])
            logs_index = len(event.logs)
    result = await handler.get()
    return result["video"]["url"]

# FILE UPLOAD — local file → fal CDN URL
url = fal_client.upload_file("local-image.png")  # returns https://fal.media/files/...
data_uri = fal_client.encode_file("local-image.png")  # inline data URI, no upload
```

### REST API Pattern (curl)

```bash
# Sync (blocks until done, timeout risk for long generations)
curl -X POST "https://fal.run/{endpoint}" \
  -H "Authorization: Key $FAL_KEY" \
  -H "Content-Type: application/json" \
  -d '{"prompt": "...", ...}'

# Queue submit (non-blocking, recommended)
REQUEST_ID=$(curl -s -X POST "https://queue.fal.run/{endpoint}" \
  -H "Authorization: Key $FAL_KEY" \
  -H "Content-Type: application/json" \
  -d '{"prompt": "...", ...}' | jq -r '.request_id')

# Check status
curl -s "https://queue.fal.run/{endpoint}/requests/$REQUEST_ID/status" \
  -H "Authorization: Key $FAL_KEY"

# Get result
curl -s "https://queue.fal.run/{endpoint}/requests/$REQUEST_ID" \
  -H "Authorization: Key $FAL_KEY"

# Webhook (fire and forget)
curl -X POST "https://queue.fal.run/{endpoint}" \
  -H "Authorization: Key $FAL_KEY" \
  -H "Content-Type: application/json" \
  -H "X-Fal-Webhook-Url: https://your-server.com/webhook" \
  -d '{"prompt": "..."}'
```

### Kling v3 Pro — Image to Video [E1-verified]

Best for: product reveals, design comp animation, character motion, multi-shot sequences.

```python
import fal_client

result = fal_client.run(
    "fal-ai/kling-video/v3/pro/image-to-video",
    arguments={
        "prompt": "the headphones slowly rotate 360 degrees, studio lighting, smooth motion",
        "start_image_url": "https://...keyframe-product.png",
        "duration": "5",           # "3" to "15" seconds (string)
        "aspect_ratio": "16:9",    # 16:9, 9:16, 1:1
        "generate_audio": False,   # True adds $0.056/sec
    }
)
video_url = result["video"]["url"]
print(f"Video: {video_url}")
```

```javascript
// Node.js
import { fal } from "@fal-ai/client";

const result = await fal.subscribe("fal-ai/kling-video/v3/pro/image-to-video", {
  input: {
    prompt: "the headphones slowly rotate 360 degrees",
    start_image_url: "https://...keyframe.png",
    duration: "5",
    aspect_ratio: "16:9",
    generate_audio: false,
  },
  logs: true,
  onQueueUpdate: (update) => {
    if (update.status === "IN_PROGRESS") {
      update.logs.map((log) => log.message).forEach(console.log);
    }
  },
});
```

**Parameters:**
| Param | Values | Default | Notes |
|-------|--------|---------|-------|
| `prompt` | string | — | Describe MOTION, not scene. Mutually exclusive with `multi_prompt` |
| `start_image_url` | URL | required | Source keyframe (**not** `image_url`) |
| `end_image_url` | URL | — | End frame for interpolation |
| `duration` | `"3"`-`"15"` | `"5"` | String, per-second increments |
| `aspect_ratio` | `16:9`, `9:16`, `1:1` | `16:9` | |
| `generate_audio` | bool | `true` | Native audio (Chinese/English) |
| `negative_prompt` | string | `"blur, distort, and low quality"` | |
| `cfg_scale` | float | `0.5` | Prompt adherence strength |
| `multi_prompt` | array | — | Multi-shot: `[{"prompt": "...", "duration": "5"}, ...]` |
| `elements` | array | — | Character/object consistency via reference images |

**Cost [E1]:**
- No audio: $0.112/sec → 5s = $0.56, 10s = $1.12
- With audio: $0.168/sec → 5s = $0.84, 10s = $1.68
- With voice control: $0.196/sec

**Multi-shot example:**
```python
result = fal_client.run("fal-ai/kling-video/v3/pro/image-to-video", arguments={
    "start_image_url": "https://...keyframe.png",
    "multi_prompt": [
        {"prompt": "camera orbits around the product slowly", "duration": "5"},
        {"prompt": "zoom in to the detail panel", "duration": "5"}
    ],
    "generate_audio": False,
})
```

**curl:**
```bash
curl -X POST "https://fal.run/fal-ai/kling-video/v3/pro/image-to-video" \
  -H "Authorization: Key $FAL_KEY" \
  -H "Content-Type: application/json" \
  -d '{"start_image_url": "https://...keyframe.png", "prompt": "rotate slowly", "duration": "5", "generate_audio": false}'
```

**Prompting for motion:**
- Describe WHAT MOVES: "the camera slowly orbits around the product"
- NOT the scene: ~~"beautiful product on dark background"~~
- Be specific: "rotate 90 degrees clockwise" > "rotate"
- Keep it simple: one primary motion per clip

### Luma Ray 2 — Looping Hero Videos [E1-verified]

Best for: hero section backgrounds, ambient loops, living textures, start→end interpolation.

```python
result = fal_client.run(
    "fal-ai/luma-dream-machine/ray-2/image-to-video",
    arguments={
        "prompt": "gentle flowing motion, seamless loop",
        "image_url": "https://...keyframe-hero.png",
        "loop": True,           # seamless loop!
        "resolution": "540p",   # 540p, 720p (2x), 1080p (4x)
        "duration": "5s",       # "5s" or "9s"
        "aspect_ratio": "16:9",
    }
)
video_url = result["video"]["url"]
```

**Key features:**
- `loop: True` → seamless loops for `<video loop autoplay>` hero sections
- `end_image_url` → interpolate between start and end keyframes (морфинг!)

**Start→End keyframe example:**
```python
result = fal_client.run("fal-ai/luma-dream-machine/ray-2/image-to-video", arguments={
    "prompt": "smooth transformation between the two states",
    "image_url": "https://...keyframe-start.png",
    "end_image_url": "https://...keyframe-end.png",
    "duration": "5s",
    "aspect_ratio": "16:9",
})
```

**Parameters:**
| Param | Values | Default | Notes |
|-------|--------|---------|-------|
| `prompt` | string | required | Min 3, max 5000 chars |
| `image_url` | URL | required | Start frame |
| `end_image_url` | URL | — | End frame (interpolation) |
| `loop` | bool | `false` | End blended with beginning |
| `duration` | `"5s"`, `"9s"` | `"5s"` | 9s costs 2x |
| `resolution` | `"540p"`, `"720p"`, `"1080p"` | `"540p"` | 720p=2x, 1080p=4x cost |
| `aspect_ratio` | `16:9`, `9:16`, `4:3`, `3:4`, `21:9`, `9:21` | `"16:9"` | |

**Cost [E1]:**
| Resolution | 5s | 9s |
|------------|----|----|
| 540p | $0.50 | $1.00 |
| 720p | $1.00 | $2.00 |
| 1080p | $2.00 | $4.00 |

**curl:**
```bash
curl -X POST "https://fal.run/fal-ai/luma-dream-machine/ray-2/image-to-video" \
  -H "Authorization: Key $FAL_KEY" \
  -H "Content-Type: application/json" \
  -d '{"prompt": "gentle flowing motion, seamless loop", "image_url": "https://...hero.png", "loop": true, "resolution": "540p", "duration": "5s"}'
```

### Veo 3 — Maximum Quality [E1-verified]

Best for: cinematic hero videos, with audio. Two variants: standard and fast.

```python
# Standard — best quality
result = fal_client.run(
    "fal-ai/veo3",
    arguments={
        "prompt": "cinematic reveal of headphones, dramatic lighting, camera dollies in slowly",
        "image_url": "https://...keyframe.png",  # optional for text-to-video
        "duration": "8s",          # "4s", "6s", "8s"
        "resolution": "1080p",     # "720p", "1080p"
        "aspect_ratio": "16:9",    # "16:9", "9:16"
        "generate_audio": True,    # unique: audio generation
    }
)
video_url = result["video"]["url"]

# Fast I2V — 50% cheaper for image-to-video
result = fal_client.run(
    "fal-ai/veo3/fast/image-to-video",
    arguments={
        "prompt": "camera slowly orbits the product",
        "image_url": "https://...keyframe.png",
        "duration": "8s",
        "generate_audio": False,
    }
)
```

**Unique:** Veo 3 generates **audio** with the video. `safety_tolerance` 1-6 (1=strict, 6=permissive). `auto_fix` auto-rewrites failing prompts. `seed` for reproducibility.

**Cost [E1]:**
| Variant | No audio | With audio |
|---------|----------|------------|
| Standard | $0.20/sec | $0.40/sec |
| Fast I2V | $0.10/sec | $0.15/sec |

8s standard with audio = $3.20. 8s fast I2V no audio = $0.80.

**Image input constraints (Veo 3):** Max 8MB, must be 720p+, formats: PNG/JPEG/WebP. Aspect must be 16:9 or 9:16.

**curl (fast I2V):**
```bash
curl -X POST "https://fal.run/fal-ai/veo3/fast/image-to-video" \
  -H "Authorization: Key $FAL_KEY" \
  -H "Content-Type: application/json" \
  -d '{"prompt": "gentle camera movement, ambient sounds", "image_url": "https://...keyframe.png", "duration": "8s", "generate_audio": true}'
```

### LTX 2.0 Fast — Budget Option [E1-verified]

Best for: bulk generation, prototyping, long videos up to 20s, up to 4K.

```python
# Text-to-video
result = fal_client.run(
    "fal-ai/ltx-2/text-to-video/fast",
    arguments={
        "prompt": "flowing abstract particles, dark background",
        "duration": 6,            # 6, 8, 10, 12, 14, 16, 18, 20 seconds
        "resolution": "1080p",    # "1080p", "1440p", "2160p"
        "fps": 25,                # 25 or 50
        "generate_audio": True,
    }
)

# Image-to-video
result = fal_client.run(
    "fal-ai/ltx-2/image-to-video/fast",
    arguments={
        "prompt": "gentle camera movement revealing details",
        "image_url": "https://...keyframe.png",
        "duration": 6,
        "resolution": "1080p",
    }
)

# Extend existing video
result = fal_client.run(
    "fal-ai/ltx-2/extend-video",
    arguments={
        "video_url": "https://...existing.mp4",
        "prompt": "continue the motion smoothly",
        "duration": 5,            # max 20s total
        "mode": "end",            # "start" or "end"
    }
)
```

**Cost [E1]:** $0.04/sec @1080p, $0.08/sec @1440p, $0.16/sec @2160p
6s @1080p = $0.24. **5x cheaper** than Kling (no audio).

**Extra endpoints:** `extend-video` (extend existing), `retake-video` (regenerate segment), `audio-to-video` (audio→video).

### PixVerse v4.5 — Stylized [E1-verified]

Best for: 3D render, cyberpunk, anime, clay, comic styles. Has camera presets.

```python
result = fal_client.run(
    "fal-ai/pixverse/v4.5/image-to-video",
    arguments={
        "prompt": "cyberpunk city comes alive, neon signs flicker",
        "image_url": "https://...keyframe.png",
        "style": "cyberpunk",     # anime, 3d_animation, clay, comic, cyberpunk
        "camera_movement": "smooth_zoom_in",  # see list below
        "resolution": "720p",     # 360p, 540p, 720p, 1080p
        "duration": "5",          # "5" or "8" (8s = 2x cost, 1080p = 5s only)
    }
)
```

**Camera presets:** `horizontal_left`, `horizontal_right`, `vertical_up`, `vertical_down`, `zoom_in`, `zoom_out`, `crane_up`, `quickly_zoom_in`, `quickly_zoom_out`, `smooth_zoom_in`, `camera_rotation`, `robo_arm`, `super_dolly_out`, `whip_pan`, `hitchcock`, `left_follow`, `right_follow`, `pan_left`, `pan_right`, `fix_bg`

**Cost [E1]:**
| Resolution | 5s | 8s (2x) |
|------------|----|----|
| 360p/540p | $0.15 | $0.30 |
| 720p | $0.20 | $0.40 |
| 1080p | $0.40 | — (5s only) |

---

## Step 3: Process for Web (FFmpeg)

### Download Generated Video

```bash
# Download from fal.ai URL
curl -o generated.mp4 "VIDEO_URL_FROM_FAL"
```

### Route A: Frame Sequence (for scroll animations)

Use when the video will be played via GSAP ScrollTrigger or canvas scroll.

```bash
# Extract frames at 30fps, 1920x1080
mkdir -p frames/desktop
ffmpeg -i generated.mp4 -vf "fps=30,scale=1920:1080" frames/desktop/frame_%04d.png

# Convert to WebP
for f in frames/desktop/*.png; do
  cwebp -q 80 "$f" -o "${f%.png}.webp"
  rm "$f"
done

# Mobile version (fewer frames, smaller)
mkdir -p frames/mobile
ffmpeg -i generated.mp4 -vf "fps=15,scale=960:540" frames/mobile/frame_%04d.png
for f in frames/mobile/*.png; do
  cwebp -q 80 "$f" -o "${f%.png}.webp"
  rm "$f"
done
```

→ Then use `scroll-animation` skill for GSAP ScrollTrigger canvas playback.

### Route B: Optimized Video (for autoplay/embed)

Use for hero video backgrounds, embedded players.

```bash
# Web-optimized MP4 (H.264 for max compat)
ffmpeg -i generated.mp4 \
  -c:v libx264 -preset slow -crf 23 \
  -c:a aac -b:a 128k \
  -movflags +faststart \
  -vf "scale=1920:1080" \
  hero-video.mp4

# WebM (VP9, smaller, modern browsers)
ffmpeg -i generated.mp4 \
  -c:v libvpx-vp9 -crf 30 -b:v 2M \
  -c:a libopus -b:a 128k \
  -vf "scale=1920:1080" \
  hero-video.webm

# AV1 (smallest, newer browsers)
ffmpeg -i generated.mp4 \
  -c:v libaom-av1 -crf 30 -b:v 0 \
  -c:a libopus -b:a 128k \
  -vf "scale=1920:1080" \
  hero-video.av1.mp4
```

### Route C: Scrub-Optimized Video (for scroll-driven playback)

```bash
# Every frame is keyframe = instant seeking
ffmpeg -i generated.mp4 \
  -c:v libx264 -preset slow -crf 23 \
  -g 1 -an -movflags +faststart \
  -vf "scale=1920:1080" \
  scrub-video.mp4
```

### Route D: Looping GIF (for social/email)

```bash
# High quality GIF with palette optimization
ffmpeg -i generated.mp4 \
  -vf "fps=15,scale=480:-1:flags=lanczos,split[s0][s1];[s0]palettegen[p];[s1][p]paletteuse" \
  -loop 0 output.gif
```

---

## Step 4: Integrate into Web

### Hero Video Background

```html
<section class="hero">
  <video autoplay loop muted playsinline class="hero-video">
    <source src="hero-video.av1.mp4" type="video/mp4; codecs=av01.0.05M.08">
    <source src="hero-video.webm" type="video/webm">
    <source src="hero-video.mp4" type="video/mp4">
  </video>
  <div class="hero-content">
    <h1>Product Name</h1>
  </div>
</section>

<style>
.hero { position: relative; height: 100vh; overflow: hidden; }
.hero-video {
  position: absolute; inset: 0;
  width: 100%; height: 100%;
  object-fit: cover;
}
.hero-content {
  position: relative; z-index: 1;
  display: grid; place-items: center;
  height: 100%;
}
</style>
```

### Scroll-Driven Frame Sequence

→ Invoke `scroll-animation` skill with the frame sequence from Step 3A.

### Reduced Motion Fallback

```html
<picture>
  <source srcset="keyframe-hero.avif" type="image/avif">
  <source srcset="keyframe-hero.webp" type="image/webp">
  <img src="keyframe-hero.jpg" alt="Hero" class="hero-fallback">
</picture>

<style>
.hero-video { display: block; }
.hero-fallback { display: none; }

@media (prefers-reduced-motion: reduce) {
  .hero-video { display: none; }
  .hero-fallback { display: block; }
}
</style>
```

---

## Cost Calculator [E1-verified]

### Per-Video Costs (no audio, cheapest resolution)

| Model | 5s | 10s | With keyframe ($0.07) |
|-------|-----|------|-----------------------|
| Kling v3 Pro (no audio) | $0.56 | $1.12 | $0.63 / $1.19 |
| Kling v3 Pro (audio) | $0.84 | $1.68 | $0.91 / $1.75 |
| Luma Ray 2 @540p | $0.50 | — | $0.57 |
| Luma Ray 2 @1080p | $2.00 | — | $2.07 |
| Veo 3 Standard (no audio) | $1.00 | — | $1.07 |
| Veo 3 Fast I2V (no audio) | $0.50 | — | $0.57 |
| LTX 2.0 @1080p | $0.20 | $0.40 | $0.27 / $0.47 |
| PixVerse v4.5 @720p | $0.20 | — | $0.27 |

### Landing Page Budget Estimates

| Scenario | Videos | Model | Est. Cost |
|----------|--------|-------|-----------|
| 1 hero loop @540p | 1 | Luma | ~$0.57 |
| Product reveal 10s | 1 | Kling no audio | ~$1.19 |
| Product + hero | 2 | Kling + Luma | ~$1.20 |
| Full cinematic page | 3-5 | Mixed | ~$2-5 |
| Bulk content (10 clips) | 10 | LTX 2.0 | ~$2.70 |
| Max quality (Veo 3 audio) | 3 | Veo 3 Standard | ~$10+ |

### Before Running — API Cost Guard

1. Check fal.ai dashboard balance
2. Calculate exact cost: `duration × cost_per_sec + keyframe_cost`
3. Generate ONE test clip first
4. Review quality before batch
5. Tell user total cost before proceeding

---

## Full Pipeline Examples

### Example 1: Apple-Style Product Scroll

```bash
# 1. Keyframe
nano-banana "premium smart watch floating on dark background, studio rim lighting, centered" -s 2K -a 16:9 -o watch-keyframe

# 2. Animate (product rotation)
python3 -c "
import fal_client
result = fal_client.run('fal-ai/kling-video/v3/pro/image-to-video', arguments={
    'prompt': 'the watch slowly rotates 360 degrees, camera orbits smoothly, dramatic lighting',
    'start_image_url': 'file://watch-keyframe.png',
    'duration': '10',
    'aspect_ratio': '16:9',
    'generate_audio': False,
})
print(result['video']['url'])
"

# 3. Download & extract frames
curl -o watch-rotation.mp4 "VIDEO_URL"
mkdir -p public/frames/watch/desktop public/frames/watch/mobile
ffmpeg -i watch-rotation.mp4 -vf "fps=30,scale=1920:1080" public/frames/watch/desktop/frame_%04d.png
ffmpeg -i watch-rotation.mp4 -vf "fps=15,scale=960:540" public/frames/watch/mobile/frame_%04d.png

# Convert to WebP
for dir in public/frames/watch/desktop public/frames/watch/mobile; do
  for f in "$dir"/*.png; do cwebp -q 80 "$f" -o "${f%.png}.webp" && rm "$f"; done
done

# 4. → Use scroll-animation skill for GSAP canvas playback
```

**Cost:** ~$0.07 (keyframe) + $1.12 (Kling 10s no audio) = **$1.19**

### Example 2: Hero Background Loop

```bash
# 1. Keyframe
nano-banana "abstract flowing liquid metal, iridescent, macro photography, dark background" -s 2K -a 16:9 -o hero-bg

# 2. Animate (seamless loop)
python3 -c "
import fal_client
result = fal_client.run('fal-ai/luma-dream-machine/ray-2/image-to-video', arguments={
    'prompt': 'gentle flowing motion, metallic liquid ripples, seamless loop',
    'image_url': 'file://hero-bg.png',
    'loop': True,
    'aspect_ratio': '16:9'
})
print(result['video']['url'])
"

# 3. Optimize for web autoplay
curl -o hero-loop-raw.mp4 "VIDEO_URL"
ffmpeg -i hero-loop-raw.mp4 -c:v libx264 -preset slow -crf 23 -an -movflags +faststart hero-loop.mp4
ffmpeg -i hero-loop-raw.mp4 -c:v libvpx-vp9 -crf 30 -b:v 2M -an hero-loop.webm
```

**Cost:** ~$0.07 + $0.50 = **$0.57**

### Example 3: Bulk Content Videos (Budget)

```bash
# 1. Generate 5 keyframes
for i in 1 2 3 4 5; do
  nano-banana "feature illustration $i, minimal, clean" -s 1K -a 16:9 -o "feature-$i"
done

# 2. Animate all with LTX (cheapest)
python3 << 'EOF'
import fal_client
prompts = [
    "gentle zoom in, subtle particle effects",
    "slow pan right revealing details",
    "elements float upward gently",
    "soft glow pulses outward",
    "camera slowly pulls back"
]
for i, prompt in enumerate(prompts, 1):
    result = fal_client.run("fal-ai/ltx-2/text-to-video/fast", arguments={
        "prompt": prompt,
        "image_url": f"file://feature-{i}.png",
        "duration": 5,
        "aspect_ratio": "16:9"
    })
    print(f"Feature {i}: {result['video']['url']}")
EOF
```

**Cost:** 5 × ($0.07 + $0.20) = **$1.35** for 5 videos

---

## Prompting Guide for Video Models

### Motion Prompts (DO)

- "slowly rotates 360 degrees clockwise"
- "camera dollies in from medium to close-up"
- "gentle floating up and down motion"
- "particles drift outward from center"
- "liquid flows from left to right"
- "zoom in on the detail, rack focus"

### Motion Prompts (DON'T)

- ~~"beautiful product"~~ (describes scene, not motion)
- ~~"4K cinematic masterpiece"~~ (quality tags don't help)
- ~~"the scene transforms into a different scene"~~ (too complex)
- ~~"person walks, talks, and dances"~~ (too many actions)

### Rules

1. **One motion per clip** — don't combine rotate + zoom + pan
2. **Describe the camera** — "camera orbits", "dolly in", "static wide"
3. **Describe speed** — "slowly", "gently", "dramatically fast"
4. **5s for simple motion**, 10s for complex reveals
5. **Match prompt to model** — Kling handles product/object motion best, Luma handles ambient/organic best

---

## Integration with Other Skills

| Skill | Role | When |
|-------|------|------|
| `nano-banana` | Generate keyframe images | Step 1 |
| `video-gen` | Process video → frame sequence | Step 3A |
| `scroll-animation` | GSAP ScrollTrigger playback | Step 4 (scroll) |
| `motion-design` | Page transitions + micro-interactions | Step 4 (ambient) |
| `web-assets` | Optimize final video/frames | Step 3 (all routes) |
| `landing-page` | Full page orchestration | When recipe needs video |
| `3d-scene` | Three.js alternative to AI video | When real 3D needed |

---

## nano-banana via fal.ai API (Alternative to CLI)

If the nano-banana CLI is not available, use the fal.ai endpoint directly:

```python
import fal_client

# Generate keyframe via fal.ai nano-banana (Gemini image gen)
result = fal_client.run("fal-ai/nano-banana", arguments={
    "prompt": "premium smart watch on dark background, studio rim lighting, centered, clean edges",
    "num_images": 1,
    "aspect_ratio": "16:9",   # 1:1, 16:9, 9:16, 4:3, 3:4, 3:2, 2:3, 4:5, 5:4, 21:9
    "output_format": "png",   # png, jpeg, webp
    "safety_tolerance": 4,    # 1 (strict) to 6 (permissive)
})
keyframe_url = result["images"][0]["url"]
# Cost: $0.039 per image

# Now chain directly into video generation (no upload needed — already a URL!)
video_result = fal_client.run("fal-ai/kling-video/v3/pro/image-to-video", arguments={
    "start_image_url": keyframe_url,  # direct URL from nano-banana
    "prompt": "watch slowly rotates 360 degrees, studio lighting",
    "duration": "5",
    "generate_audio": False,
})
video_url = video_result["video"]["url"]
```

**Key advantage:** nano-banana output is already a fal.ai URL, so it chains directly into any video model without upload. Total cost for keyframe + 5s Kling video: $0.039 + $0.56 = **$0.60**.

```bash
# curl version
KEYFRAME_URL=$(curl -s -X POST "https://fal.run/fal-ai/nano-banana" \
  -H "Authorization: Key $FAL_KEY" \
  -H "Content-Type: application/json" \
  -d '{"prompt": "premium watch on dark bg, studio lighting, centered", "aspect_ratio": "16:9"}' \
  | jq -r '.images[0].url')

curl -X POST "https://queue.fal.run/fal-ai/kling-video/v3/pro/image-to-video" \
  -H "Authorization: Key $FAL_KEY" \
  -H "Content-Type: application/json" \
  -d "{\"start_image_url\": \"$KEYFRAME_URL\", \"prompt\": \"watch rotates slowly\", \"duration\": \"5\", \"generate_audio\": false}"
```

---

## Workflow Summary

1. **Define goal** — what animation, where on page, what triggers it?
2. **Pick model** — use selection matrix above
3. **Cost estimate** — calculate and tell user BEFORE generating
4. **Generate keyframe** — nano-banana with animation-friendly composition
5. **Animate** — fal.ai with focused motion prompt
6. **Review** — watch output, iterate prompt if needed (budget 2-3 attempts)
7. **Process** — choose route: frame sequence / video / scrub / GIF
8. **Integrate** — embed with scroll-animation, video tag, or motion-design
9. **Fallback** — static keyframe for prefers-reduced-motion
