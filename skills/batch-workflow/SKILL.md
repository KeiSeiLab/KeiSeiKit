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
2. `/content-pipeline` — write launch blog post
3. `/social-post platform=all` — create social media posts
4. `/email-sequence type=launch` — create launch email sequence

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
2. `auditor` agent — Constructor Pattern audit

## Execution
- Present the workflow steps to user BEFORE starting
- Execute skills sequentially, passing context between them
- After each skill: report results, ask if user wants to continue or skip
- Track progress in TODO tasks
