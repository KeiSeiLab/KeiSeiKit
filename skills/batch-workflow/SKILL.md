---
name: batch-workflow
description: Use when running multi-skill pipelines — new-feature, marketing-launch, design-to-code, web-creation, full-audit, rag-setup workflows
arguments:
  - name: workflow
    description: "Workflow name: new-feature, marketing-launch, design-to-code, web-creation, full-audit, rag-setup"
    required: true
  - name: context
    description: Additional context for the workflow
    required: false
---

# Batch Workflow — Multi-Skill Pipelines

## When to use

- Running a multi-step workflow that chains several skills together (new-feature, marketing-launch, design-to-code, web-creation, full-audit, rag-setup).
- Kicking off a named workflow pipeline rather than invoking individual skills manually.
- Orchestrating sequential tasks that each depend on the output of the previous step.

## Available Workflows

### new-feature
Full feature development pipeline:
1. `/brainstorming` — explore requirements and design
2. `/test-gen` — write tests for the feature (TDD)
3. Implementation (manual or via `/quick-api` for API features)
4. `/refactor` — if code needs restructuring
5. `/pr-review` — self-review before committing
6. `doc-writer` agent — update docs

### marketing-launch
Product launch content pipeline:
1. `/competitor-analysis` — understand the landscape
2. `/landing-page` — create product landing page
3. `/content-pipeline` — write launch blog post
4. `/social-post platform=all` — create social media posts
5. `/email-sequence type=launch` — create launch email sequence
6. `/seo-audit` — audit the landing page

### design-to-code
Design implementation pipeline:
1. `/figma-to-code` — convert Figma design to code
2. `/design-system` — extract/create tokens if needed
3. `/ui-component` — build reusable components
4. `/responsive-audit` — verify responsiveness

### web-creation
Elite website pipeline (7-level constructor):
1. Brief & goal definition (manual — product, audience, goal, tone)
2. `/competitor-analysis` — market positioning, 3-5 competitors
3. `/design-inspiration` — reference board from awwwards/godly/dribbble/21st.dev
4. `/site-teardown` — deconstruct best reference into recipe + tokens
5. `/frontend-design` — design direction from tokens + archetype
6. `/design-system` — create design system from tokens
7. `/landing-page recipe=<type>` — implement page with recipe
8. Animation skills as needed (`/scroll-animation`, `/motion-design`, `/web-effects`)
9. `/web-assets pipeline` — optimize all images, fonts, video
10. `/a11y-audit scan` — WCAG 2.2 AA compliance
11. `/seo-audit` — meta, schema, OG tags
12. `/responsive-audit` — 6 breakpoints
13. `/perf-audit` — Lighthouse >90
14. `/web-deploy` — deploy to Cloudflare Pages

### rag-setup
RAG knowledge base setup pipeline:
1. `/rag-pipeline init` — choose embedding provider + vector DB
2. Document ingestion — PDF/text/image processing
3. `/rag-pipeline ingest` — chunk, embed, store
4. `/rag-pipeline search` — test retrieval quality
5. Integration — connect to app (tool_use or context injection)

### full-audit
Comprehensive project audit:
1. `/perf-audit target=full` — performance check
2. `/seo-audit` — SEO check (if web project)
3. `/responsive-audit` — responsive check (if web project)
4. `/a11y-audit scan` — accessibility check (if web project)
5. `auditor` agent — Constructor Pattern audit

## Execution
- Present the workflow steps to user BEFORE starting
- Execute skills sequentially, passing context between them
- After each skill: report results, ask if user wants to continue or skip
- Track progress in TODO tasks
