---
name: share-page
description: Use when you need to share an HTML page (visual report, diagram, presentation) via a public URL. Triggers on "share", "поделись ссылкой", "выгрузи", "задеплой", "расшарь", "share link", "deploy page", "make shareable".
---

# Share Page

Deploy self-contained HTML files to GitHub Pages and get a shareable URL.

## Usage

```
/share-page <file-path> [custom-slug]
```

**Examples:**
```
/share-page ~/.agent/diagrams/my-report.html
/share-page ~/.agent/diagrams/my-report.html meeting-research
```

## How It Works

1. Run the deploy script:
```bash
bash ~/.claude/skills/share-page/deploy.sh <file-path> [slug]
```

2. Script clones/pulls `KeiSei84/shares` repo, copies HTML, pushes to main
3. GitHub Pages serves at `https://keisei84.github.io/shares/{slug}.html`
4. Pages deploy takes 30-60 seconds after push

## After Deploying

- Tell the user the URL
- Note: GitHub Pages cache is ~10 min. Force refresh with `?v=2` query param
- To update an existing page, deploy with the same slug (overwrites)
- All pages are **public** — anyone with URL can view

## Limitations

- Only self-contained HTML (no external assets except CDN links)
- GitHub Pages has 1GB repo size limit and 100MB per-file limit
- Deploy propagation: 30-60 seconds (first deploy may take longer)
