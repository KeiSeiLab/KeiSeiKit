---
name: landing-page
description: Use when creating a landing page — orchestrates design, copy, assets, animations, SEO. Supports recipe system for specific page types (apple-product, saas, portfolio, ecommerce). Triggers on "landing page", "create page", "website".
arguments:
  - name: product
    description: Product/service name and brief description
    required: true
  - name: recipe
    description: "Recipe: apple-product, saas, portfolio, ecommerce, agency, startup (auto-suggest if omitted)"
    required: false
  - name: goal
    description: "Page goal: signups, downloads, waitlist, sales, portfolio showcase"
    required: false
---

# Landing Page Orchestrator

Creates premium landing pages by composing specialized skills.

## Step 1: Design Direction

Invoke `/frontend-design` with product context:
- Suggest archetype based on recipe (see matrix below)
- Define differentiator, anti-references, tokens
- Output: design direction + CSS custom properties

## Step 2: Research & Copy

- Understand product: features, audience, value proposition
- WebSearch 3-5 competitors for positioning
- Write copy: headline (<10 words, benefit-driven), subheadline, CTAs, feature descriptions
- Tone matches archetype from Step 1

## Step 3: Page Structure

Adapt structure to recipe (see below). Core sections:
1. **Hero** — headline, subheadline, CTA, visual
2. **Problem** — pain point (empathy)
3. **Solution** — how product solves it (3 features max)
4. **Social proof** — testimonials, metrics, logos
5. **How it works** — 3-step process
6. **Pricing** (if applicable)
7. **FAQ** (3-5 questions)
8. **Final CTA** — repeat conversion action

## Step 4: Implementation

- Framework: Astro 6 (default for marketing) or project's stack
- Invoke skills per recipe (see matrix)
- Mobile-first responsive design
- Performance: lazy load below-fold, optimize all assets

## Step 5: Quality Pipeline

Sequential audit chain:
1. `/web-assets audit` — image formats, sizes, fonts
2. `/a11y-audit scan` — WCAG 2.2 AA compliance
3. `/seo-audit` — meta, headings, schema, OG tags
4. `/responsive-audit` — 6 breakpoints
5. `/perf-audit` — Lighthouse >90

## Step 6: Deploy

`/web-deploy deploy` — Cloudflare Pages (default)

---

## Recipe System

### `apple-product` — Premium Product Reveal

**Archetype:** Minimal or Swiss
**Skills invoked:** ai-animation, scroll-animation, video-gen, 3d-scene, web-assets, motion-design

**Structure:**
1. Hero: product floating in space, minimal text
2. Video scrub section: product rotation/reveal on scroll (frame sequence or 3D)
3. Feature deep-dives: pin + scrub with parallax text
4. Specs grid: bento layout with micro-animations
5. CTA: clean, single action

**Key techniques:**
- Frame sequence (120-180 WebP frames) or Three.js model with ScrollControls
- GSAP ScrollTrigger pin + scrub
- Lenis smooth scroll
- Staggered text reveals with blur-in animation
- Dark background, dramatic lighting

### `saas` — SaaS Product Landing

**Archetype:** Minimal or Editorial
**Skills invoked:** motion-design, ui-component, web-assets, form-builder

**Structure:**
1. Hero: headline + product screenshot/video + CTA
2. Logo bar: client/integration logos
3. Features: bento grid (3-6 cards) with hover micro-interactions
4. Demo: embedded video or interactive preview
5. Testimonials: carousel or grid with photos
6. Pricing: 2-3 tier comparison table
7. FAQ: accordion
8. CTA: signup form (Turnstile + Zod)

**Key techniques:**
- Bento grid layout with staggered entrance
- View Transitions for page navigation
- Dark/light mode toggle
- Micro-interactions on every card (hover scale, shadow elevation)
- Auto-animate for list/grid transitions

### `portfolio` — Creative Portfolio

**Archetype:** Editorial or Maximalist
**Skills invoked:** scroll-animation, web-effects, motion-design, 3d-scene

**Structure:**
1. Hero: kinetic typography (name/title animates on load)
2. Project showcase: horizontal scroll or masonry grid
3. Project detail: image distortion on hover (WebGL)
4. About: asymmetric editorial layout
5. Contact: minimal form

**Key techniques:**
- Custom cursor that reacts to content
- Image distortion on hover (curtains.js displacement)
- GSAP horizontal scroll for project gallery
- SVG line drawing for decorative elements
- Kinetic typography with SplitText

### `ecommerce` — Product E-Commerce

**Archetype:** Minimal or Organic
**Skills invoked:** ui-component, web-assets, form-builder, motion-design

**Structure:**
1. Hero: product lifestyle image + CTA
2. Product grid: filterable with auto-animate transitions
3. Product detail: gallery + variant selector + add-to-cart
4. Reviews: social proof grid
5. Related products: horizontal scroll
6. Trust: shipping, returns, secure payment badges

**Key techniques:**
- Image zoom on hover
- Variant selector with instant preview update
- Add-to-cart animation (fly to cart icon)
- Skeleton loading states
- Optimistic UI updates

### `agency` — Creative Agency

**Archetype:** Brutalist or Swiss
**Skills invoked:** scroll-animation, web-effects, 3d-scene, motion-design

**Structure:**
1. Hero: bold statement + reel/showreel video
2. Services: icon grid with hover reveals
3. Case studies: full-bleed image + overlay text
4. Team: grid with playful hover effects
5. Process: timeline with scroll-linked progress
6. Contact: multi-step form

**Key techniques:**
- Full-screen video hero (AV1 + H.264 fallback)
- Noise/grain texture overlay
- Scroll-driven timeline with pin sections
- Magnetic cursor on interactive elements
- Page transitions with View Transitions API

### `startup` — Early-Stage Startup

**Archetype:** Minimal or Retro-Futuristic
**Skills invoked:** motion-design, form-builder, web-assets

**Structure:**
1. Hero: problem statement + waitlist CTA
2. Pain points: 3 illustrated scenarios
3. Solution: how it works (3 steps)
4. Early metrics/traction (if available)
5. Founder story (optional)
6. Waitlist form with social proof counter

**Key techniques:**
- Simple fade-in animations (AutoAnimate)
- Email capture with Turnstile
- Social proof: "Join 1,234 others" counter
- Minimal JavaScript, maximum speed
- Ship fast: Astro + Tailwind + Cloudflare Pages
