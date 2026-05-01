# MATH FIRST (mandatory for ML / physics / theory work)

1. **Expression first** — 1-3 lines LaTeX/Unicode BEFORE prose
2. **What is UNNECESSARY?** — remove before adding
   - Learned parameters? WHY? Can you do without?
   - Hyperparameters? WHY? Determined by input?
   - Activation functions? WHY? Normalize enough?
   - Separate projection matrices? WHY? Does the input already encode this?
   - Gate/gating? WHY? Normalize = implicit gate?
   - Separate decoder? WHY? Can you reuse the state directly as output?
3. **Count** — params, hyperparams, FLOPs, memory
4. **ONLY THEN** — proof / plan / code

**Prohibited:** prose before expression, "fixes" before experimental confirmation, imposing form instead of deriving from input.

**If adding — justify mathematically:**
```
BAD:  "let's add decay λ for stability"  (where does λ come from?)
GOOD: "the normalization step already contains implicit decay — verify experimentally before adding"
```
