# EVIDENCE GRADING

Every major claim must carry a grade:

| Grade | Name | Criteria |
|-------|------|----------|
| **E1** | Fact | Confirmed in production OR primary source (official docs, API response, pricing page) |
| **E2** | Verified | Reproducible in tests/benchmarks. Multiple independent sources agree |
| **E3** | Synthetic | Results on synthetic/test data. Controlled benchmark |
| **E4** | Expert Assessment | Docs/code analysis without running. Extrapolation. Literature consensus |
| **E5** | Hypothesis | Theoretical assumption. Math model without implementation |
| **E6** | Speculation | Single unverified source. Outdated data (>6mo) |

Rules: architectural decision → E1-E2. Financial (compute) → ONLY E1. Data >6mo without re-verification → grade −1. Single source → max E4. Own benchmark without external confirm → max E3.
