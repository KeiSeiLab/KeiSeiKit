---
name: perf-audit
description: Use when auditing performance — baseline, profile, identify top 3 bottlenecks, fix, remeasure
arguments:
  - name: target
    description: "What to audit: endpoint, page, function, or 'full'"
    required: true
---

# Performance Audit Workflow

## Step 1: Establish Baseline
- Measure current performance:
  - API: response time (p50, p95, p99), throughput
  - Frontend: LCP, FID, CLS, bundle size
  - Function: execution time, memory usage
- Record numbers BEFORE any changes
- Use project's existing tools or:
  - Python: `time`, `cProfile`, `memory_profiler`
  - JS/TS: `performance.now()`, Lighthouse, `webpack-bundle-analyzer`
  - API: `curl -w @-` timing, `ab`, `wrk`

## Step 2: Profile
- Identify WHERE time is spent:
  - Database queries (N+1, missing indexes, full scans)
  - Network calls (sequential vs parallel, caching)
  - CPU (algorithmic complexity, unnecessary computation)
  - Memory (leaks, large allocations, unnecessary copies)
  - I/O (file reads, disk writes)

## Step 3: Identify Top 3 Bottlenecks
- Rank by impact (% of total time)
- Focus on top 3 — don't optimize everything
- For each: document what, why slow, potential fix

## Step 4: Checkpoint
- `checkpoint: before perf-audit $target`

## Step 5: Fix (One at a Time)
- Fix #1 bottleneck → measure → confirm improvement
- Fix #2 bottleneck → measure → confirm improvement
- Fix #3 bottleneck → measure → confirm improvement
- After each fix: run tests — no regressions

## Step 6: Final Measurement
- Re-run baseline measurements
- Compare before/after
- Report: metric, before, after, improvement %

## Step 7: Commit
- `perf: optimize $target — <summary of improvements>`
