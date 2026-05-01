---
name: video-gen
description: Use when generating video from frame sequences — FFmpeg extraction, WebP/AVIF conversion, sprite sheets, scroll-synced playback, video encoding/transcoding. Covers the full pipeline from video source to web-ready frame sequence or optimized video.
arguments:
  - name: source
    description: "Source: video file path, image sequence directory, or AI-generated frames"
    required: true
  - name: target
    description: "Target: frame-sequence, sprite-sheet, optimized-video, gif (default: frame-sequence)"
    required: false
---

# Video-Gen Skill — Frame Sequence Pipeline

## Pipeline Overview

```
Source Video (MP4/MOV/ProRes)
  │
  ├─→ [Frame Extraction] FFmpeg → PNG sequence
  │     │
  │     ├─→ [Optimize] cwebp → WebP sequence (primary)
  │     ├─→ [Optimize] avifenc → AVIF sequence (smaller, slower encode)
  │     └─→ [Sprite Sheet] ImageMagick montage → single image
  │
  ├─→ [Video Scrub] FFmpeg re-encode → optimized MP4 for scroll scrub
  │
  └─→ [Web Playback] Canvas + ScrollTrigger / video.currentTime
```

---

## 1. Frame Extraction [E1]

### Basic Extraction

```bash
# Extract all frames at source FPS
ffmpeg -i source.mp4 -qscale:v 2 frames/frame_%04d.png

# Extract at specific FPS (30fps → 150 frames for 5s video)
ffmpeg -i source.mp4 -vf "fps=30" frames/frame_%04d.png

# Extract with resolution scaling
ffmpeg -i source.mp4 -vf "fps=30,scale=1920:1080" frames/frame_%04d.png

# Extract specific time range (2s to 7s)
ffmpeg -i source.mp4 -ss 2 -t 5 -vf "fps=30" frames/frame_%04d.png
```

### Frame Count Guidelines

| Duration | Desktop (30fps) | Mobile (15fps) | Notes |
|----------|-----------------|----------------|-------|
| 3 seconds | 90 frames | 45 frames | Short reveal |
| 5 seconds | 150 frames | 75 frames | Product showcase |
| 10 seconds | 300 frames | 150 frames | Full story section |
| 15 seconds | 450 frames | 225 frames | Max recommended |

**Rule:** More frames = smoother but heavier. 120-180 is the sweet spot for most scroll animations.

---

## 2. Format Conversion [E1]

### PNG to WebP (Recommended)

```bash
# Single file
cwebp -q 80 frame_0001.png -o frame_0001.webp

# Batch convert all PNGs
for f in frames/*.png; do
  cwebp -q 80 "$f" -o "${f%.png}.webp"
done

# Parallel batch (faster)
find frames/ -name "*.png" | xargs -P 8 -I {} sh -c '
  cwebp -q 80 "{}" -o "$(echo {} | sed s/.png/.webp/)"
'
```

### PNG to AVIF (Smaller, Slower Encode)

```bash
# Requires avifenc (brew install libavif)
avifenc --min 20 --max 30 -s 6 frame_0001.png frame_0001.avif

# Batch
for f in frames/*.png; do
  avifenc --min 20 --max 30 -s 6 "$f" "${f%.png}.avif"
done
```

### Format Comparison

| Format | Quality at q80 | Size vs PNG | Encode Speed | Browser Support |
|--------|----------------|-------------|--------------|-----------------|
| WebP | Excellent | -85-90% | Fast | All modern [E1] |
| AVIF | Excellent | -90-95% | Slow (10x) | Chrome, Firefox, Safari 16+ [E1] |
| JPEG | Good | -80-85% | Fastest | Universal [E1] |
| PNG | Lossless | Baseline | Fast | Universal [E1] |

**Decision:** WebP for production (best balance). AVIF if encode time is not an issue and you need minimum size.

---

## 3. Size Budgets [E2]

### Per-Frame Targets

| Resolution | WebP q80 | AVIF q30 | Target per frame |
|------------|----------|----------|------------------|
| 960x540 (mobile) | 12-18 KB | 8-12 KB | <15 KB |
| 1280x720 (tablet) | 18-28 KB | 12-20 KB | <25 KB |
| 1920x1080 (desktop) | 25-45 KB | 18-30 KB | <35 KB |

### Total Budget

| Frames | Desktop total | Mobile total | Acceptable? |
|--------|---------------|--------------|-------------|
| 60 | ~1.5 MB | ~0.7 MB | Great |
| 120 | ~3.0 MB | ~1.4 MB | Good |
| 180 | ~4.5 MB | ~2.1 MB | Acceptable |
| 300 | ~7.5 MB | ~3.5 MB | Heavy, needs lazy load |

**Hard limit:** <5MB total for initial load. Lazy load anything beyond.

---

## 4. Responsive Frame Sets [E2]

### Directory Structure

```
/public/frames/
  /desktop/    # 1920x1080, 150 frames
  /tablet/     # 1280x720, 120 frames
  /mobile/     # 960x540, 75 frames
```

### FFmpeg Multi-Resolution Script

```bash
#!/bin/bash
SOURCE="source.mp4"

# Desktop: 1920x1080, 30fps
mkdir -p frames/desktop
ffmpeg -i "$SOURCE" -vf "fps=30,scale=1920:1080" frames/desktop/frame_%04d.png

# Tablet: 1280x720, 24fps
mkdir -p frames/tablet
ffmpeg -i "$SOURCE" -vf "fps=24,scale=1280:720" frames/tablet/frame_%04d.png

# Mobile: 960x540, 15fps
mkdir -p frames/mobile
ffmpeg -i "$SOURCE" -vf "fps=15,scale=960:540" frames/mobile/frame_%04d.png

# Convert all to WebP
for dir in frames/desktop frames/tablet frames/mobile; do
  for f in "$dir"/*.png; do
    cwebp -q 80 "$f" -o "${f%.png}.webp"
    rm "$f"  # remove PNG after conversion
  done
done
```

### Responsive Loading (JS)

```js
function getBreakpoint() {
  const w = window.innerWidth;
  if (w >= 1280) return { dir: "desktop", count: 150 };
  if (w >= 768)  return { dir: "tablet",  count: 120 };
  return { dir: "mobile", count: 75 };
}

const { dir, count } = getBreakpoint();
const basePath = `/frames/${dir}/frame_`;
```

---

## 5. Sprite Sheet (Alternative) [E2]

For fewer frames (<60), a single sprite sheet can be faster than individual files:

```bash
# Create sprite sheet with ImageMagick
montage frames/frame_*.webp -tile 10x6 -geometry 480x270+0+0 spritesheet.webp
# 60 frames, 10 columns x 6 rows, each 480x270
```

### CSS Sprite Animation

```css
.sprite-player {
  width: 480px;
  height: 270px;
  background: url("spritesheet.webp");
  background-size: 4800px 1620px; /* 10 cols x 6 rows */
}
```

### JS Sprite + Scroll

```js
const sprite = document.querySelector(".sprite-player");
const cols = 10, rows = 6, total = 60;
const frameW = 480, frameH = 270;

gsap.to({ frame: 0 }, {
  frame: total - 1,
  snap: "frame",
  ease: "none",
  scrollTrigger: {
    trigger: "#sprite-section",
    start: "top top",
    end: "+=2000",
    pin: true,
    scrub: 0.5,
  },
  onUpdate: function() {
    const i = Math.round(this.targets()[0].frame);
    const col = i % cols;
    const row = Math.floor(i / cols);
    sprite.style.backgroundPosition = `-${col * frameW}px -${row * frameH}px`;
  }
});
```

---

## 6. Video Scrub (Alternative to Frame Sequence) [E2]

Apple's modern approach: single compressed video + scroll-driven playback.

### Optimize Video for Scrub

```bash
# Encode for web scrub: low bitrate, many keyframes
ffmpeg -i source.mp4 \
  -c:v libx264 \
  -preset slow \
  -crf 23 \
  -g 1 \          # keyframe every frame (critical for scrub)
  -an \           # no audio
  -movflags +faststart \
  -vf "scale=1920:1080" \
  output-scrub.mp4
```

**Key:** `-g 1` makes every frame a keyframe, enabling instant seeking. File will be larger than normal video but smaller than frame sequence.

### Playback

```js
const video = document.getElementById("scrub-video");

// Ensure video is loaded
video.preload = "auto";

gsap.to(video, {
  currentTime: video.duration || 5, // fallback if metadata not loaded
  ease: "none",
  scrollTrigger: {
    trigger: "#video-section",
    start: "top top",
    end: "+=4000",
    pin: true,
    scrub: true,
  }
});

// Alternative: manual scroll control
video.addEventListener("loadedmetadata", () => {
  const section = document.getElementById("video-section");
  window.addEventListener("scroll", () => {
    const rect = section.getBoundingClientRect();
    const progress = Math.max(0, Math.min(1,
      -rect.top / (rect.height - window.innerHeight)
    ));
    video.currentTime = progress * video.duration;
  });
});
```

### Frame Sequence vs Video Scrub

| Factor | Frame Sequence | Video Scrub |
|--------|---------------|-------------|
| Smoothness | Best (instant) | Good (may drop on mobile) |
| File size | 2-5 MB (150 frames) | 1-3 MB (one file) |
| HTTP requests | 150 requests | 1 request |
| Memory usage | High (all frames in RAM) | Low (decoded on demand) |
| Mobile perf | Good | Variable |
| Complexity | More code | Simpler |

**Decision:** Frame sequence for hero sections where smoothness is critical. Video scrub for secondary content or when bandwidth is limited.

---

## 7. Preloading Strategy [E1]

### Priority Loading

```js
async function loadFrames(basePath, count) {
  const frames = [];

  // Phase 1: Load first 10 frames immediately (show something fast)
  const priority = Array.from({ length: 10 }, (_, i) =>
    loadImage(`${basePath}${String(i + 1).padStart(4, "0")}.webp`)
  );
  const first10 = await Promise.all(priority);
  frames.push(...first10);

  // Phase 2: Load remaining frames in background
  for (let i = 10; i < count; i++) {
    const img = new Image();
    img.src = `${basePath}${String(i + 1).padStart(4, "0")}.webp`;
    frames.push(img);
  }

  return frames;
}

function loadImage(src) {
  return new Promise((resolve, reject) => {
    const img = new Image();
    img.onload = () => resolve(img);
    img.onerror = reject;
    img.src = src;
  });
}
```

### IntersectionObserver Trigger

```js
// Only start loading when section is near viewport
const observer = new IntersectionObserver(
  ([entry]) => {
    if (entry.isIntersecting) {
      loadFrames("/frames/desktop/frame_", 150);
      observer.disconnect();
    }
  },
  { rootMargin: "200px" } // start 200px before visible
);
observer.observe(document.getElementById("sequence-section"));
```

---

## Workflow

1. **Source video** — get MP4/MOV at highest quality available
2. **Plan frame count** — duration * fps, aim for 120-180 sweet spot
3. **Extract frames** — FFmpeg with target resolution and fps
4. **Convert to WebP** — cwebp q80, check total size budget
5. **Create responsive sets** — desktop/tablet/mobile with different counts
6. **Implement playback** — Canvas + GSAP ScrollTrigger (or video scrub)
7. **Add preloading** — priority first 10, lazy rest, IntersectionObserver
8. **Test on mobile** — check memory usage, reduce frames if needed
9. **Add fallback** — static image for prefers-reduced-motion
