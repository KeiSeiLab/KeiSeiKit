---
name: scroll-animation
description: Use when building scroll-driven animations — GSAP ScrollTrigger, CSS scroll-timeline, frame sequences, parallax, pin/scrub effects. Covers Apple-style scroll playback, progress-linked animations, and smooth scroll integration.
arguments:
  - name: technique
    description: "Technique: gsap, css-native, frame-sequence, parallax, hybrid (auto-detect if omitted)"
    required: false
  - name: framework
    description: "Framework: react, next, astro, vue, svelte, vanilla (auto-detect if omitted)"
    required: false
---

# Scroll Animation Skill

## Decision Matrix — Pick Technique

| Need | Technique | Why |
|------|-----------|-----|
| Pin + scrub + snap | GSAP ScrollTrigger | Most mature, free since Webflow acquisition |
| Simple fade/slide on scroll | CSS `animation-timeline` | Zero JS, native performance |
| Apple-style frame playback | Canvas frame sequence | Smoothest result for product reveals |
| Parallax layers | CSS or GSAP | CSS for simple, GSAP for complex |
| Smooth scroll feel | Lenis + GSAP | Industry standard combo |

---

## 1. GSAP ScrollTrigger

**License:** 100% FREE including all plugins
**Install:** `npm i gsap`

### Core API

```js
import gsap from "gsap";
import { ScrollTrigger } from "gsap/ScrollTrigger";
gsap.registerPlugin(ScrollTrigger);

// Pin + Scrub
gsap.to(".hero-content", {
  y: -100,
  opacity: 0,
  scrollTrigger: {
    trigger: ".hero",
    start: "top top",
    end: "bottom top",
    pin: true,
    scrub: 1,
    snap: { snapTo: 1 / 4, duration: 0.3, ease: "power1.inOut" }
  }
});

// Batch — stagger elements entering viewport
ScrollTrigger.batch(".card", {
  onEnter: (elements) => {
    gsap.to(elements, { opacity: 1, y: 0, stagger: 0.1 });
  },
  start: "top 85%"
});
```

### React Integration (useGSAP hook)

```jsx
import { useGSAP } from "@gsap/react";
import gsap from "gsap";
import { ScrollTrigger } from "gsap/ScrollTrigger";

gsap.registerPlugin(ScrollTrigger);

function Section({ children }) {
  const container = useRef(null);

  useGSAP(() => {
    gsap.from(".animate-in", {
      y: 50,
      opacity: 0,
      stagger: 0.2,
      scrollTrigger: { trigger: container.current, start: "top 80%" }
    });
  }, { scope: container });

  return <section ref={container}>{children}</section>;
}
```

**Key:** `useGSAP` = drop-in for `useEffect`, auto-cleanup via `gsap.context()`.

### Astro Integration

```astro
<section id="scroll-section">
  <div class="pin-target">Content</div>
</section>

<script>
  import gsap from "gsap";
  import { ScrollTrigger } from "gsap/ScrollTrigger";
  gsap.registerPlugin(ScrollTrigger);

  gsap.to(".pin-target", {
    x: 500,
    scrollTrigger: { trigger: "#scroll-section", pin: true, scrub: true }
  });
</script>
```

### Performance Best Practices

- Use `will-change: transform` on pinned elements
- Prefer `transform` and `opacity` — GPU-composited, no layout recalc
- `scrub: 1` (or higher) smooths jank vs `scrub: true` (instant)
- `invalidateOnRefresh: true` for responsive layouts
- Call `ScrollTrigger.refresh()` after dynamic content loads
- Avoid animating `width`, `height`, `top`, `left` — triggers reflow

---

## 2. CSS Scroll-Driven Animations (Native)

### Scroll Progress Timeline

```css
@keyframes fade-in {
  from { opacity: 0; transform: translateY(30px); }
  to   { opacity: 1; transform: translateY(0); }
}

.animate-on-scroll {
  animation: fade-in linear both;
  animation-timeline: scroll();
  animation-range: entry 0% entry 100%;
}
```

### View Progress Timeline

```css
.reveal {
  animation: fade-in linear both;
  animation-timeline: view();
  animation-range: entry 25% cover 50%;
}
```

### Progressive Enhancement

```css
@supports (animation-timeline: scroll()) {
  .animate { animation-timeline: scroll(); }
}
/* Fallback: use IntersectionObserver + classList toggle */
```

### What CSS Can Replace from GSAP

| Feature | CSS Native | Still Need GSAP |
|---------|-----------|-----------------|
| Fade/slide on scroll | Yes | No |
| Progress-linked animation | Yes | No |
| View-enter/exit triggers | Yes | No |
| Pin element | No | Yes |
| Snap to sections | No (scroll-snap is separate) | Yes (integrated) |
| Batch stagger | No | Yes |
| Timeline sequencing | Limited | Yes |
| Complex easing curves | Limited | Yes |
| JS callbacks on progress | No | Yes |

**Rule of thumb:** CSS for simple reveal animations. GSAP for anything with pin, snap, stagger, or JS logic.

---

## 3. Lenis Smooth Scroll

**Install:** `npm i lenis`
**Bundle:** ~14KB min+gzip (no dependencies)

```js
import Lenis from "lenis";

const lenis = new Lenis({
  duration: 1.2,
  easing: (t) => Math.min(1, 1.001 - Math.pow(2, -10 * t)),
  orientation: "vertical",
  smoothWheel: true,
});

// Connect to GSAP ticker for sync
gsap.ticker.add((time) => { lenis.raf(time * 1000); });
gsap.ticker.lagSmoothing(0);

// Connect to ScrollTrigger
lenis.on("scroll", ScrollTrigger.update);
```

**When to use:** Agency-style smooth scroll feel. Pairs with GSAP ScrollTrigger.
**When NOT to use:** Content-heavy sites, accessibility-first projects.

---

## 4. Frame Sequence on Scroll (Apple-Style)

### Pipeline

```
Video (MP4/MOV)
  → FFmpeg frame extraction (PNG)
  → Convert to WebP (90% size reduction vs PNG)
  → Canvas playback synced to scroll
```

### FFmpeg Extraction

```bash
ffmpeg -i source.mp4 -vf "fps=30,scale=1280:720" frames/frame_%04d.png
for f in frames/*.png; do cwebp -q 80 "$f" -o "${f%.png}.webp"; done
```

### Optimal Parameters

| Parameter | Desktop | Mobile |
|-----------|---------|--------|
| Frame count | 120-180 | 60-90 |
| Resolution | 1920x1080 | 960x540 |
| Format | WebP q80 | WebP q75 |
| Total budget | 2-4 MB | 1-2 MB |

### Canvas Implementation

```js
const canvas = document.getElementById("sequence-canvas");
const ctx = canvas.getContext("2d");
const frameCount = 150;
const frames = [];

function preloadFrames() {
  for (let i = 1; i <= frameCount; i++) {
    const img = new Image();
    img.src = `/frames/frame_${String(i).padStart(4, "0")}.webp`;
    frames.push(img);
  }
}

gsap.to({ frame: 0 }, {
  frame: frameCount - 1,
  snap: "frame",
  ease: "none",
  scrollTrigger: {
    trigger: "#sequence-section", start: "top top", end: "+=3000", pin: true, scrub: 0.5,
  },
  onUpdate: function() {
    const index = Math.round(this.targets()[0].frame);
    ctx.clearRect(0, 0, canvas.width, canvas.height);
    if (frames[index]?.complete) {
      ctx.drawImage(frames[index], 0, 0, canvas.width, canvas.height);
    }
  }
});
```

### Alternative: Video Scrub

```js
const video = document.getElementById("scrub-video");

gsap.to(video, {
  currentTime: video.duration,
  ease: "none",
  scrollTrigger: { trigger: "#video-section", start: "top top", end: "+=4000", pin: true, scrub: true }
});
```

**Tradeoff:** Video scrub = smaller payload, less smooth on mobile. Frame sequence = more bytes, smoother everywhere.

---

## Accessibility

```css
@media (prefers-reduced-motion: reduce) {
  *, *::before, *::after {
    animation-duration: 0.01ms !important;
    transition-duration: 0.01ms !important;
    scroll-behavior: auto !important;
  }
}
```

```js
const prefersReduced = window.matchMedia("(prefers-reduced-motion: reduce)").matches;
if (prefersReduced) { ScrollTrigger.getAll().forEach(st => st.kill()); }
```

---

## Workflow

1. **Define scroll sections** — wireframe which content pins, reveals, or plays
2. **Pick technique** — use Decision Matrix above
3. **Implement with GSAP** — pin/scrub/snap for complex, CSS for simple reveals
4. **Add Lenis** — only if smooth scroll feel is required
5. **Test performance** — Chrome DevTools Performance panel, aim for <16.6ms/frame
6. **Add a11y** — `prefers-reduced-motion`, keyboard nav still works
7. **Test mobile** — reduce frame counts, disable heavy effects on low-end
