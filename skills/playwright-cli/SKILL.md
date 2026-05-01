---
name: playwright-cli
description: Browser automation via Playwright CLI — UI testing, form filling, screenshots, scraping. Token-efficient alternative to MCP. Use when user asks to test UI, automate browser, take screenshots, fill forms, or scrape web pages.
---

# Browser Automation with playwright-cli

## Overview

`playwright-cli` is a token-efficient CLI tool for browser automation. It saves the accessibility tree to disk and only passes a summary to the agent — ~90K tokens cheaper than MCP per task.

**Run via:** `npx @playwright/cli <command>` (installed globally would be just `playwright-cli <command>`)

## Key Workflow Pattern

```
1. npx @playwright/cli open [url]     — launch browser (headless by default)
2. npx @playwright/cli snapshot       — get accessibility tree, identify element refs
3. npx @playwright/cli click <ref>    — interact using refs from snapshot
4. npx @playwright/cli screenshot     — capture result
5. npx @playwright/cli close          — cleanup
```

**For headed (visible) browser:** `npx @playwright/cli open --headed [url]`

## Essential Commands

### Opening & Navigation
- `open [url]` — launch browser (add `--headed` to see it)
- `goto <url>` — navigate to URL
- `go-back` / `go-forward` — browser history
- `close` — close browser session

### Page Interaction (use refs from `snapshot`)
- `click <ref>` — click element
- `dblclick <ref>` — double click
- `fill <ref> <text>` — fill input field
- `type <text>` — type text into focused element
- `select <ref> <value>` — select dropdown option
- `check <ref>` / `uncheck <ref>` — checkboxes
- `hover <ref>` — hover over element
- `drag <startRef> <endRef>` — drag and drop
- `upload <file>` — file upload

### Inspection
- `snapshot` — capture page accessibility tree, get element refs
- `screenshot` — take screenshot (saved to disk)
- `pdf` — export page as PDF

### Keyboard & Mouse
- `press <key>` — press key (Enter, Tab, etc.)
- `keydown <key>` / `keyup <key>` — key press/release
- `mousemove <x> <y>` / `mousedown` / `mouseup` / `mousewheel`

### Storage & Cookies
- `cookie-list` / `cookie-get <name>` / `cookie-set` / `cookie-delete`
- `localstorage-list` / `localstorage-get` / `localstorage-set` / `localstorage-delete`
- `sessionstorage-list` / `sessionstorage-get` / `sessionstorage-set`

### Network
- `route <pattern>` — mock network requests
- `route-list` — list active mocks
- `unroute [pattern]` — remove mocks
- `network` — list all network requests

### DevTools & Recording
- `console [min-level]` — list console messages
- `eval <func> [ref]` — evaluate JS on page or element
- `run-code <code>` — run Playwright code snippet
- `tracing-start` / `tracing-stop` — trace recording
- `video-start` / `video-stop` — video recording

### Session Management
- `list` — list browser sessions
- `close-all` — close all sessions
- `kill-all` — force kill stale sessions
- Use `-s=<session>` flag to work with named sessions (parallel testing)

### Browser Options
- Chrome (default), Firefox, WebKit, Edge
- `--browser=firefox` to use different browser
- Persistent profiles supported

## Parallel UI Testing Pattern

For parallel sub-agent testing (recommended for form/UI validation):

```
Prompt to Claude Code:
"Use playwright-cli to test [feature]. Run 3 parallel sub-agents:
1. Happy path — valid inputs, expected flow
2. Edge cases — empty fields, special characters, very long input
3. Validation — invalid email, missing required fields, XSS attempts

Use headed browsers so I can see. Take screenshots of results."
```

Each sub-agent gets its own session via `-s=<name>` flag.

## Tips

- **Default is headless** — add `--headed` to `open` command to see the browser
- **Always `snapshot` first** — to get element refs before interacting
- **Sessions for parallelism** — use `-s=agent1`, `-s=agent2` etc. for parallel agents
- **Screenshots for evidence** — always take screenshots of test results
- **Close sessions** — always `close` or `close-all` when done
- **Token efficiency** — CLI saves accessibility tree to disk, only passes summary to agent
