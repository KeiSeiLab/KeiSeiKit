---
name: site-teardown
description: "Deconstruct any live website into reusable recipe — extract HTML, CSS, JS, design tokens, animations. Use when user says: teardown, deconstruct, clone site, reverse engineer, how is this site built."
arguments:
  - name: url
    description: URL of the website to deconstruct
    required: true
  - name: depth
    description: "quick = tokens + screenshots only, full = complete teardown (default: full)"
    required: false
---

# Site Teardown — Deconstruct Any Website into a Reusable Recipe

Extracts design tokens, layout structure, animation techniques, and library stack from a live website.
Output: structured recipe that can be fed into `/frontend-design`, `/landing-page`, `/design-system`.

## Phase 1 — Navigate & Screenshot

```
1. browser_navigate → {url}
2. browser_resize → width: 1280, height: 900
3. browser_take_screenshot → fullPage: true, filename: "teardown-desktop.png"
4. browser_resize → width: 375, height: 812
5. browser_take_screenshot → fullPage: true, filename: "teardown-mobile.png"
6. browser_resize → width: 1280, height: 900  (restore)
```

Save screenshots to `teardown/{domain}/` in the project directory (relative to `$PWD`).

## Phase 2 — Extract HTML Structure

Run `browser_evaluate` with:

```javascript
() => {
  const sections = Array.from(document.querySelectorAll('section, [class*="section"], main > div'));
  const nav = document.querySelector('nav, header');
  const footer = document.querySelector('footer');
  const headings = Array.from(document.querySelectorAll('h1, h2, h3')).map(h => ({
    tag: h.tagName, text: h.textContent.trim().slice(0, 80)
  }));
  return {
    title: document.title,
    sectionCount: sections.length,
    hasNav: !!nav,
    navType: nav?.classList?.toString() || 'unknown',
    hasFooter: !!footer,
    headings,
    bodyClasses: document.body.classList.toString(),
    htmlLang: document.documentElement.lang
  };
}
```

Also extract full HTML for deep analysis:
```javascript
() => document.documentElement.outerHTML
```
Save to `teardown/{domain}/index.html`.

## Phase 3 — Extract Design Tokens

Run `browser_evaluate` to extract computed styles from key elements:

```javascript
() => {
  const get = (sel) => {
    const el = document.querySelector(sel);
    return el ? getComputedStyle(el) : null;
  };
  const body = get('body');
  const h1 = get('h1');
  const btn = get('a[class*="btn"], button[class*="btn"], .cta, a[class*="cta"]');
  const card = get('[class*="card"], [class*="Card"]');
  const props = {};
  const root = getComputedStyle(document.documentElement);
  for (const name of ['--primary', '--secondary', '--accent', '--background', '--foreground',
    '--radius', '--font-sans', '--font-mono', '--brand']) {
    const val = root.getPropertyValue(name).trim();
    if (val) props[name] = val;
  }
  return {
    colors: {
      background: body?.backgroundColor,
      text: body?.color,
      heading: h1?.color,
      button: btn ? { bg: btn.backgroundColor, text: btn.color, radius: btn.borderRadius } : null,
      card: card ? { bg: card.backgroundColor, border: card.borderColor, radius: card.borderRadius, shadow: card.boxShadow } : null
    },
    typography: {
      bodyFont: body?.fontFamily,
      bodySize: body?.fontSize,
      h1Font: h1?.fontFamily,
      h1Size: h1?.fontSize,
      h1Weight: h1?.fontWeight,
      lineHeight: body?.lineHeight
    },
    spacing: {
      bodyPadding: body?.padding,
      sectionPadding: get('section')?.padding
    },
    customProperties: props
  };
}
```

**Output:** Save as `teardown/{domain}/tokens.json`.

## Phase 4 — Fetch CSS & JS Sources

### 4a. Collect resource URLs

```javascript
() => {
  const css = Array.from(document.querySelectorAll('link[rel="stylesheet"]')).map(l => l.href);
  const js = Array.from(document.querySelectorAll('script[src]')).map(s => s.src);
  const inlineStyles = document.querySelectorAll('style').length;
  return { css, js, inlineStyleBlocks: inlineStyles };
}
```

### 4b. Fetch each CSS file via WebFetch

For each CSS URL: `WebFetch` with prompt:
> "Extract ALL design-relevant CSS from this stylesheet: custom properties (--vars), @keyframes, @font-face, color values, gradient definitions, backdrop-filter, box-shadow patterns, border-radius values, transition/animation properties. Return as structured list."

### 4c. Detect JS libraries

```javascript
() => ({
  gsap: typeof gsap !== 'undefined',
  ScrollTrigger: typeof ScrollTrigger !== 'undefined',
  lenis: !!document.querySelector('[data-lenis-prevent]') || typeof Lenis !== 'undefined',
  framerMotion: !!document.querySelector('[data-framer-component-type]'),
  three: typeof THREE !== 'undefined',
  curtains: typeof Curtains !== 'undefined',
  particles: typeof tsParticles !== 'undefined',
  aos: typeof AOS !== 'undefined',
  locomotive: !!document.querySelector('[data-scroll-container]'),
  swiper: typeof Swiper !== 'undefined',
  tailwind: !!document.querySelector('[class*="bg-"], [class*="text-"], [class*="flex"]'),
  react: typeof __NEXT_DATA__ !== 'undefined' || !!document.getElementById('__next'),
  astro: !!document.querySelector('[data-astro-source-file]'),
  vue: !!document.getElementById('__nuxt') || !!document.querySelector('[data-v-]')
})
```

### 4d. Network analysis (supplementary)

`browser_network_requests` with `filter: "\\.css$|\\.js$"`, `static: false` — cross-reference with DOM-extracted URLs.

## Phase 5 — Animation Catalog

```javascript
() => {
  const anims = [];
  const allEls = document.querySelectorAll('*');
  const seen = new Set();
  allEls.forEach(el => {
    const s = getComputedStyle(el);
    if (s.animationName && s.animationName !== 'none' && !seen.has(s.animationName)) {
      seen.add(s.animationName);
      anims.push({ type: 'css-animation', name: s.animationName, duration: s.animationDuration });
    }
    if (s.transition && s.transition !== 'all 0s ease 0s' && s.transition !== 'none') {
      const key = s.transition.slice(0, 60);
      if (!seen.has(key)) { seen.add(key); anims.push({ type: 'transition', value: s.transition.slice(0, 120) }); }
    }
  });
  const canvases = document.querySelectorAll('canvas').length;
  const videos = document.querySelectorAll('video').length;
  const svgAnims = document.querySelectorAll('animate, animateTransform').length;
  return { animations: anims, canvasCount: canvases, videoCount: videos, svgAnimations: svgAnims };
}
```

**Output:** Save analysis as `teardown/{domain}/animations.md`.

If `depth=quick` → STOP here with tokens + screenshots only.

## Phase 6 — Compile Recipe

Assemble `teardown/{domain}/recipe.md`:

```markdown
# Site Teardown: {domain}
Date: {date}

## Layout Structure
{section map from Phase 2}

## Design Tokens
{from Phase 3 — colors, typography, spacing}

## Tech Stack
- Framework: {React/Next/Astro/Vue from Phase 4c}
- CSS: {Tailwind/custom/styled-components}
- Animation: {GSAP/Framer Motion/CSS/AOS from Phase 4c}
- Scroll: {Lenis/Locomotive/native from Phase 4c}
- 3D/WebGL: {Three.js/curtains.js/none from Phase 4c}

## Animation Techniques
{catalog from Phase 5}

## Reproduction Steps
1. Set up {framework} project with {css approach}
2. Apply design tokens: {token summary}
3. Implement layout: {section sequence}
4. Add animations: {technique list with skill references}
5. Optimize: /web-assets → /a11y-audit → /perf-audit

## Recommended Skills
- /frontend-design archetype={suggested}
- /scroll-animation technique={if GSAP detected}
- /web-effects effect={if WebGL detected}
- /motion-design {if Framer Motion detected}
```

## Chaining

| Direction | Skill | How |
|-----------|-------|-----|
| FROM | `/design-inspiration` | User picks best reference → teardown |
| FROM | `/competitor-analysis` | Deep-dive competitor's site |
| TO | `/frontend-design` | Feed tokens → suggest archetype |
| TO | `/landing-page` | Use recipe as template |
| TO | `/design-system` | Generate token system from extracted tokens |
| TO | `/scroll-animation` | Reproduce detected scroll effects |
| TO | `/web-effects` | Reproduce detected WebGL/particle effects |
