---
name: seo-audit
description: Use when auditing SEO — technical + content analysis via WebFetch and code inspection
arguments:
  - name: url
    description: URL or project path to audit
    required: true
---

# SEO Audit Workflow

## When to use

- Auditing a live URL or project codebase for technical SEO issues (meta tags, canonical, sitemap, Core Web Vitals).
- Before launching a site or page to ensure discoverability and ranking fundamentals are in place.
- Generating a prioritised fix list covering both technical and content SEO signals.

## Step 1: Technical SEO
Fetch and analyze the page:
- **Meta tags:** title (<60 chars), description (<155 chars), viewport, robots
- **Headings:** proper H1-H6 hierarchy, single H1
- **URLs:** clean, descriptive, no query params for content pages
- **Canonical:** present and correct
- **Sitemap:** exists at /sitemap.xml
- **Robots.txt:** exists, not blocking important pages
- **HTTPS:** enforced, no mixed content
- **Mobile:** responsive meta tag, no horizontal scroll

## Step 2: Performance Impact
- Image optimization: format (WebP/AVIF), size, lazy loading, alt text
- Core Web Vitals indicators:
  - LCP: largest element load time
  - CLS: layout shift from images/fonts without dimensions
  - FID/INP: heavy JS blocking interaction
- Bundle size check if applicable

## Step 3: Content SEO
- Keyword presence in: title, H1, first paragraph, URL
- Content length (>300 words for ranking)
- Internal links (to other pages on same domain)
- External links (to authoritative sources)
- Structured data (JSON-LD): Article, Product, FAQ, etc.
- Open Graph + Twitter Card meta tags

## Step 4: Issues Report
Format as prioritized list:
- **Critical:** blocks indexing or ranking (missing title, noindex, broken canonical)
- **Important:** significant ranking impact (no meta description, missing alt text, slow LCP)
- **Nice-to-have:** minor improvements (schema markup, additional links)

Each issue: what's wrong, where, how to fix, impact level.

## Step 5: Action Items
- Generate fix list ordered by impact
- For code changes: specific file + line + suggested edit
