---
name: rag-pipeline
description: Use when building RAG (Retrieval-Augmented Generation) systems — embedding pipeline, vector database, document ingestion, semantic search, hybrid search. Triggers on "RAG", "embeddings", "vector search", "semantic search", "document ingestion", "knowledge base".
arguments:
  - name: command
    description: "Command: init, ingest, search, upgrade"
    required: false
  - name: provider
    description: "Embedding provider: openai, gemini, voyage, cohere, local (default: openai)"
    required: false
---

# RAG Pipeline Skill

Build retrieval-augmented generation systems with swappable components.

## Architecture

```
Documents → Ingestion → Chunking → Embedding → Vector DB
                                                    ↓
Query → Embed Query → Hybrid Search (dense + BM25) → Rerank → LLM Context
```

## Tier Selection

| Tier | Embedding | Vector DB | Cost | Use Case |
|------|-----------|-----------|------|----------|
| **Minimal** | OpenAI small ($0.02/MTok) | LanceDB (embedded) | ~$0 | Prototyping, offline |
| **Production** | Voyage-4 or OpenAI large | LanceDB hybrid / Qdrant | Low | Most projects |
| **Multimodal** | Gemini Embedding 2 ($0.20/MTok) | LanceDB / Pinecone | Medium | Text + images + video |

## Step 1: Init — Choose Stack

### Default: LanceDB + OpenAI (zero infrastructure)

```bash
npm install lancedb @lancedb/vectordb openai
```

LanceDB: embedded (no server), Apache Arrow, hybrid search via RRF, scales to billions, Node.js + Python native. Free forever [E1].

### Embedding Providers [E1]

| Provider | Model | $/MTok | Dims | Context | Multimodal |
|----------|-------|--------|------|---------|------------|
| OpenAI | text-embedding-3-small | $0.02 | 1536 | 8K | No |
| OpenAI | text-embedding-3-large | $0.13 | 3072 | 8K | No |
| Gemini | Embedding 2 | $0.20 | 3072 | 8K | Text+Image+Video+Audio |
| Voyage | voyage-3.5 | $0.06 | flex | 32K | No |
| Cohere | Embed 4 | $0.12 | 1536 | 128K | Text+Image |
| Local | nomic-embed-text-v2-moe | FREE | 768 | 8K | No |

**Decision:** OpenAI small for text-only (cheapest quality). Gemini 2 for multimodal (only unified embedding space). Voyage for domain-specific (code/law/finance). Local nomic for privacy/offline.

### Vector DB Comparison [E1]

| DB | Type | Free Tier | Hybrid Search | Setup |
|----|------|-----------|---------------|-------|
| **LanceDB** | Embedded | Unlimited (OSS) | Yes (RRF) | `npm install` |
| ChromaDB | Embedded | Unlimited (OSS) | Yes (BM25) | `pip install` |
| Pinecone | Cloud | 2GB, 2M writes/mo | Yes | API key |
| Qdrant | Cloud+self | 1GB RAM free | Yes | Docker or API |

**Default: LanceDB** — zero ops, no server, embedded, free.

## Step 2: Ingest — Document Processing

### PDF Parsing [E2]

**Python (best quality):**
```bash
pip install pymupdf4llm  # PyMuPDF with LLM-optimized markdown output
```
```python
import pymupdf4llm
md_text = pymupdf4llm.to_markdown("document.pdf")
```

**Node.js:**
```bash
npm install pdf-parse  # basic text extraction
```

For complex PDFs with tables/images: use LlamaParse API or call PyMuPDF via subprocess.

### Chunking Strategy [E2]

**Default: Recursive character splitting (512 tokens, 50 overlap)**

```typescript
function chunkText(text: string, maxTokens = 512, overlap = 50): string[] {
  const separators = ['\n\n', '\n', '. ', ' '];
  const chunks: string[] = [];
  let remaining = text;

  for (const sep of separators) {
    if (remaining.length <= maxTokens * 4) break; // ~4 chars/token
    const parts = remaining.split(sep);
    let current = '';
    for (const part of parts) {
      if ((current + sep + part).length > maxTokens * 4) {
        if (current) chunks.push(current.trim());
        current = part;
      } else {
        current = current ? current + sep + part : part;
      }
    }
    remaining = current;
  }
  if (remaining.trim()) chunks.push(remaining.trim());
  return chunks;
}
```

**Advanced (production):**
- Semantic chunking: split on topic boundaries (+70% accuracy vs fixed) [E2]
- Contextual retrieval: prepend document context to each chunk (-69% error rate with hybrid) [E2]
- Hierarchical: paragraph + section level chunks for multi-granularity retrieval

## Step 3: Embed & Store

### Embedding

```typescript
import OpenAI from 'openai';
const openai = new OpenAI();

async function embed(texts: string[]): Promise<number[][]> {
  const res = await openai.embeddings.create({
    model: 'text-embedding-3-small',
    input: texts,
  });
  return res.data.map(d => d.embedding);
}
```

### Gemini Multimodal (images + video + audio in same space)

```typescript
import { GoogleGenAI } from '@google/genai';
const ai = new GoogleGenAI({ apiKey: process.env.GEMINI_API_KEY });

const result = await ai.models.embedContent({
  model: 'gemini-embedding-exp-03-07',
  contents: [{ parts: [{ text: 'query' }] }],
  config: { taskType: 'RETRIEVAL_DOCUMENT', outputDimensionality: 768 },
});
```

### Store in LanceDB

```typescript
import lancedb from 'lancedb';

const db = await lancedb.connect('./vectors');
const table = await db.createTable('docs', [
  { id: '1', text: 'chunk text', vector: embedding, source: 'file.pdf', page: 1 },
]);
```

## Step 4: Search

### Dense Search (cosine similarity)

```typescript
const results = await table.search(queryEmbedding).limit(5).toArray();
```

### Hybrid Search (dense + BM25 via RRF) [E2]

```typescript
const results = await table
  .search(queryEmbedding, 'vector')   // dense
  .search('keyword query', 'text')     // full-text BM25
  .rerank('rrf')                       // Reciprocal Rank Fusion
  .limit(5)
  .toArray();
```

Hybrid search reduces error rate ~69% vs dense-only when combined with contextual retrieval [E2].

### Vercel AI SDK Pattern

```typescript
import { embed, cosineSimilarity } from 'ai';

const { embedding } = await embed({
  model: openai.embedding('text-embedding-3-small'),
  value: query,
});

const results = chunks
  .map(c => ({ ...c, score: cosineSimilarity(embedding, c.embedding) }))
  .sort((a, b) => b.score - a.score)
  .slice(0, 5);
```

### Claude Tool-Based Retrieval

```typescript
const tools = [{
  name: 'search_documents',
  description: 'Search the knowledge base for relevant information',
  input_schema: {
    type: 'object',
    properties: {
      query: { type: 'string', description: 'Search query' },
      limit: { type: 'number', description: 'Max results (default 5)' },
    },
    required: ['query'],
  },
}];
// Claude decides when to search. Backend queries vector DB, returns as tool result.
```

## Cost Calculator

For 1000 documents (~500 pages, ~0.4M tokens):

| Component | OpenAI small | Gemini 2 | Local |
|-----------|-------------|----------|-------|
| Embedding | $0.008 | $0.080 | $0 |
| Storage (LanceDB) | $0 | $0 | $0 |
| Per query embed | $0.000002 | $0.00002 | $0 |
| **LLM call dominates query cost** | ~$0.003-0.015 per query |

## Upgrade Paths

- **Minimal → Production:** Add hybrid search (BM25 + vector), add reranking
- **Production → Multimodal:** Switch to Gemini Embedding 2, add image/video ingestion
- **Embedded → Cloud:** Swap LanceDB for Qdrant Cloud or Pinecone (API-compatible)
