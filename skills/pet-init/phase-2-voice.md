# Phase 2 — Voice

Gather the four `[voice]` fields: `tone_primary`, `tone_secondary`,
`humor_style`, `humor_frequency`. Entirely click-driven — no free text.

## 2a — Voice batch (AskUserQuestion, 1 batch with 4 questions)

Emit a single `AskUserQuestion` call:

```json
{
  "questions": [
    {
      "question": "Primary tone of the pet?",
      "header": "Primary tone",
      "multiSelect": false,
      "options": [
        {"label": "Warm",     "description": "Friendly, supportive, caring default"},
        {"label": "Neutral",  "description": "Even-keel, factual, no emotional color"},
        {"label": "Formal",   "description": "Polite, structured, keeps a professional distance"},
        {"label": "Playful",  "description": "Light, curious, uses wordplay and side-remarks"}
      ]
    },
    {
      "question": "Secondary tones (pick up to 2, or none)?",
      "header": "Secondary tones",
      "multiSelect": true,
      "options": [
        {"label": "Warm",     "description": "Add warmth on top of primary"},
        {"label": "Neutral",  "description": "Temper intensity of primary"},
        {"label": "Formal",   "description": "Add politeness on top of primary"},
        {"label": "Playful",  "description": "Add light tangents on top of primary"},
        {"label": "Direct",   "description": "Shorter, more to-the-point"},
        {"label": "Gentle",   "description": "Softer phrasing on hard topics"}
      ]
    },
    {
      "question": "Humor style?",
      "header": "Humor",
      "multiSelect": false,
      "options": [
        {"label": "None",    "description": "No jokes, no wordplay — task-focused"},
        {"label": "Dry",     "description": "Understated, deadpan, rare smirks"},
        {"label": "Witty",   "description": "Clever, observational, occasional puns"},
        {"label": "Silly",   "description": "Absurd, playful, freely silly"}
      ]
    },
    {
      "question": "How often should humor appear?",
      "header": "Humor frequency",
      "multiSelect": false,
      "options": [
        {"label": "Rare",         "description": "Only when the moment clearly invites it"},
        {"label": "Occasional",   "description": "A few light remarks per long conversation"},
        {"label": "Frequent",     "description": "Frequent jokes, side-remarks, playful asides"}
      ]
    }
  ]
}
```

## 2b — Map clicks to variables

`TONE_PRIMARY` — lowercase the chosen label:

| Label    | Value       |
|----------|-------------|
| Warm     | `warm`      |
| Neutral  | `neutral`   |
| Formal   | `formal`    |
| Playful  | `playful`   |

`TONE_SECONDARY` — lowercase each ticked label. Rules:

- if the user ticked more than 2 → keep the first 2 in the order they
  appeared in the response; tell the user: `Kept first 2 secondary tones; re-run /pet-init to adjust.`
- if the user ticked zero → `TONE_SECONDARY = []` (valid per schema)
- if the user ticked the SAME label as `TONE_PRIMARY` → drop the duplicate
  silently; if that leaves 0, leave `TONE_SECONDARY = []`

`HUMOR_STYLE` — lowercase:

| Label   | Value    |
|---------|----------|
| None    | `none`   |
| Dry     | `dry`    |
| Witty   | `witty`  |
| Silly   | `silly`  |

`HUMOR_FREQUENCY` — lowercase:

| Label        | Value         |
|--------------|---------------|
| Rare         | `rare`        |
| Occasional   | `occasional`  |
| Frequent     | `frequent`    |

## 2c — Consistency check

If `HUMOR_STYLE == "none"` and `HUMOR_FREQUENCY != "rare"`, emit a regular
message:

> Humor style is "none" but frequency is "<freq>". "None" overrides
> frequency — the pet will simply not attempt humor. Continue? (yes / change)

- `yes` → set `HUMOR_FREQUENCY = "rare"` (schema-valid + semantically honest)
- `change` → re-emit the Phase-2 batch (no partial re-runs; the whole
  voice set is asked again)

## Verify-criterion

- `TONE_PRIMARY` is one of `warm` / `neutral` / `formal` / `playful`
- `TONE_SECONDARY` is a list of 0-2 entries, no duplicates, none equal to
  `TONE_PRIMARY`
- `HUMOR_STYLE` is one of `none` / `dry` / `witty` / `silly`
- `HUMOR_FREQUENCY` is one of `rare` / `occasional` / `frequent`
- Consistency rule (2c) has been applied

## Failure modes (constructive paths)

If the user bails mid-batch (closes without answering):
- (A) keep whatever is set; emit defaults for unset: `neutral` / `[]` / `none` / `rare`; show the user what was defaulted and ask confirm
- (B) abort `/pet-init` cleanly, no file written
- (C) re-emit the whole batch once more
