# Stack Compatibility Matrix

## Verified Stack Combinations

### Fullstack TypeScript (Primary Stack — Denis/KeiSei)
```
Frontend:  Next.js 14+ (App Router) + React 18+ + Zustand + Tailwind CSS
Backend:   NestJS 10+ + Prisma ORM + PostgreSQL + Redis
Queue:     BullMQ + Redis
Bot:       Grammy (Telegram)
Storage:   MinIO (S3-compatible)
Auth:      NextAuth 5 / Passport.js
Search:    pgvector (vector similarity)
Deploy:    Docker Compose + nginx reverse proxy + certbot SSL
```

### AI/ML Stack
```
API:       OpenAI SDK + Anthropic SDK + Google Generative AI
Embeddings: text-embedding-3-small (OpenAI) / pgvector storage
RAG:       Custom chunking + embeddings + vector search
Images:    Gemini nano-banana-pro / imagen-4.0-ultra
Video:     Veo 3.1 / Kling 3.0 via Fal.ai
Batch:     Anthropic Batch API (max_tokens=8192, batch_size=25)
```

### Game Dev Stack
```
Engine:    Phaser 3.90 + TypeScript + Vite
Emulation: Nostalgist.js (snes9x core)
Assets:    DALL-E 3 (sprites) + procedural generation
```

### Systems Programming Stack
```
Kernel:    C + eBPF
Userspace: Go
Training:  Python + WGAN-GP
Pattern:   Event Stream → Ring Buffer → Feature Extraction → MicroML → Decision
```

## Technology Compatibility Rules

### Node.js Version Constraints
| Package | Min Node | Notes |
|---------|----------|-------|
| Next.js 14+ | 18.17+ | App Router requires 18.17+ |
| NestJS 10+ | 16+ | |
| Prisma 5+ | 16.13+ | |
| BullMQ 5+ | 18+ | |
| Grammy 1.x | 16+ | |

### Database Compatibility
| ORM | PostgreSQL | MySQL | SQLite | MongoDB |
|-----|-----------|-------|--------|---------|
| Prisma | Full | Full | Full | Full |
| TypeORM | Full | Full | Full | Full |
| Drizzle | Full | Full | Full | No |
| Sequelize | Full | Full | Full | No |
| Knex | Full | Full | Full | No |

### pgvector Requirements
- PostgreSQL 15+ recommended (14+ minimum)
- Prisma: use `$queryRawUnsafe` for `::vector` cast (not `$queryRaw`)
- Index type: IVFFlat (fast, approximate) or HNSW (slower build, faster query)
- Dimension limit: 2000 (OpenAI ada-002 = 1536, text-embedding-3-small = 1536)

### Next.js + NestJS Integration Rules
1. `NEXT_PUBLIC_*` vars baked at BUILD time — not runtime
2. Docker Compose reads `.env` NOT `.env.production`
3. Behind nginx: set `AUTH_TRUST_HOST=true` for NextAuth
4. API routes: `export const dynamic = "force-dynamic"` (no DB at build)
5. Mixed content: always use domain URL, never IP:port

### Docker Networking
- Shared network: all services can resolve by container name
- Port mapping: `host:container` — host port must be free
- DNS: container names as hostnames (e.g., `tip-postgres`, `tip-redis`)
- Health checks: always add for DB and Redis before app starts

### CSS Framework Compatibility
| Framework | React | Vue | Svelte | Angular |
|-----------|-------|-----|--------|---------|
| Tailwind CSS | Full | Full | Full | Full |
| styled-components | Full | No | No | No |
| Emotion | Full | No | No | No |
| CSS Modules | Full | Full | Full | Full |

## Anti-Compatible Combinations (DO NOT MIX)

| Bad Combo | Why | Use Instead |
|-----------|-----|-------------|
| TypeORM + Prisma in same project | Conflicting migration systems | Pick one ORM |
| Express + NestJS (raw) | NestJS wraps Express internally | Use NestJS decorators |
| Redux + Zustand in same project | Two state managers fighting | Pick one |
| Mongoose + Prisma MongoDB | Two ODMs, schema conflicts | Pick one |
| Webpack + Vite in same project | Two bundlers | Vite (modern) or Webpack (legacy) |
| npm + pnpm in same project | Lock file conflicts | Pick one package manager |
| .env.local + .env.production + .env | Loading order confusion | Use `.env` only in Docker |

## Migration Paths

### Express → NestJS
1. Create NestJS project alongside
2. Move routes → controllers one by one
3. Move middleware → guards/interceptors
4. Move services → injectable providers
5. Test each migration step

### Redux → Zustand
1. Create Zustand store matching Redux state shape
2. Replace `useSelector` → Zustand selectors
3. Replace `dispatch(action)` → store methods
4. Remove Redux boilerplate (reducers, action creators)

### Pages Router → App Router (Next.js)
1. Move `pages/api/*` → `app/api/*/route.ts`
2. Move `pages/*.tsx` → `app/*/page.tsx`
3. Add `'use client'` to interactive components
4. Replace `getServerSideProps` → server components
5. Replace `getStaticProps` → `generateStaticParams`

## Web Creation Stack (March 2026)

### Framework Selection
| Framework | Use When | Zero JS | Islands |
|-----------|----------|---------|---------|
| **Astro 6** | Marketing, content, portfolios (DEFAULT) | Yes | Yes |
| Next.js 16 | Full-stack React apps, dashboards | No | No |
| SvelteKit 5 | Animation-heavy, mobile-first | Compiles | No |

- Astro acquired by Cloudflare (Jan 2026) — first-class Workers integration
- Next.js 16: Turbopack default, React Compiler stable, PPR
- SvelteKit: 50%+ less JS than Next.js

### Animation & Motion
| Library | Version | Size | Use When |
|---------|---------|------|----------|
| GSAP | 3.14+ | ~28KB | ScrollTrigger, pin, scrub, timeline (100% FREE — Webflow) |
| Motion | 12.x | ~18KB | React animations, layout, gestures |
| Lenis | 1.3+ | ~14KB | Smooth scroll (industry standard) |
| AutoAnimate | 0.9 | ~2KB | Zero-config FLIP animations |
| Rive | runtime ~78KB | WASM | Interactive animations (state machines) |
| Lottie (dotlottie) | v3 | varies | After Effects animations |

### 3D & Visual Effects
| Library | Size | Use When |
|---------|------|----------|
| Three.js + R3F | ~150KB | Full 3D scenes, scroll-linked |
| Spline | embed | No-code 3D (limited precision) |
| curtains.js | ~30KB | DOM-synced WebGL (image distortion) |
| OGL | ~8KB | Minimal WebGL, custom shaders |
| tsParticles | config | Particle effects (<1000 particles) |

### Asset Pipeline
| Tool | Purpose |
|------|---------|
| Sharp.js | Image processing (AVIF/WebP/responsive) |
| glyphhanger | Font subsetting (60%+ reduction) |
| FFmpeg | Video encoding, frame extraction |
| nano-banana | AI image generation (Gemini) |

### Deployment
| Platform | Free Tier | Best For |
|----------|-----------|----------|
| **Cloudflare Pages** | Unlimited BW, 500 builds/mo | DEFAULT |
| Vercel | 100GB BW, 100 deploys/day | Next.js apps |
| Netlify | 100GB BW, 300 build min | Static + forms |

### RAG / Embeddings
| Component | Default | Alternative |
|-----------|---------|-------------|
| Vector DB | LanceDB (embedded, free) | Qdrant, Pinecone |
| Embeddings | OpenAI text-embedding-3-small ($0.02/MTok) | Gemini 2 (multimodal), Voyage (domain) |
| PDF parsing | PyMuPDF4LLM (Python) | pdf-parse (Node.js) |
| Chunking | Recursive 512tok/50overlap | Semantic, hierarchical |

### Forms & Validation
| Library | Version | Notes |
|---------|---------|-------|
| react-hook-form | v7 | Production. v8 beta — AVOID |
| Zod | v4 | Schema shared client+server (SSOT) |
| Turnstile | - | Free CAPTCHA, unlimited (Cloudflare) |

### CSS (Tailwind v4)
- `@theme` directive replaces tailwind.config.ts
- OKLCH color space for perceptually uniform palettes
- CSS-first configuration

### Anti-Compatible (Web Creation)
| Bad Combo | Why | Use Instead |
|-----------|-----|-------------|
| GSAP + CSS scroll-timeline for same element | Competing scroll handlers | Pick one per section |
| Lenis + native smooth scroll (scroll-behavior) | Double smooth-scrolling | Lenis only |
| Motion + GSAP in same component | Two animation systems fighting | Pick one per component |
| react-hook-form v7 + v8 in same project | Breaking API changes | Stick to v7 |
| Astro + full React hydration | Defeats zero-JS purpose | Use Astro islands (client:*) |

## Version Lock Warnings

When you see these in package.json, flag immediately:
- `"react": "^17"` with Next.js 14+ → needs React 18+
- `"prisma": "^4"` with Node 20+ → upgrade to Prisma 5+
- `"next-auth": "^4"` with App Router → consider NextAuth 5 beta
- `"bullmq": "^3"` with Redis 7+ → upgrade to BullMQ 5+
- `"gsap": "^3.11"` → upgrade to 3.14+ (all plugins now free)
- `"framer-motion"` → renamed to `motion` (npm install motion)
- Any `"*"` version → pin immediately
