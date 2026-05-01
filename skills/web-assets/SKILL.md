---
name: web-assets
description: Use when optimizing images, fonts, and video for web — AVIF pipeline, responsive srcset, font subsetting, video codec selection, Sharp.js processing. Triggers on "optimize images", "web assets", "image pipeline", "font optimization".
arguments:
  - name: command
    description: "Command: optimize, picture, fonts, video, audit, pipeline"
    required: false
  - name: target
    description: Directory or file path to process
    required: false
---

# Image & Asset Optimization Pipeline

Optimize images, fonts, and video for premium web performance.

## Decision Matrix

| Asset | Tool | Format | Quality |
|-------|------|--------|---------|
| Photos | Sharp.js | AVIF primary, WebP fallback | avif:50, webp:75, jpg:80 |
| Icons | SVG sprite | `<symbol>` + `<use>` | N/A |
| Fonts | glyphhanger | WOFF2 only, subset | variable font preferred |
| Video | FFmpeg | AV1 > H.265 > H.264 | CRF 28-32 |
| AI-generated images | External generator (e.g. fal.ai) + Sharp | Process through Sharp | per above |

## Image Pipeline (Sharp.js)

```bash
npm ls sharp 2>/dev/null || npm install sharp
```

Breakpoints: 400, 640, 768, 1024, 1280, 1920px. Max 2560px for Retina.

```javascript
const sharp = require('sharp');
const WIDTHS = [400, 640, 768, 1024, 1280, 1920];
const FORMATS = ['avif', 'webp', 'jpg'];
const QUALITY = { avif: 50, webp: 75, jpg: 80 };

async function processImage(inputPath, outputDir) {
  const name = path.parse(inputPath).name;
  fs.mkdirSync(outputDir, { recursive: true });
  for (const width of WIDTHS) {
    for (const format of FORMATS) {
      await sharp(inputPath)
        .resize(width, null, { withoutEnlargement: true })
        .toFormat(format, { quality: QUALITY[format] })
        .toFile(path.join(outputDir, `${name}-${width}.${format}`));
    }
  }
}
```

### Picture Element

```html
<picture>
  <source type="image/avif"
    srcset="img/hero-400.avif 400w, img/hero-768.avif 768w, img/hero-1280.avif 1280w, img/hero-1920.avif 1920w"
    sizes="(max-width: 640px) 100vw, (max-width: 1024px) 80vw, 60vw" />
  <source type="image/webp"
    srcset="img/hero-400.webp 400w, img/hero-768.webp 768w, img/hero-1280.webp 1280w, img/hero-1920.webp 1920w"
    sizes="(max-width: 640px) 100vw, (max-width: 1024px) 80vw, 60vw" />
  <img src="img/hero-1280.jpg" alt="Descriptive alt text"
    width="1280" height="720" loading="lazy" decoding="async" />
</picture>
```

Hero/LCP image: `fetchpriority="high"`, NO `loading="lazy"`.

## Font Optimization

- Variable fonts = industry standard. WOFF2 only (97%+ support)
- Subset with glyphhanger: `glyphhanger --US_ASCII --subset=font.woff2 --formats=woff2` (60%+ reduction)
- `font-display: swap` + preload critical: `<link rel="preload" href="/fonts/heading.woff2" as="font" type="font/woff2" crossorigin />`

## SVG Sprites

```html
<svg xmlns="http://www.w3.org/2000/svg" style="display:none">
  <symbol id="icon-arrow" viewBox="0 0 24 24"><path d="M5 12h14M12 5l7 7-7 7"/></symbol>
</svg>
<svg class="icon" aria-hidden="true"><use href="/sprites.svg#icon-arrow"/></svg>
```

## Video

AV1 primary (30-50% better than H.264), H.265 fallback, H.264 universal. Always set poster, width/height.

```html
<video autoplay muted loop playsinline poster="hero-poster.avif" preload="none" width="1920" height="1080">
  <source src="hero.av1.mp4" type='video/mp4; codecs="av01.0.08M.08"' />
  <source src="hero.h265.mp4" type='video/mp4; codecs="hvc1"' />
  <source src="hero.h264.mp4" type="video/mp4" />
</video>
```

Lazy load via IntersectionObserver (no native `loading="lazy"` for `<video>`).

## Audit Checklist

- [ ] All images: AVIF + WebP + fallback, responsive srcset
- [ ] All `<img>`: explicit width/height (prevents CLS)
- [ ] Hero/LCP: `fetchpriority="high"`, no lazy loading
- [ ] Below-fold: `loading="lazy" decoding="async"`
- [ ] Fonts: WOFF2, subsetted, font-display: swap, critical preloaded
- [ ] Icons: SVG sprites (not individual files or icon fonts)
- [ ] Video: AV1 > H.265 > H.264 cascade, poster image
- [ ] No images >500KB, total page <1.5MB ideal
