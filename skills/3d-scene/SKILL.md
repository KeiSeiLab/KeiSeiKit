---
name: 3d-scene
description: Use when building 3D scenes for web — Three.js, React Three Fiber (R3F), Spline embeds, GLTF/GLB loading, scroll-linked 3D, camera animations. Covers scene setup, model optimization, and performance budgets.
arguments:
  - name: approach
    description: "Approach: r3f, threejs-vanilla, spline-embed (auto-detect if omitted)"
    required: false
  - name: interaction
    description: "Interaction: scroll-linked, orbit, hover, static (default: scroll-linked)"
    required: false
---

# 3D Scene Skill

## Decision Matrix — Pick Approach

| Need | Approach | Bundle Impact | Complexity |
|------|----------|---------------|------------|
| Product showcase with scroll | R3F + ScrollControls | ~150KB (Three.js) | Medium |
| Hero 3D scene (no-code) | Spline embed | ~0KB (iframe) | Low |
| Custom shaders + 3D | Three.js vanilla | ~150KB | High |
| Lightweight shader-only | OGL | ~8KB | Medium |
| Interactive configurator | R3F + Leva/dat.gui | ~150KB | Medium |

---

## 1. React Three Fiber (R3F) + Drei [E1]

**Install:**
```bash
npm i three @react-three/fiber @react-three/drei
```

### Basic Scene

```jsx
import { Canvas } from "@react-three/fiber";
import { OrbitControls, Environment, useGLTF } from "@react-three/drei";

function Model({ url }) {
  const { scene } = useGLTF(url);
  return <primitive object={scene} />;
}

function Scene() {
  return (
    <Canvas
      camera={{ position: [0, 2, 5], fov: 45 }}
      style={{ width: "100%", height: "100vh" }}
    >
      <Environment preset="studio" />
      <Model url="/models/product.glb" />
      <OrbitControls enableZoom={false} />
    </Canvas>
  );
}
```

### ScrollControls (Scroll-Linked 3D) [E1]

```jsx
import { Canvas, useFrame } from "@react-three/fiber";
import { ScrollControls, Scroll, useScroll } from "@react-three/drei";

function AnimatedModel() {
  const scroll = useScroll();
  const ref = useRef();

  useFrame(() => {
    const offset = scroll.offset; // 0 to 1
    // Rotate model based on scroll
    ref.current.rotation.y = offset * Math.PI * 2;
    // Move camera/model along a path
    ref.current.position.y = offset * -5;
  });

  return (
    <mesh ref={ref}>
      <boxGeometry args={[1, 1, 1]} />
      <meshStandardMaterial color="royalblue" />
    </mesh>
  );
}

function ScrollScene() {
  return (
    <Canvas>
      <ScrollControls
        pages={5}       // 5 pages of scroll (500vh)
        damping={0.25}  // friction (seconds to catch up)
      >
        {/* 3D content */}
        <Scroll>
          <AnimatedModel />
        </Scroll>

        {/* HTML content overlaid */}
        <Scroll html>
          <div style={{ position: "absolute", top: "100vh" }}>
            <h2>Section 2</h2>
          </div>
          <div style={{ position: "absolute", top: "200vh" }}>
            <h2>Section 3</h2>
          </div>
        </Scroll>
      </ScrollControls>

      <ambientLight intensity={0.5} />
      <directionalLight position={[5, 5, 5]} />
    </Canvas>
  );
}
```

### useScroll Utilities

```jsx
const scroll = useScroll();

// scroll.offset     — 0 to 1 (overall progress)
// scroll.delta      — scroll speed
// scroll.range(from, distance)  — 0-1 within range
// scroll.curve(from, distance)  — bell curve within range
// scroll.visible(from, distance) — boolean visibility

useFrame(() => {
  // Animate only in section 2 (20%-40% of scroll)
  const sectionProgress = scroll.range(0.2, 0.2);
  ref.current.scale.setScalar(1 + sectionProgress * 0.5);

  // Fade in section 3
  const visible = scroll.visible(0.4, 0.2);
  material.current.opacity = visible ? scroll.curve(0.4, 0.2) : 0;
});
```

### R3F + GSAP (Alternative to ScrollControls) [E2]

When you need GSAP's pin/snap with 3D:

```jsx
import { useGSAP } from "@gsap/react";
import gsap from "gsap";
import { ScrollTrigger } from "gsap/ScrollTrigger";

function GSAPModel() {
  const mesh = useRef();

  useGSAP(() => {
    gsap.to(mesh.current.rotation, {
      y: Math.PI * 2,
      scrollTrigger: {
        trigger: "#scene-container",
        start: "top top",
        end: "+=3000",
        pin: true,
        scrub: 1,
      }
    });
  });

  return <mesh ref={mesh}>...</mesh>;
}
```

---

## 2. Spline Embeds [E2]

**What:** No-code 3D design tool with web export.
**Best for:** Quick 3D heroes, interactive product views, landing pages.

### Embed Methods

```jsx
// Method 1: iframe (simplest, no bundle impact)
<iframe
  src="https://my.spline.design/scene-abc123/"
  frameBorder="0"
  width="100%"
  height="500px"
  style={{ border: "none" }}
/>

// Method 2: React component (more control)
import Spline from "@splinetool/react-spline";

<Spline scene="https://prod.spline.design/abc123/scene.splinecode" />

// Method 3: Vanilla JS
import { Application } from "@splinetool/runtime";
const app = new Application(canvas);
app.load("https://prod.spline.design/abc123/scene.splinecode");
```

### Spline Capabilities

- Scroll-linked animations (native events)
- Mouse follow / hover interactions
- State changes on click
- Physics simulations
- Responsive layout
- AI text-to-3D / image-to-3D generation

### Spline Limitations

- **Performance:** Keep <3 lights per scene
- **Polygons:** Smooth subdivision max 2 levels
- **Loading:** Scenes can be 1-5MB+ for complex objects
- **Control:** Less fine-grained than custom Three.js
- **Offline:** Requires network to load from Spline CDN (self-hosted export available)
- **Scroll sync:** Less precise than R3F ScrollControls or GSAP

### Spline Optimization

- Delete invisible objects (inside or behind other objects)
- Use <3 lights
- Compress on export (quality vs size tradeoff)
- Reduce subdivision levels
- Optimize CAD imports: strip internal geometry, small fillets
- Keep total polygon count reasonable for target devices

---

## 3. GLTF/GLB Loading & Optimization [E1]

### The Pipeline

```
Source (FBX/OBJ/Blender)
  → Export as GLTF/GLB
  → Optimize with gltf-transform
    → Draco/Meshopt compression
    → KTX2 texture compression
    → Mesh quantization
  → Load in Three.js/R3F
```

### gltf-transform Optimization [E1]

```bash
# Install
npm i -g @gltf-transform/cli

# Full optimization pipeline
gltf-transform optimize input.glb output.glb \
  --compress draco \
  --texture-compress webp

# Or step by step:
gltf-transform dedup input.glb deduped.glb           # remove duplicate data
gltf-transform draco deduped.glb compressed.glb       # geometry compression
gltf-transform webp compressed.glb textured.glb       # texture to WebP
gltf-transform quantize textured.glb output.glb       # mesh quantization
```

### Real-World Size Reductions

| Stage | Example Size | Reduction |
|-------|-------------|-----------|
| Raw FBX | 50 MB | baseline |
| GLTF/GLB export | 29 MB | -42% |
| Draco compression | 5 MB | -83% |
| + KTX2 textures | 2.5 MB | -91% |
| + Mesh quantization | 2 MB | -93% |

### Compression Methods

| Method | Compression | Decode Speed | Three.js Version |
|--------|-------------|--------------|-------------------|
| Draco | Best (~90%) | Slower (Web Worker) | r100+ |
| Meshopt | Good (~85%) | Faster | r122+ |
| Quantization | Moderate (~50%) | Instant | r100+ |

**Decision:** Draco for smallest files, Meshopt for fastest client decode. Combine with KTX2 textures.

### Loading in R3F

```jsx
import { useGLTF, useTexture } from "@react-three/drei";

// Preload for instant display
useGLTF.preload("/models/product.glb");

function Product() {
  const { scene, nodes, materials } = useGLTF("/models/product.glb");

  return (
    <primitive
      object={scene}
      scale={0.5}
      position={[0, -1, 0]}
    />
  );
}

// With Draco decoder
import { DRACOLoader } from "three/examples/jsm/loaders/DRACOLoader";

// In Canvas setup or loader config:
// DRACOLoader points to decoder files (usually from CDN)
```

---

## 4. Performance Budgets [E1]

### Polygon Limits

| Device Class | Max Triangles | Draw Calls | Notes |
|-------------|---------------|------------|-------|
| High-end desktop | 500K-1M | <200 | Gaming GPU |
| Average desktop | 100K-300K | <100 | Integrated GPU |
| Mobile (flagship) | 50K-150K | <50 | iPhone 14+ level |
| Mobile (budget) | 20K-50K | <30 | Older Android |

**Key insight:** Draw call count matters MORE than polygon count. Below 100 draw calls, most devices maintain 60fps. Above 500, even powerful GPUs struggle.

### Texture Budgets

| Texture | Max Size | Format | VRAM |
|---------|----------|--------|------|
| Diffuse/Albedo | 2048x2048 | KTX2/WebP | ~5MB |
| Normal map | 1024x1024 | KTX2 | ~1.3MB |
| Roughness/Metal | 512x512 | KTX2 | ~0.3MB |
| Environment | 256x256 cube | HDR/EXR | ~2MB |

**Critical:** A 200KB PNG on disk = 20MB+ in VRAM! KTX2 with Basis Universal stays compressed on GPU (~10x reduction).

### R3F Performance Tips

```jsx
// 1. Use instancing for repeated objects
import { Instances, Instance } from "@react-three/drei";

<Instances>
  {positions.map((pos, i) => (
    <Instance key={i} position={pos} />
  ))}
</Instances>

// 2. Frustum culling (enabled by default in Three.js)
// 3. LOD (Level of Detail)
import { Detailed } from "@react-three/drei";

<Detailed distances={[0, 50, 100]}>
  <HighPolyModel />
  <MedPolyModel />
  <LowPolyModel />
</Detailed>

// 4. Limit pixel ratio on mobile
<Canvas dpr={[1, 2]}> {/* max 2x, not device native */}

// 5. Use drei's Preload for loading screen
import { Preload } from "@react-three/drei";
<Canvas>
  <Suspense fallback={null}>
    <Scene />
  </Suspense>
  <Preload all />
</Canvas>
```

### Monitor Performance

```jsx
import { useFrame } from "@react-three/fiber";
import { Stats } from "@react-three/drei";

// Show FPS/MS/MB overlay
<Stats />

// Or manual monitoring
useFrame((state) => {
  const info = state.gl.info;
  // info.render.triangles — triangles per frame
  // info.render.calls — draw calls per frame
  // info.memory.textures — loaded textures
  // info.memory.geometries — loaded geometries
});
```

---

## 5. Lighting & Environment [E2]

### Quick Setups (drei)

```jsx
// Studio lighting (product showcase)
<Environment preset="studio" />

// HDR environment
<Environment files="/hdri/warehouse.hdr" />

// Simple 3-point lighting
<ambientLight intensity={0.3} />
<directionalLight position={[5, 5, 5]} intensity={1} castShadow />
<directionalLight position={[-3, 3, -5]} intensity={0.5} />

// Contact shadows (cheap fake shadows)
<ContactShadows
  position={[0, -1, 0]}
  opacity={0.5}
  scale={10}
  blur={2}
/>
```

### Environment Presets (drei)

Available: `apartment`, `city`, `dawn`, `forest`, `lobby`, `night`, `park`, `studio`, `sunset`, `warehouse`

---

## 6. Common Patterns

### Product Reveal on Scroll

```jsx
function ProductReveal() {
  return (
    <Canvas>
      <ScrollControls pages={4} damping={0.3}>
        <Scroll>
          <ProductModel />  {/* rotates with scroll */}
        </Scroll>
        <Scroll html>
          <section style={{ top: "100vh" }}>Feature 1</section>
          <section style={{ top: "200vh" }}>Feature 2</section>
          <section style={{ top: "300vh" }}>CTA</section>
        </Scroll>
      </ScrollControls>
      <Environment preset="studio" />
    </Canvas>
  );
}
```

### Floating Objects (Parallax)

```jsx
import { Float } from "@react-three/drei";

<Float
  speed={2}           // animation speed
  rotationIntensity={0.5}
  floatIntensity={0.5}
>
  <mesh>
    <torusGeometry args={[1, 0.3, 16, 32]} />
    <meshStandardMaterial color="hotpink" />
  </mesh>
</Float>
```

### Text in 3D

```jsx
import { Text3D, Center } from "@react-three/drei";

<Center>
  <Text3D
    font="/fonts/inter_bold.json"
    size={0.75}
    height={0.2}
    curveSegments={12}
  >
    Hello World
    <meshNormalMaterial />
  </Text3D>
</Center>
```

---

## Workflow

1. **Define the 3D need** — is it product showcase, decoration, interactive?
2. **Pick approach** — R3F for code control, Spline for no-code, OGL for shader-only
3. **Prepare models** — export GLTF/GLB, optimize with gltf-transform
4. **Set up scene** — Camera, lighting, environment
5. **Add scroll interaction** — ScrollControls or GSAP integration
6. **Optimize** — polygon budget, texture compression, instancing
7. **Test performance** — Stats component, Chrome DevTools, test on mobile
8. **Add fallbacks** — static image for low-end devices, loading state
