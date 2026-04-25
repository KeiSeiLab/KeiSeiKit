---
name: motion-design
description: Use when implementing motion design — page transitions, element animations, micro-interactions, layout animations. Covers Motion (ex Framer Motion), View Transitions API, auto-animate, SVG animation (Rive, Lottie), and accessibility.
arguments:
  - name: type
    description: "Type: page-transition, micro-interaction, layout-animation, svg-animation, loading, hover (auto-detect if omitted)"
    required: false
  - name: framework
    description: "Framework: react, next, astro, vue, svelte, vanilla (auto-detect if omitted)"
    required: false
---

# Motion Design Skill

## Decision Matrix — Pick Library

| Need | Library | Bundle | Why |
|------|---------|--------|-----|
| React component animations | Motion 12 | ~32KB gzip | Best React DX, layout animations |
| Page transitions (MPA) | View Transitions API | 0KB | Native browser API |
| Page transitions (Astro) | Astro View Transitions | 0KB | Built-in, zero JS |
| Zero-config list animations | AutoAnimate | ~2KB gzip | One line, FLIP-based |
| Interactive vector graphics | Rive | ~78KB WASM | State machines, 60fps |
| After Effects exports | Lottie/dotLottie | ~50KB runtime | Huge asset library |
| SVG path morphing | GSAP MorphSVG | included in gsap | Now free, best morph engine |
| Line drawing | CSS stroke-dasharray | 0KB | Pure CSS, no library |

---

## 1. Motion (ex Framer Motion)

**Install:** `npm i motion`
**Bundle:** ~32KB min+gzip

### Core API

```jsx
import { motion, AnimatePresence } from "motion/react";

// Basic animation
<motion.div
  initial={{ opacity: 0, y: 20 }}
  animate={{ opacity: 1, y: 0 }}
  exit={{ opacity: 0, y: -20 }}
  transition={{ duration: 0.3, ease: "easeOut" }}
>
  Content
</motion.div>

// Layout animation (FLIP under the hood)
<motion.div layout layoutId="card-expand">
  {isExpanded ? <ExpandedCard /> : <CompactCard />}
</motion.div>

// AnimatePresence — exit animations
<AnimatePresence mode="wait">
  {items.map(item => (
    <motion.div
      key={item.id}
      initial={{ opacity: 0, scale: 0.9 }}
      animate={{ opacity: 1, scale: 1 }}
      exit={{ opacity: 0, scale: 0.9 }}
    />
  ))}
</AnimatePresence>
```

### Gestures

```jsx
<motion.button
  whileHover={{ scale: 1.05 }}
  whileTap={{ scale: 0.95 }}
  transition={{ type: "spring", stiffness: 400, damping: 17 }}
>
  Click me
</motion.button>

// Drag
<motion.div
  drag="x"
  dragConstraints={{ left: -200, right: 200 }}
  dragElastic={0.1}
/>
```

### Scroll-Linked

```jsx
import { useScroll, useTransform, motion } from "motion/react";

function ParallaxHero() {
  const { scrollYProgress } = useScroll();
  const y = useTransform(scrollYProgress, [0, 1], [0, -300]);
  const opacity = useTransform(scrollYProgress, [0, 0.5], [1, 0]);

  return (
    <motion.div style={{ y, opacity }}>
      Hero Content
    </motion.div>
  );
}
```

### AnimateView (View Transitions integration)

```jsx
import { AnimateView } from "motion/react";

<AnimateView>
  <Routes>
    <Route path="/" element={<Home />} />
    <Route path="/about" element={<About />} />
  </Routes>
</AnimateView>
```

---

## 2. View Transitions API

### Vanilla Implementation

```js
// Single-document transition
document.startViewTransition(() => {
  container.innerHTML = newContent;
});
```

```css
::view-transition-old(root) { animation: fade-out 0.2s ease-out; }
::view-transition-new(root) { animation: fade-in 0.3s ease-in; }

.hero-image { view-transition-name: hero; }
```

### Astro Integration (Built-in)

```astro
---
import { ViewTransitions } from "astro:transitions";
---
<html>
  <head><ViewTransitions /></head>
  <body><slot /></body>
</html>

<img transition:name="hero" src="/hero.jpg" />
<h1 transition:animate="slide">Page Title</h1>
```

Built-in animation presets: `fade`, `slide`, `morph`, `none`.

---

## 3. AutoAnimate

**Install:** `npm i @formkit/auto-animate`
**Zero config.** Uses FLIP technique internally.

```jsx
import { useAutoAnimate } from "@formkit/auto-animate/react";

function TodoList({ items }) {
  const [parent] = useAutoAnimate();
  return (
    <ul ref={parent}>
      {items.map(item => <li key={item.id}>{item.text}</li>)}
    </ul>
  );
}

// Vanilla JS
import autoAnimate from "@formkit/auto-animate";
autoAnimate(document.getElementById("list"));
```

**Best for:** List reordering, add/remove items, accordion expand/collapse.

---

## 4. SVG Animation

### Rive

```jsx
import Rive from "@rive-app/react-canvas";

<Rive
  src="/animations/hero.riv"
  stateMachines="MainState"
  style={{ width: 400, height: 400 }}
/>
```

**Key features:** State Machines, layout engine, scroll-linked via data inputs.
**When to use:** Interactive illustrations, mascots, loading states, onboarding flows.

### Lottie / dotLottie

```jsx
import { DotLottieReact } from "@lottiefiles/dotlottie-react";

<DotLottieReact
  src="/animations/hero.lottie"
  loop
  autoplay
  style={{ width: 300, height: 300 }}
/>
```

**Rive vs Lottie:**
| Factor | Rive | Lottie |
|--------|------|--------|
| Interactivity | Built-in state machine | Manual JS coding |
| Design tool | Rive editor | After Effects + plugin |
| File size | Smaller (binary) | Larger (JSON) |
| Asset ecosystem | Growing | Massive marketplace |

### SVG Morphing

**GSAP MorphSVG** (now free with gsap):
```js
gsap.to("#star", { morphSVG: "#circle", duration: 1, ease: "power2.inOut" });
```

**SVG points limit:** Keep under 200 points for smooth 60fps morphing.

### Line Drawing (Pure CSS)

```css
.svg-line {
  stroke-dasharray: 1000;
  stroke-dashoffset: 1000;
  animation: draw 2s ease forwards;
}
@keyframes draw { to { stroke-dashoffset: 0; } }
```

Get path length: `document.querySelector("path").getTotalLength()`.

---

## 5. Micro-Interaction Patterns

### Button Hover/Tap

```jsx
<motion.button
  whileHover={{ scale: 1.03, boxShadow: "0 4px 20px rgba(0,0,0,0.15)" }}
  whileTap={{ scale: 0.97 }}
  transition={{ type: "spring", stiffness: 500, damping: 25 }}
/>
```

### Toast/Notification Enter

```jsx
<AnimatePresence>
  {toast && (
    <motion.div
      initial={{ opacity: 0, y: 50, scale: 0.9 }}
      animate={{ opacity: 1, y: 0, scale: 1 }}
      exit={{ opacity: 0, y: 20, scale: 0.95 }}
      transition={{ type: "spring", damping: 20 }}
    />
  )}
</AnimatePresence>
```

### Staggered List

```jsx
const container = { animate: { transition: { staggerChildren: 0.06 } } };
const item = { initial: { opacity: 0, y: 15 }, animate: { opacity: 1, y: 0 } };

<motion.ul variants={container} initial="initial" animate="animate">
  {items.map(i => <motion.li key={i} variants={item} />)}
</motion.ul>
```

---

## 6. Timing & Easing Reference

### Duration Guidelines

| Element | Duration | Easing |
|---------|----------|--------|
| Button hover | 150-200ms | ease-out |
| Tooltip appear | 100-150ms | ease-out |
| Modal enter | 200-300ms | ease-out / spring |
| Modal exit | 150-200ms | ease-in |
| Page transition | 200-400ms | ease-in-out |
| Layout shift | 200-350ms | ease-out / spring |
| Scroll reveal | 400-600ms | ease-out |

### Spring Presets (Motion)

```js
// Snappy UI feedback
{ type: "spring", stiffness: 500, damping: 25 }
// Smooth layout
{ type: "spring", stiffness: 300, damping: 30 }
// Bouncy/playful
{ type: "spring", stiffness: 400, damping: 10 }
```

---

## 7. Accessibility

### prefers-reduced-motion

```css
@media (prefers-reduced-motion: reduce) {
  *, *::before, *::after {
    animation-duration: 0.01ms !important;
    transition-duration: 0.01ms !important;
  }
}
```

```jsx
// Motion respects prefers-reduced-motion by default
const prefersReduced = window.matchMedia("(prefers-reduced-motion: reduce)").matches;
```

### Guidelines

- Never rely on animation alone to convey information
- Ensure all animated content is accessible via keyboard
- Provide static fallback for critical content
- Test with reduced motion enabled in OS settings

---

## Workflow

1. **Identify animation type** — page transition, reveal, micro-interaction, SVG
2. **Pick library** — use Decision Matrix above
3. **Define timing** — use Duration Guidelines, spring presets
4. **Implement** — start with `initial` + `animate` states
5. **Add exit** — wrap in AnimatePresence for unmount animations
6. **Add a11y** — prefers-reduced-motion, keyboard testing
7. **Performance audit** — Chrome DevTools, check for layout thrashing
