# TEST — Load / performance testing (baseline → profile → fix)

Load tests answer: "how much traffic does this system handle before SLO violation?" Not "does it work" (unit/integration) but "does it stay up under N RPS for T minutes with p99 < X ms". The loop is **baseline → profile → fix → re-baseline**, never "run once and ship".

**Tool choice (default):**
- **`k6`** (Grafana, JS scripting) — best for HTTP/REST/WS APIs with scripted scenarios + thresholds; built-in SLO assertions; Docker-friendly. [E4, k6.io]
- **`vegeta`** (Go, CLI) — simplest constant-rate HTTP attacker; great for flat-load smoke tests; pipes into plots. [E4, github.com/tsenart/vegeta]
- **`oha`** (Rust) — modern `hey` replacement, good for quick local baselines, HTTP/2 + HTTP/3. [E4, github.com/hatoof/oha]
- **`hyperfine`** (Rust) — microbenchmark CLI for single commands / binaries; NOT a web load tool. Use for build-time, cold-start, compile-speed measurements. [E4, github.com/sharkdp/hyperfine]

**SLO definition (write BEFORE running):**
1. **Latency:** p50 < A ms, p95 < B ms, p99 < C ms (p99 is the user-felt number).
2. **Throughput:** sustain N RPS for T minutes without error budget burn.
3. **Error rate:** < 0.1% 5xx, < 1% 4xx (excluding user errors).
4. **Resource:** CPU < 70%, memory < 80% of instance, no OOM kills.

Without SLOs written down, "the test passes" is meaningless.

**The loop:**
1. **Baseline:** lowest realistic load (10 RPS for 1 min). Record latency histogram, CPU, memory. This is the "no-load" floor.
2. **Ramp:** step-up load (10 → 50 → 100 → 200 RPS, 2 min each). Find the knee — where p99 doubles or errors appear.
3. **Profile at the knee:** attach `perf` / `pprof` / `tokio-console` / `flamegraph`. Identify top hot function.
4. **Fix** the hottest contributor (add index, cache, pooling, algorithm swap). ONE change at a time.
5. **Re-baseline** at the same step-up. Knee should move right. If not, the fix was wrong → revert, reprofile.

**k6 threshold example (copy into CI):**
```js
export const options = {
  stages: [{ duration: '2m', target: 100 }],
  thresholds: {
    http_req_duration: ['p(95)<500', 'p(99)<1000'],
    http_req_failed:   ['rate<0.01'],
  },
};
```
If thresholds fail, k6 exits non-zero → CI job red.

**CI integration:**
- Short smoke load test on every PR (30s, low RPS, strict thresholds). Catches obvious regressions.
- Nightly full load test on a dedicated environment, not shared prod.
- Publish HTML report (k6 cloud / Grafana) as a CI artifact.

**Forbidden:**
- Load-testing against production without a killswitch + comms.
- Running without SLOs defined in the test file itself (no "looks ok" verdicts).
- Running multiple load tests in parallel against the same target (interferes with each other).
- Changing two things between runs ("I added an index AND a cache") — can't attribute the delta.
- Ignoring CPU/memory — latency alone hides resource leaks that kill you at 24h.
