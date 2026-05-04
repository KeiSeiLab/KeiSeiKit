---
title: skill
path: rag-pipeline/skill.md
dna_hash: sha256:b0a212c3e3a505ed
language: markdown
size_loc: 239
generated: by-keidocs
---

# rag-pipeline/skill.md

## Public API

- `RAG Pipeline Skill` — <!-- migrated from skills/rag-pipeline/skill.md (lowercase legacy filename) on 2026-05-02 -->
- `When to use` — - Building a RAG system: embedding pipeline, vector store ingestion, semantic or hybrid search over documents.
- `Architecture` — ```
- `Tier Selection` — | Tier | Embedding | Vector DB | Cost | Use Case |
- `Step 1: Init — Choose Stack` — ### Default: LanceDB + OpenAI (zero infrastructure)
- `Step 2: Ingest — Document Processing` — ### PDF Parsing [E2]
- `Step 3: Embed & Store` — ### Embedding
- `Step 4: Search` — ### Dense Search (cosine similarity)
- `Cost Calculator` — For 1000 documents (~500 pages, ~0.4M tokens):
- `Upgrade Paths` — - **Minimal → Production:** Add hybrid search (BM25 + vector), add reranking

## Related

- parent: `rag-pipeline`

## Discussion

<script src="https://giscus.app/client.js"
        data-repo="KeiSei84/KeiSeiKit-1.0"
        data-repo-id="PLACEHOLDER_REPO_ID"
        data-category="wiki-comments"
        data-category-id="PLACEHOLDER_CATEGORY_ID"
        data-mapping="pathname"
        data-strict="0"
        data-reactions-enabled="1"
        data-emit-metadata="0"
        data-input-position="bottom"
        data-theme="preferred_color_scheme"
        data-lang="en"
        data-loading="lazy"
        crossorigin="anonymous"
        async></script>
