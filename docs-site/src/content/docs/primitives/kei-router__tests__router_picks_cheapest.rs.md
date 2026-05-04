---
title: router_picks_cheapest.rs
path: kei-router/tests/router_picks_cheapest.rs
dna_hash: sha256:ba1c30b117e87764
language: rust
size_loc: 86
generated: by-keidocs
---

# kei-router/tests/router_picks_cheapest.rs

Router cost-based selection test.

Given (in=10K, out=2K) the cheapest registered provider should be picked.
Costs (cents per MTok in/out):
anthropic = 100 / 500
openai    = 15 / 60
kimi      = 60 / 250
For (10_000 in, 2_000 out):
anthropic = ceil((100*10_000 + 500*2_000) / 1_000_000) = ceil(2_000_000/1M) = 2c
openai    = ceil((15*10_000 + 60*2_000) / 1_000_000)  = ceil(270_000/1M)   = 1c
kimi      = ceil((60*10_000 + 250*2_000) / 1_000_000) = ceil(1_100_000/1M) = 2c
→ openai wins.

## Related

- parent: `kei-router/tests`
- imports: kei_router

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
