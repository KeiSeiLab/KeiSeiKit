---
title: SKILL
path: nano-banana/SKILL.md
dna_hash: sha256:7f5a780fa75937fa
language: markdown
size_loc: 147
generated: by-keidocs
---

# nano-banana/SKILL.md

## Public API

- `nano-banana` — ---
- `When to use` — - Generating AI images for UI mockups, game sprites, marketing assets, or video content via the `nano-banana` CLI.
- `Prerequisites` — - **Bun** (installed at `~/.bun/bin/bun`)
- `Quick Reference` — - Command: `nano-banana "prompt" [options]`
- `Core Options` — | Option | Default | Description |
- `Models` — | Alias | Model | Use When |
- `Sizes & Costs` — | Size | Resolution | Flash Cost | Pro Cost |
- `Aspect Ratios` — Supported: `1:1`, `16:9`, `9:16`, `4:3`, `3:4`, `3:2`, `2:3`, `4:5`, `5:4`, `21:9`
- `Key Workflows` — ### Basic Generation
- `First -r: style reference, Last -r: blank image in target dimensions` — nano-banana "pixel art character, 256x256" -r style.png -r blank-256x256.png -o sprite
- `Prompting Best Practices` — 1. **Natural language** — write sentences, not keyword lists
- `UI mockups` — nano-banana "clean SaaS dashboard with analytics charts, white background"
- `Cinematic` — nano-banana "cyberpunk cityscape at sunset, neon reflections on wet streets" -a 16:9 -s 2K
- `Product` — nano-banana "premium software product hero image, floating UI elements" --model pro
- `Game assets` — nano-banana "pixel art treasure chest, wooden with gold trim" -t -o chest
- `Dark mode UI` — nano-banana "Premium SaaS chat interface, dark mode, minimal, Linear-style aesthetic"
- `Use Cases` — - Landing page assets (product mockups, hero images)
- `Cost Tracking` — Every generation logged to `~/.nano-banana/costs.json`.
- `API Key Resolution Order` — 1. `--api-key` flag

## Related

- parent: `nano-banana`

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
