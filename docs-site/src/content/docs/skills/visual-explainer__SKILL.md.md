---
title: SKILL
path: visual-explainer/SKILL.md
dna_hash: sha256:80c5e65c8a0e919e
language: markdown
size_loc: 472
generated: by-keidocs
---

# visual-explainer/SKILL.md

## Public API

- `Visual Explainer` — ---
- `Available Commands` — Detailed prompt templates in `./commands/`. In Pi, these are slash commands (`/diff-review`). In Claude Code, namespaced (`/visual-explainer:diff-review`). In Codex, use `/prompts:diff-review` (if installed to `~/.codex/prompts/`) or invoke `$visual-explainer` and describe the workflow.
- `Workflow` — ### 1. Think (5 seconds, not 5 minutes)
- `Generate to a temp file (use --aspect-ratio for control)` — surf gemini "descriptive prompt" --generate-image /tmp/ve-img.png --aspect-ratio 16:9
- `Base64 encode for self-containment (macOS)` — IMG=$(base64 -i /tmp/ve-img.png)
- `Linux: IMG=$(base64 -w 0 /tmp/ve-img.png)` — 
- `Embed in HTML and clean up` — 
- `<img src="data:image/png;base64,${IMG}" alt="descriptive alt text">` — rm /tmp/ve-img.png
- `Diagram Types` — ### Architecture / System Diagrams
- `Slide Deck Mode` — An alternative output format for presenting content as a magazine-quality slide presentation instead of a scrollable page. **Opt-in only** — the agent generates slides when the user invokes `/generate-slides`, passes `--slides` to an existing prompt (e.g., `/diff-review --slides`), or explicitly asks for a slide deck. Never auto-select slide format.
- `File Structure` — Every diagram is a single self-contained `.html` file. No external assets except CDN links (fonts, optional libraries). Structure:
- `Sharing Pages` — Share visual explainer pages instantly via Vercel. No account or authentication required.
- `Output:` — 
- `✓ Shared successfully!` — 
- `Live URL:  https://skill-deploy-abc123.vercel.app` — 
- `Claim URL: https://vercel.com/claim-deployment?code=...` — ```
- `Quality Checks` — Before delivering, verify:
- `Anti-Patterns (AI Slop)` — These patterns are explicitly forbidden. They signal "AI-generated template" and undermine the skill's purpose of producing distinctive, high-quality diagrams. Review every generated page against this list.

## Related

- parent: `visual-explainer`

## Discussion

<script src="https://giscus.app/client.js"
        data-repo="KeiSei84/KeiSeiKit-1.0"
        data-repo-id="PLACEHOLDER_REPO_ID"
        data-category="wiki-comments"
        data-category-id="PLACEHOLDER_CATEGORY_ID"
        data-mapping="pathname"
        data-strict="0"
        data-reactions-enabled="1"
        data-emit-metadata="0"
        data-input-position="bottom"
        data-theme="preferred_color_scheme"
        data-lang="en"
        data-loading="lazy"
        crossorigin="anonymous"
        async></script>
