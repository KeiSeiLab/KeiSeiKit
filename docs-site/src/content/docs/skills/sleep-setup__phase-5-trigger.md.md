---
title: phase-5-trigger
path: sleep-setup/phase-5-trigger.md
dna_hash: sha256:4590d5df35330dc2
language: markdown
size_loc: 175
generated: by-keidocs
---

# sleep-setup/phase-5-trigger.md

## Public API

- `Phase 5 — Emit trigger (CronCreate and/or `/schedule create`)` — Render the ready-to-paste nightly trigger(s) and ask how to register
- `5a — Load template (remote path only)` — If `SLEEP_MODE ∈ {remote-only, hybrid}`, read the cloud-agent template
- `5b — Parse local time` — `SLEEP_TIME_LOCAL` has format `HH:MM` (validated in Phase 0b). Convert
- `5c — Compute UTC cron (remote path only)` — Only needed if `SLEEP_MODE ∈ {remote-only, hybrid}`. CronCreate on the
- `macOS / GNU date — detect local TZ offset in minutes` — offset_min=$(date +%z | awk '{ s=substr($0,1,1); h=substr($0,2,2); m=substr($0,4,2); print (s=="-" ? 1 : -1) * (h*60+m) }')
- `5d — Render blocks per mode` — ### Mode: `local-only`
- `5e — Render placeholders (remote path only)` — For `remote-only` / `hybrid`: replace `{REPO_URL}` and `{UTC_CRON}` in
- `5f — Fallback inline template (remote path, if kit missing file)` — If `~/.claude/agents/_primitives/templates/sleep-trigger-prompt.md` is
- `Verify-criterion` — - `local-only`: exactly ONE `AskUserQuestion`; exactly ONE fenced

## Related

- parent: `sleep-setup`

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
