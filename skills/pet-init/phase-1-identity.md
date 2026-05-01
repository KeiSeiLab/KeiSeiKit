# Phase 1 — Identity

Gather the four `[identity]` fields: `pet_name`, `user_name`, `addressing`,
`languages`. Free-text for names (no enum), click-based for the rest.

## 1a — Pet name (free text)

Emit a regular message (NOT AskUserQuestion):

> What should your pet be called?
> - 1 to 30 characters
> - letters, digits, hyphen, underscore, space
> - examples: `Kei`, `Momo`, `Pixel`, `小可`
>
> Reply with the name on one line.

Capture the reply as `PET_NAME`. Validate:

- length 1-30 chars after trimming whitespace
- at least one non-whitespace character

If validation fails → tell the user which rule was violated and ask again.
Never fall through with an invalid name. Never invent a default.

## 1b — User name (free text)

Emit a regular message:

> What should your pet call YOU?
> - examples: `Alex`, `Den`, `boss`, `capitan`
> - 1-30 characters, any script
>
> Reply on one line.

Capture as `USER_NAME`. Same validation as `PET_NAME`.

## 1c — Addressing + languages (AskUserQuestion, 1 batch)

Emit a single `AskUserQuestion` call with TWO questions:

```json
{
  "questions": [
    {
      "question": "How should the pet address you?",
      "header": "Addressing",
      "multiSelect": false,
      "options": [
        {"label": "By name",   "description": "Uses your name directly, e.g. \"Alex, look at this\""},
        {"label": "Formal",    "description": "Respectful, keeps distance, e.g. \"You may want to see this\""},
        {"label": "Casual",    "description": "Relaxed, nickname-friendly, e.g. \"Hey, check this out\""}
      ]
    },
    {
      "question": "Which languages should the pet use?",
      "header": "Languages",
      "multiSelect": true,
      "options": [
        {"label": "English (en)",   "description": "Default for most users"},
        {"label": "Russian (ru)",   "description": "русский"},
        {"label": "Spanish (es)",   "description": "español"},
        {"label": "French (fr)",    "description": "français"},
        {"label": "German (de)",    "description": "Deutsch"},
        {"label": "Chinese (zh)",   "description": "中文"},
        {"label": "Japanese (ja)",  "description": "日本語"},
        {"label": "Other",          "description": "I'll specify after this batch"}
      ]
    }
  ]
}
```

Map the addressing click to `ADDRESSING`:

| Label     | Value      |
|-----------|------------|
| By name   | `by-name`  |
| Formal    | `formal`   |
| Casual    | `casual`   |

Map the language multi-select to `LANGUAGES` (ISO 639-1 codes). If the user
ticked "Other":

- emit a regular message: `Which other language? Reply with ISO 639-1 code (e.g. "it", "pt", "ko") or space-separated list.`
- parse reply into additional 2-letter codes
- append to `LANGUAGES`

If no language is selected (all options unchecked) → default to `["en"]`
and tell the user: `No language chosen — defaulting to English.`

## Verify-criterion

- `PET_NAME` set, trimmed, 1-30 chars
- `USER_NAME` set, trimmed, 1-30 chars
- `ADDRESSING` is exactly one of `by-name` / `formal` / `casual`
- `LANGUAGES` is a non-empty array of 2-letter ISO codes
- If user typed "Other", at least one extra code was captured

## Failure modes (constructive paths, NO DOWNGRADE)

If the user declines to give a name:
- (A) suggest `Kei` as a placeholder — explain it can be changed later via re-run
- (B) abort `/pet-init` and invite them to try when ready
- (C) pick a name from a small curated list (`Kei`, `Momo`, `Pixel`, `Echo`)

Offer all three; never silently fall through.
