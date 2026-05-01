---
name: content-pipeline
description: Use when creating content — research, outline, draft, SEO, format for platform, social promo
arguments:
  - name: topic
    description: Content topic or title
    required: true
  - name: platform
    description: "Target platform: blog, medium, dev.to, docs"
    required: false
  - name: audience
    description: Target audience
    required: false
---

# Content Pipeline Workflow

## Step 1: Research
- WebSearch for existing content on the topic (top 5 results)
- Identify gaps — what's NOT covered well?
- Find data, statistics, examples to reference
- Check if project has existing content to build on

## Step 2: Outline
Structure (adapt to content type):
- Hook — why should reader care? (1-2 sentences)
- Context — background needed to understand
- Main sections (3-5, each with a clear takeaway)
- Code examples / visuals where applicable
- Conclusion with actionable next steps

Present outline to user for approval before drafting.

## Step 3: Draft
- Write in the voice/tone matching the brand
- Technical content: accurate, code-tested, no hand-waving
- Marketing content: benefit-driven, concise, scannable
- Include code blocks with language tags
- Add internal links to other project content if available

## Step 4: SEO Optimization
- Target keyword in title, H1, first paragraph
- Related keywords in H2s and body
- Meta description: <155 chars, compelling
- Internal and external links (2-3 each)
- Image alt text

## Step 5: Platform Formatting
- **Blog/Next.js:** MDX with frontmatter (title, date, tags, description)
- **Medium:** clean markdown, no custom components
- **dev.to:** frontmatter (title, published, tags, cover_image)
- **Docs:** follow existing docs structure and style

## Step 6: Social Promotion
- Generate 2-3 social snippets (for social-post skill)
- Key quote or stat for sharing
- Thread-worthy breakdown if content is long
