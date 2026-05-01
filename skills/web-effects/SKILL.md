---
name: web-effects
description: Use when building visual web effects — WebGL shaders, image distortion, particles, noise/grain, hover effects, displacement maps. Covers curtains.js, OGL, tsParticles, custom WebGL, and CSS-only effects.
arguments:
  - name: effect
    description: "Effect: distortion, particles, noise, hover, displacement, gradient, blur (auto-detect if omitted)"
    required: false
  - name: approach
    description: "Approach: css-only, webgl, canvas, library (auto-detect by complexity)"
    required: false
---

# Web Effects Skill

## Decision Matrix — Pick Approach

| Effect | CSS Only | Canvas 2D | WebGL (library) | Custom WebGL |
|--------|----------|-----------|-----------------|--------------|
| Image hover distortion | No | No | curtains.js | Possible |
| Particles (decorative) | Limited | Possible | tsParticles | Best perf |
| Noise/grain overlay | Yes | Yes | Shader | Overkill |
| Gradient animation | Yes | Possible | Unnecessary | No |
| Blur/glassmorphism | Yes | No | No | No |
| Displacement on scroll | No | No | curtains.js/OGL | Possible |
| Liquid/fluid effects | No | No | OGL | Yes |
| Image reveal/transition | CSS clip-path | Canvas | curtains.js | Possible |

**Rule:** Start with CSS. Escalate to Canvas/WebGL only when CSS cannot achieve the effect.

---

## 1. Curtains.js — DOM-Driven WebGL

**Bundle:** ~30KB min+gzip
**What it does:** Converts HTML images/videos/canvases into WebGL textured planes that stay positioned with DOM layout.

**Best for:** Image hover distortion, displacement effects, WebGL transitions between slides.

```js
import { Curtains, Plane } from "curtainsjs";

const curtains = new Curtains({ container: "#canvas" });

const plane = new Plane(curtains, document.querySelector(".image-wrapper"), {
  vertexShader: vertexShaderSource,
  fragmentShader: fragmentShaderSource,
  uniforms: {
    uMouse: { name: "uMouse", type: "2f", value: [0, 0] },
    uTime:  { name: "uTime",  type: "1f", value: 0 },
  }
});

plane.onRender(() => { plane.uniforms.uTime.value++; });

document.querySelector(".image-wrapper").addEventListener("mousemove", (e) => {
  const rect = e.target.getBoundingClientRect();
  plane.uniforms.uMouse.value = [
    (e.clientX - rect.left) / rect.width,
    1 - (e.clientY - rect.top) / rect.height
  ];
});
```

### Displacement Shader (Hover Distortion)

```glsl
precision mediump float;
varying vec2 vTextureCoord;
uniform sampler2D uSampler0;
uniform sampler2D uDisplacement;
uniform vec2 uMouse;

void main() {
  vec2 uv = vTextureCoord;
  vec4 disp = texture2D(uDisplacement, uv);
  float dist = distance(uv, uMouse);
  float strength = smoothstep(0.3, 0.0, dist) * 0.05;
  uv += disp.rg * strength;
  gl_FragColor = texture2D(uSampler0, uv);
}
```

**Note:** `gpu-curtains` is a WebGPU successor worth watching.

---

## 2. OGL — Minimal WebGL

**Bundle:** ~8KB gzip, zero dependencies
**What it does:** Thin WebGL abstraction, you write your own shaders.

**Best for:** Custom shader effects, fullscreen post-processing, when curtains.js is too opinionated.

```js
import { Renderer, Camera, Program, Mesh, Plane } from "ogl";

const renderer = new Renderer();
const gl = renderer.gl;
document.body.appendChild(gl.canvas);

const camera = new Camera(gl);
camera.position.z = 1;

const geometry = new Plane(gl);

const program = new Program(gl, {
  vertex: `
    attribute vec3 position;
    attribute vec2 uv;
    varying vec2 vUv;
    void main() { vUv = uv; gl_Position = vec4(position, 1.0); }
  `,
  fragment: `
    precision highp float;
    varying vec2 vUv;
    uniform float uTime;
    void main() {
      gl_FragColor = vec4(vec3(sin(uTime + vUv.x * 6.28) * 0.5 + 0.5), 1.0);
    }
  `,
  uniforms: { uTime: { value: 0 } }
});

const mesh = new Mesh(gl, { geometry, program });

function update(t) {
  requestAnimationFrame(update);
  program.uniforms.uTime.value = t * 0.001;
  renderer.render({ scene: mesh, camera });
}
requestAnimationFrame(update);
```

**OGL vs Three.js:** OGL is 8KB vs Three.js ~150KB. Use OGL for shader effects where you do not need a scene graph, models, or physics.

---

## 3. Particles

### tsParticles (Library)

**Install:** `npm i tsparticles`
**Bundle:** ~40KB min+gzip (core), modular
**Frameworks:** React, Vue, Svelte, Angular, Solid, vanilla

```jsx
import Particles from "@tsparticles/react";
import { loadSlim } from "@tsparticles/slim";

function Background() {
  const init = useCallback(async (engine) => { await loadSlim(engine); }, []);

  return (
    <Particles
      init={init}
      options={{
        particles: {
          number: { value: 50 },
          size: { value: { min: 1, max: 3 } },
          move: { enable: true, speed: 0.5 },
          opacity: { value: { min: 0.1, max: 0.5 } },
          links: { enable: true, distance: 150, opacity: 0.2 },
        },
        detectRetina: true,
      }}
    />
  );
}
```

### Custom WebGL Particles (Performance-Critical)

When you need 10K+ particles at 60fps, do everything in shaders:

```glsl
attribute vec3 position;
attribute vec2 velocity;
attribute float life;
uniform float uTime;
uniform float uDelta;

void main() {
  vec3 pos = position + vec3(velocity * uDelta, 0.0);
  gl_Position = projectionMatrix * modelViewMatrix * vec4(pos, 1.0);
  gl_PointSize = mix(3.0, 0.0, life);
}
```

**Decision:** tsParticles for <1000 particles with config flexibility. Custom WebGL for >1000 particles or specific visual needs.

---

## 4. CSS-Only Effects

### Animated Gradient

```css
.gradient-bg {
  background: linear-gradient(-45deg, #ee7752, #e73c7e, #23a6d5, #23d5ab);
  background-size: 400% 400%;
  animation: gradient-shift 15s ease infinite;
}
@keyframes gradient-shift {
  0%   { background-position: 0% 50%; }
  50%  { background-position: 100% 50%; }
  100% { background-position: 0% 50%; }
}
```

### Noise/Grain Overlay (CSS)

```css
.grain::after {
  content: "";
  position: fixed;
  inset: 0;
  background-image: url("data:image/svg+xml,...");
  opacity: 0.05;
  pointer-events: none;
  z-index: 9999;
  mix-blend-mode: overlay;
}
```

### Glassmorphism

```css
.glass {
  background: rgba(255, 255, 255, 0.1);
  backdrop-filter: blur(12px) saturate(150%);
  -webkit-backdrop-filter: blur(12px) saturate(150%);
  border: 1px solid rgba(255, 255, 255, 0.15);
  border-radius: 16px;
}
```

### Image Reveal (Clip-Path)

```css
.reveal {
  clip-path: inset(0 100% 0 0);
  transition: clip-path 0.8s cubic-bezier(0.77, 0, 0.175, 1);
}
.reveal.visible { clip-path: inset(0 0 0 0); }
```

### Hover Magnetic Effect (JS Required)

```js
const btn = document.querySelector(".magnetic-btn");
btn.addEventListener("mousemove", (e) => {
  const rect = btn.getBoundingClientRect();
  const x = e.clientX - rect.left - rect.width / 2;
  const y = e.clientY - rect.top - rect.height / 2;
  btn.style.transform = `translate(${x * 0.3}px, ${y * 0.3}px)`;
});
btn.addEventListener("mouseleave", () => {
  btn.style.transform = "translate(0, 0)";
  btn.style.transition = "transform 0.5s ease";
});
```

---

## 5. Performance Rules

### GPU-Composited Properties (animate these)

```
transform    — translate, rotate, scale
opacity      — fade in/out
filter       — blur, brightness
clip-path    — reveal/hide
```

### Layout-Triggering Properties (avoid animating)

```
width, height, top, left, right, bottom
margin, padding, border-width
font-size, line-height
```

### will-change

```css
.about-to-animate { will-change: transform, opacity; }
/* Do NOT: * { will-change: transform; } */
```

### Frame Budget

- **60fps target:** 16.66ms per frame
- **Pause offscreen:** IntersectionObserver to stop animations outside viewport

```js
const observer = new IntersectionObserver(([entry]) => {
  if (entry.isIntersecting) startRenderLoop();
  else stopRenderLoop();
});
observer.observe(canvasElement);
```

---

## Workflow

1. **Define the effect** — what visual result is needed?
2. **Try CSS first** — gradient, blur, clip-path, mix-blend-mode
3. **Escalate to Canvas/WebGL** — only if CSS cannot achieve it
4. **Pick library** — curtains.js for DOM-synced, OGL for custom shaders
5. **Write shader** — keep fragment shaders simple, profile on mobile
6. **Add IntersectionObserver** — pause offscreen effects
7. **Test performance** — Chrome DevTools Performance, GPU memory
8. **Add prefers-reduced-motion** — disable or simplify effects
