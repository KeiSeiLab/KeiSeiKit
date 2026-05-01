# `_bridges/` — Cross-tool bridge templates

Tool-agnostic coding-rules templates, rendered into any project via `_bridges/emit.sh`. Placeholders: `{{PROJECT_NAME}}`, `{{PROJECT_DESCRIPTION}}`, `{{YEAR}}`, `{{MONTH}}`, `{{DATE}}`.

| Template | Output path |
|---|---|
| `cursorrules.tmpl` | `.cursorrules` |
| `agents-md.tmpl` | `AGENTS.md` |
| `copilot.tmpl` | `.github/copilot-instructions.md` |
| `cursor-mdc.tmpl` | `.cursor/rules/main.mdc` |
| `windsurf.tmpl` | `.windsurf/rules/main.md` |
| `junie.tmpl` | `.junie/guidelines.md` |
| `continue.tmpl` | `.continue/rules/main.md` |
| `gemini.tmpl` | `GEMINI.md` |
| `aider-conventions.tmpl` | `CONVENTIONS.md` |
| `aider-conf.tmpl` | `.aider.conf.yml` |
| `replit.tmpl` | `replit.md` |

Render: `_bridges/emit.sh <target-dir> [project-name] [project-description]`. Idempotent — existing files are skipped.
