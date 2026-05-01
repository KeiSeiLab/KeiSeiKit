# DEPLOY — Modal (GPU compute)

A real cost-overrun incident (tens of dollars lost to unchecked runs) and a real KILL-GUARD incident (over an hour of training killed for a non-critical bug) shape every rule below.

**Pre-launch 10-step checklist (all ticks before `modal run`):**
1. `modal app list` — verify no collisions/duplicates
2. GPU compat: A10G torch ≥ 2.0 (~$1.10/hr), H100 torch ≥ 2.1 (~$4.50/hr), B200 torch ≥ 2.6 (~$8/hr)
3. `cat` the script — confirm file edits actually landed
4. Cost estimate in dollars, verified on live https://modal.com/pricing (NOT from memory)
5. Volume + `vol.commit()` after each write
6. Checkpoints every 500 steps saving `state_dict` (not just JSON metrics)
7. `retries=modal.Retries(max_retries=1)` minimum
8. `.spawn()` for batches — NEVER `.map()` (cascade-kill on single failure)
9. `flush=True` on every print; progress every 250 steps
10. Single-variant smoke run BEFORE fanning out to N variants

**Cost tiers:** AUTO < $5 · WARN $5-$20 (daily cap $20) · STOP > $20 (explicit user "yes, launch").

**anti-stop guard (no exception):**
- NEVER `modal app stop`, `modal app kill`, `kill <pid>`, `pkill -f modal` without literal user phrase "yes, stop it".
- Before any stop: `modal app list` → show user what is running, how long in, how much remaining, current checkpoint state.
- A bug in the launching script is NOT a reason to kill a running training run.

**Volume persistence:** results survive only inside `modal.Volume` with explicit `vol.commit()`. Stdout is ephemeral — checkpoints in volume, metrics in volume, logs to volume.

**Forbidden:** guessed prices from memory; `.map(return_exceptions=False)` for batches; `print()` without `flush=True`; launching N variants before one verified single-variant; restarting "for cleanliness" when checkpoints are flowing; stopping a run to fix the launching script.
