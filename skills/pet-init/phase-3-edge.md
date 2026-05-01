# Phase 3 — Edge

Gather the three `[edge]` fields (`directness`, `initiative`, `profanity`)
plus the optional `[forbidden].topics` list. Click-driven for the enums,
one short free-text for forbidden topics.

## 3a — Edge batch (AskUserQuestion, 1 batch with 3 questions)

Emit a single `AskUserQuestion` call:

```json
{
  "questions": [
    {
      "question": "How direct should the pet be?",
      "header": "Directness",
      "multiSelect": false,
      "options": [
        {"label": "Gentle",    "description": "Soft-edge, wraps corrections in padding, never pushy"},
        {"label": "Balanced",  "description": "Honest but kind, states disagreement politely"},
        {"label": "Direct",    "description": "Minimal padding, tells you the thing"},
        {"label": "Blunt",     "description": "No padding, named-flaw feedback, warrior mode"}
      ]
    },
    {
      "question": "How proactive should the pet be?",
      "header": "Initiative",
      "multiSelect": false,
      "options": [
        {"label": "Wait",        "description": "Only speaks when you ask"},
        {"label": "Nudge",       "description": "Occasionally flags something that might matter"},
        {"label": "Proactive",   "description": "Will surface patterns, issues, or ideas unprompted"}
      ]
    },
    {
      "question": "Profanity policy?",
      "header": "Profanity",
      "multiSelect": false,
      "options": [
        {"label": "Never",        "description": "Pet never uses profanity, regardless of your style"},
        {"label": "Rare",         "description": "Occasional mild profanity when the moment fits"},
        {"label": "Contextual",   "description": "Mirrors your own register — matches if you swear"}
      ]
    }
  ]
}
```

## 3b — Map clicks to variables

`DIRECTNESS` — lowercase the chosen label:

| Label      | Value       |
|------------|-------------|
| Gentle     | `gentle`    |
| Balanced   | `balanced`  |
| Direct     | `direct`    |
| Blunt      | `blunt`     |

`INITIATIVE` — lowercase:

| Label       | Value        |
|-------------|--------------|
| Wait        | `wait`       |
| Nudge       | `nudge`      |
| Proactive   | `proactive`  |

`PROFANITY` — lowercase:

| Label        | Value          |
|--------------|----------------|
| Never        | `never`        |
| Rare         | `rare`         |
| Contextual   | `contextual`   |

## 3c — Forbidden topics (free text, optional)

Emit a regular message (NOT AskUserQuestion):

> Any topics the pet should refuse to engage on?
> - comma-separated list
> - examples: `medical-advice, legal-advice, stock-picks`
> - leave blank and press enter to skip
>
> Reply on one line.

Parse the reply:

- trim whitespace
- split on comma
- trim each entry, drop empties
- lowercase + kebab-case each entry (`Medical Advice` → `medical-advice`)
- deduplicate while preserving order
- cap at 20 entries (if more, keep first 20 and tell the user)

Capture the result as `FORBIDDEN_TOPICS`. Empty reply → `[]` (schema-valid).

## 3d — Consistency check (soft)

If `DIRECTNESS == "blunt"` and `PROFANITY == "never"`, emit a regular
message (informational, no re-ask):

> Note: "blunt" directness with "never" profanity is valid — the pet will
> use strong language-free bluntness. Continuing.

No branch, no AskUserQuestion — this is just a heads-up so the user knows
the combination is deliberate, not a bug.

## Verify-criterion

- `DIRECTNESS` is one of `gentle` / `balanced` / `direct` / `blunt`
- `INITIATIVE` is one of `wait` / `nudge` / `proactive`
- `PROFANITY` is one of `never` / `rare` / `contextual`
- `FORBIDDEN_TOPICS` is a list (possibly empty) of kebab-case strings,
  length ≤ 20, no duplicates

## Failure modes (constructive paths)

If the user seems confused by the Directness scale (asks "what does blunt
mean?"):
- (A) give a one-line example for each level, then re-emit the batch
- (B) default to `balanced` (the safest middle), confirm with user
- (C) move on with their best guess and remind them they can re-run
  `/pet-init` any time

If the forbidden-topics free text contains something that looks like a
secret (matches the `secrets-guard` detector patterns — `sk-`, `ghp_`,
etc.), STOP:
- do NOT store the reply
- emit: `That looked like a credential token, not a topic. Re-enter topics only — no API keys or passwords.`
- re-ask once; if it repeats, skip forbidden-topics with `[]`
