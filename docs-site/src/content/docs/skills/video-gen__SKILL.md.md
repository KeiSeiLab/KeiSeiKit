---
title: SKILL
path: video-gen/SKILL.md
dna_hash: sha256:e31e06c3cc05c68f
language: markdown
size_loc: 367
generated: by-keidocs
---

# video-gen/SKILL.md

## Public API

- `Video-Gen Skill — Frame Sequence Pipeline` — ---
- `Pipeline Overview` — ```
- `1. Frame Extraction [E1]` — ### Basic Extraction
- `Extract all frames at source FPS` — ffmpeg -i source.mp4 -qscale:v 2 frames/frame_%04d.png
- `Extract at specific FPS (30fps → 150 frames for 5s video)` — ffmpeg -i source.mp4 -vf "fps=30" frames/frame_%04d.png
- `Extract with resolution scaling` — ffmpeg -i source.mp4 -vf "fps=30,scale=1920:1080" frames/frame_%04d.png
- `Extract specific time range (2s to 7s)` — ffmpeg -i source.mp4 -ss 2 -t 5 -vf "fps=30" frames/frame_%04d.png
- `2. Format Conversion [E1]` — ### PNG to WebP (Recommended)
- `Single file` — cwebp -q 80 frame_0001.png -o frame_0001.webp
- `Batch convert all PNGs` — for f in frames/*.png; do
- `Parallel batch (faster)` — find frames/ -name "*.png" | xargs -P 8 -I {} sh -c '
- `Requires avifenc (brew install libavif)` — avifenc --min 20 --max 30 -s 6 frame_0001.png frame_0001.avif
- `Batch` — for f in frames/*.png; do
- `3. Size Budgets [E2]` — ### Per-Frame Targets
- `4. Responsive Frame Sets [E2]` — ### Directory Structure
- `Desktop: 1920x1080, 30fps` — mkdir -p frames/desktop
- `Tablet: 1280x720, 24fps` — mkdir -p frames/tablet
- `Mobile: 960x540, 15fps` — mkdir -p frames/mobile
- `Convert all to WebP` — for dir in frames/desktop frames/tablet frames/mobile; do
- `5. Sprite Sheet (Alternative) [E2]` — For fewer frames (<60), a single sprite sheet can be faster than individual files:
- `Create sprite sheet with ImageMagick` — montage frames/frame_*.webp -tile 10x6 -geometry 480x270+0+0 spritesheet.webp
- `60 frames, 10 columns x 6 rows, each 480x270` — ```
- `6. Video Scrub (Alternative to Frame Sequence) [E2]` — Apple's modern approach: single compressed video + scroll-driven playback.
- `Encode for web scrub: low bitrate, many keyframes` — ffmpeg -i source.mp4 \
- `7. Preloading Strategy [E1]` — ### Priority Loading
- `Workflow` — 1. **Source video** — get MP4/MOV at highest quality available

## Related

- parent: `video-gen`

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
