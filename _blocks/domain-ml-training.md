# DOMAIN — ML Training

Math-First block (`rule-math-first.md`) MUST be included alongside this one.

**Pre-Experiment Check — blocking checklist (answer all before launch — each GPU run costs real money):**

1. **TOKENIZATION** — BPE / character / byte / morphological? Different tokenizations produce different units and are NOT directly comparable.
2. **ARCHITECTURE** — exact class / file / commit. No ambiguity.
3. **INIT / MATRICES** — random / structured / pretrained? Note initialization distribution and rank if relevant.
4. **TRAINING DIRECTION** — forward / reverse / mixed? State it; some models are only tested one way.
5. **METRIC** — what EXACT metric and on what EXACT data split. State units (PPL on which tokenizer, accuracy on which set).
6. **RESEARCH QUESTION** — "This run tests hypothesis: ___". Cannot formulate → DO NOT LAUNCH.
7. **PRIOR RESULTS** — check your `memory/{project}.md` + any `wrong-paths*.md` notes. Don't repeat failed configs.
8. **KNOWN BUGS** — list the known-broken configurations for the current architecture. Don't re-hit them.

**Results logging — IMMEDIATELY after every run (success / timeout / failed / NaN):**
Record in `memory/{project}.md` BEFORE analysis. Mandatory fields: Model name, Architecture, Dimensions, Key config, Params **EXACT** (never "~7M"), Data + count, Steps/Epochs, Batch/Seq, Seed, Metric, Best, Time, Hardware, Status, Cost actual, Notes.

**Multi-seed rigor (for any claim going into DECISIONS.md, a paper, or a public result):**
- Minimum **≥ 5 seeds** (3 for smoke tests). Default `[42, 137, 256, ...]`.
- Report cross-validation mean ± std, NOT single-fold cherry-pick. Single-fold cherry-picking can inflate published numbers by double-digit percentage points.
- Cache ablation table (full / zero / random / shuffled) on zero-model AND one-trained-model.

**Baseline-first discipline:** before running ANY exploration-heavy training (hill-climb, ES, PPO, RL) on a task, SEARCH for an existing published baseline (env source tree, paper README, leaderboards). If one exists — run it locally, extract trajectories, distill your model via supervised loss, THEN fine-tune. Pure exploration from scratch when a baseline exists is wasted compute.

**Forbidden:** launching without the checklist; "~N M" params; analyzing before logging; single-seed claims for anything public; class weighting when val matches train prior; cosine LR on < 50 epochs; tuning before ablating what's unnecessary.
