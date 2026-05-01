# STACK — Python ML (PyTorch / JAX)

Python is acceptable here because ML training > ~10M params is still the dominant ecosystem. Inference should still be Rust/C++/ONNX where possible.

**Core:** PyTorch ≥ 2.0 (compile, FlashAttn 2). `pyproject.toml` only — NO `setup.py`, NO `requirements.txt` as source of truth (lock via `uv lock` or `pip-compile`).

**Tooling:**
- `ruff` format + lint (replaces black / isort / flake8)
- `mypy --strict` on library modules; relaxed on training scripts
- `pytest` + `pytest-asyncio` for tests; synthetic-data smoke test that runs in < 5 s

**Observability (non-negotiable — a silent long run with no output is a real incident we've hit):**
- `print(..., flush=True)` on EVERY print in any script > 2 min wall-time.
- Progress every 250 steps OR every 30 s wall-time, whichever first.
- Launch via `python3 -u` or `PYTHONUNBUFFERED=1`.
- Format: `[env/topo/seed] ep N: last100=X.X, time=Ys`.

**Reproducibility:**
- Seeds fixed: `torch.manual_seed(seed)`, `np.random.seed(seed)`, `random.seed(seed)`. Default `[42, 137, 256]` for multi-seed runs.
- Log ALL hyperparams at run start — exact param count (not "~7M"), batch, LR, seq-len, dataset hash.

**Training on Modal:** see `deploy-modal.md`. `flush=True`, `vol.commit()` after each write, checkpoints every 500 steps, `.spawn()` not `.map()`, `retries=modal.Retries(max_retries=1)`, anti-stop guard (never stop a running job without explicit user confirmation).

**Results logging:** after EVERY run record in `memory/{project}.md` — architecture, dims, params (EXACT), data, steps, metric, time, hardware, status, cost, notes. DATA FIRST, analysis second.

**Forbidden:** `print()` without `flush=True`; "~7M" instead of exact param count; skipping result logging; LR schedule tuning before ablating what's unnecessary (Math-First); single-seed claims for anything that will be published or cited (need ≥ 5 seeds).
