---
name: nano-banana
description: Generates AI images using the nano-banana CLI (Gemini 3.1 Flash default, Pro available). Handles multi-resolution (512-4K), aspect ratios, reference images for style transfer, green screen workflow for transparent assets, cost tracking, and exact dimension control. Use when asked to "generate an image", "create a sprite", "make an asset", "generate artwork", or any image generation task for UI mockups, game assets, videos, or marketing materials.
---

# nano-banana

AI image generation CLI. Default model: Gemini 3.1 Flash Image Preview (Nano Banana 2).
Installed at: `~/tools/nano-banana-2/`
Binary: `~/.bun/bin/nano-banana`

## Prerequisites

- **Bun** (installed at `~/.bun/bin/bun`)
- **FFmpeg + ImageMagick** for transparent mode: `brew install ffmpeg imagemagick`
- **Gemini API key** at `~/.nano-banana/.env` (get at https://aistudio.google.com/apikey)

## Quick Reference

- Command: `nano-banana "prompt" [options]`
- Default: 1K resolution, Flash model, current directory

## Core Options

| Option | Default | Description |
|--------|---------|-------------|
| `-o, --output` | `nano-gen-{timestamp}` | Output filename (no extension) |
| `-s, --size` | `1K` | Image size: `512`, `1K`, `2K`, or `4K` |
| `-a, --aspect` | model default | Aspect ratio: `1:1`, `16:9`, `9:16`, `4:3`, `3:4`, etc. |
| `-m, --model` | `flash` | Model: `flash`/`nb2`, `pro`/`nb-pro`, or any model ID |
| `-d, --dir` | current directory | Output directory |
| `-r, --ref` | - | Reference image (can use multiple times) |
| `-t, --transparent` | - | Green screen + FFmpeg background removal |
| `--api-key` | - | Gemini API key (overrides env/file) |
| `--costs` | - | Show cost summary |

## Models

| Alias | Model | Use When |
|-------|-------|----------|
| `flash`, `nb2` | Gemini 3.1 Flash | Default. Fast, cheap (~$0.067/1K image) |
| `pro`, `nb-pro` | Gemini 3 Pro | Highest quality (~$0.134/1K image) |

## Sizes & Costs

| Size | Resolution | Flash Cost | Pro Cost |
|------|-----------|------------|----------|
| `512` | ~512x512 | ~$0.045 | Flash only |
| `1K` | ~1024x1024 | ~$0.067 | ~$0.134 |
| `2K` | ~2048x2048 | ~$0.101 | ~$0.201 |
| `4K` | ~4096x4096 | ~$0.151 | ~$0.302 |

## Aspect Ratios

Supported: `1:1`, `16:9`, `9:16`, `4:3`, `3:4`, `3:2`, `2:3`, `4:5`, `5:4`, `21:9`

## Key Workflows

### Basic Generation
```bash
nano-banana "minimal dashboard UI with dark theme"
nano-banana "cinematic landscape" -s 2K -a 16:9
nano-banana "quick concept sketch" -s 512
```

### Pro Model (highest quality)
```bash
nano-banana "detailed portrait" --model pro -s 2K
```

### Reference Images (Style Transfer / Editing)
```bash
nano-banana "change the background to pure white" -r dark-ui.png -o light-ui
nano-banana "combine these two styles" -r style1.png -r style2.png -o combined
```

### Transparent Assets
```bash
nano-banana "robot mascot character" -t -o mascot
nano-banana "pixel art treasure chest" -t -o chest
```
The `-t` flag: green screen prompt -> FFmpeg colorkey + despill -> ImageMagick trim.

### Exact Dimensions
```bash
# First -r: style reference, Last -r: blank image in target dimensions
nano-banana "pixel art character, 256x256" -r style.png -r blank-256x256.png -o sprite
```

## Prompting Best Practices

1. **Natural language** — write sentences, not keyword lists
2. **Be specific** — include subject details, materials, lighting, mood
3. **Include context** — what is the image for (landing page, game asset, etc.)
4. **Edit conversationally** — refine existing generations with `-r` flag

### Anti-Patterns (avoid)
- Tag-soup keywords: `beautiful, HD, 4K, masterpiece`
- Vague subjects: `a nice landscape`
- Missing mood/lighting: just describing objects without atmosphere
- Over-prompting: contradictory instructions

### Good Prompt Examples
```bash
# UI mockups
nano-banana "clean SaaS dashboard with analytics charts, white background"
# Cinematic
nano-banana "cyberpunk cityscape at sunset, neon reflections on wet streets" -a 16:9 -s 2K
# Product
nano-banana "premium software product hero image, floating UI elements" --model pro
# Game assets
nano-banana "pixel art treasure chest, wooden with gold trim" -t -o chest
# Dark mode UI
nano-banana "Premium SaaS chat interface, dark mode, minimal, Linear-style aesthetic"
```

## Use Cases

- Landing page assets (product mockups, hero images)
- Image editing (transform with text prompts + `-r`)
- Style transfer (combine multiple references)
- Marketing materials (feature illustrations)
- UI iterations (quick design variations)
- Transparent assets (icons, logos, mascots with `-t`)
- Game assets (sprites, characters, tilesets)
- Video production (visual elements for compositions)
- 3D website animations (generate keyframes for scroll-driven animations)

## Cost Tracking

Every generation logged to `~/.nano-banana/costs.json`.
```bash
nano-banana --costs
```

## API Key Resolution Order
1. `--api-key` flag
2. `GEMINI_API_KEY` env var
3. `.env` in current directory
4. `.env` in repo root
5. `~/.nano-banana/.env`
