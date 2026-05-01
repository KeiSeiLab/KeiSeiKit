# Phase 2 — Build matrix (OS × version × target)

Decide how the build fans out. Matrix minimum: OS × primary-language version. Max reasonable: 3 OS × 3 versions × 3 targets = 27 cells — beyond that CI time-to-green kills iteration speed.

## 2a — Matrix click (AskUserQuestion, multi-select across three axes)

The question encodes three axes in one screen to keep the click contract tight. Each selection is stored as a set; the cartesian product becomes `MATRIX`.

```json
{
  "questions": [
    {
      "question": "Build OS?",
      "header": "OS",
      "multiSelect": true,
      "options": [
        {"label": "ubuntu-24.04",       "description": "GH-hosted default; also available as self-hosted label on Forgejo"},
        {"label": "ubuntu-22.04",       "description": "Older glibc; pick if targeting older prod"},
        {"label": "macos-14",           "description": "Apple Silicon (M1); required for Swift/iOS/macOS builds"},
        {"label": "macos-13",           "description": "Intel macOS; x86_64 test matrix on Apple software"},
        {"label": "windows-2022",       "description": "Only when a .exe / MSVC artefact is shipped"},
        {"label": "self-hosted (Forgejo runner)", "description": "Labels from ci-forgejo-actions.md: self-hosted,linux,x64,docker"}
      ]
    },
    {
      "question": "Language / toolchain versions (picks combine with every OS)?",
      "header": "Versions",
      "multiSelect": true,
      "options": [
        {"label": "Rust stable + MSRV 1.80",    "description": "rust: [stable, 1.80] — MSRV pin catches accidental newer-feature use"},
        {"label": "Node 20 LTS + 22 LTS",       "description": "node-version: [20, 22] — covers current + upcoming LTS"},
        {"label": "Python 3.11 + 3.12 + 3.13",  "description": "python-version: ['3.11','3.12','3.13'] — matches supported pip-audit range"},
        {"label": "Go 1.22 + 1.23",             "description": "go-version: ['1.22','1.23'] — current + previous minor"},
        {"label": "Flutter stable",             "description": "Single version; pin via flutter-version-file"},
        {"label": "Swift 6.0 (Xcode 16)",       "description": "xcode-select on macos-14; one version per OS cell"},
        {"label": "Single version only",        "description": "Matrix collapses on the version axis; OS axis still fans out"}
      ]
    },
    {
      "question": "Cross-compile targets (Rust / Go only; skip otherwise)?",
      "header": "Targets",
      "multiSelect": true,
      "options": [
        {"label": "host (no cross)",           "description": "Default; one per OS"},
        {"label": "aarch64-unknown-linux-gnu", "description": "ARM64 server deploy; use cross or native ARM runner"},
        {"label": "wasm32-unknown-unknown",    "description": "Browser / edge Worker target"},
        {"label": "x86_64-pc-windows-gnu",     "description": "MinGW Windows from Linux build"},
        {"label": "aarch64-apple-darwin",      "description": "Apple Silicon; native on macos-14, cross on ubuntu"}
      ]
    }
  ]
}
```

Store the three sets as `MATRIX.os`, `MATRIX.versions`, `MATRIX.targets`. The scaffold uses the cartesian product.

## 2b — Sanity check (no AskUserQuestion)

Compute `N = |os| × |versions| × |targets|`. Print inline:

```
Matrix cells: N (os=<...>) × (versions=<...>) × (targets=<...>)
Estimated runtime: ~<N × typical-cell-minutes> minutes wall-clock (parallel)
```

If `N > 12`, warn once: "Matrix is wide. Consider dropping one axis or using `strategy.fail-fast: true` for PR-time feedback. Continue?"

If `MATRIX.os` includes both `macos-*` and `MATRIX.targets` includes only `host`, collapse the targets axis silently — host-on-mac is the only useful combination.

## 2c — Fail-fast click inferred (inline, no extra AskUserQuestion)

- PR matrix: `fail-fast: false` (user wants to see ALL failing cells at once).
- Scheduled cron + release matrix: `fail-fast: true` (first failure is enough to trigger remediation).

Emitted in Phase 3 `ci.yml` / `release.yml` accordingly.

## Verify-criterion

- `MATRIX.os` has ≥1 entry.
- `MATRIX.versions` has ≥1 entry (or "Single version only").
- `MATRIX.targets` has ≥1 entry (defaults to `host`).
- `N ≤ 27` or explicit user override recorded in the final report.
- If `LANGS = {Swift}` then `MATRIX.os` MUST include a `macos-*` entry (fail-closed otherwise).
