# Phase 1 — Repo provider + visibility

Ask the user to pick where the memory-repo lives. Purely click-based —
two `AskUserQuestion` batches, zero free text.

## 1a — Provider click

Emit ONE `AskUserQuestion`:

```json
{
  "questions": [
    {
      "question": "Where does your memory-repo live?",
      "header": "Provider",
      "multiSelect": false,
      "options": [
        {"label": "GitHub",      "description": "github.com — easiest; private repo recommended"},
        {"label": "GitLab",      "description": "gitlab.com or self-managed GitLab"},
        {"label": "Bitbucket",   "description": "bitbucket.org (Atlassian)"},
        {"label": "Self-hosted", "description": "Forgejo / Gitea / custom; requires SSH access"}
      ]
    }
  ]
}
```

Store the pick as `PROVIDER`.

## 1b — Visibility click

Emit ONE `AskUserQuestion`:

```json
{
  "questions": [
    {
      "question": "Repo visibility — private is strongly recommended. Your traces contain prompts and tool calls.",
      "header": "Visibility",
      "multiSelect": false,
      "options": [
        {"label": "Private (recommended)", "description": "Only you + the deploy key can read"},
        {"label": "Public (I accept the risk)", "description": "Traces visible to anyone — confirm below"}
      ]
    }
  ]
}
```

Store the pick as `VISIBILITY`.

## 1c — Public-visibility warning

If `VISIBILITY == "Public (I accept the risk)"`, print the warning block
below to stdout BEFORE proceeding to Phase 2:

```
WARNING: a public memory repo leaks your session prompts, tool usage, and
file paths to anyone who finds the repo. This is rarely what you want.
If you proceed, the rest of the wizard will continue unchanged — there is
no second confirmation.
```

Do NOT emit a third AskUserQuestion for re-confirm — the user already
picked "I accept the risk".

## Verify-criterion

- `PROVIDER ∈ {GitHub, GitLab, Bitbucket, Self-hosted}`.
- `VISIBILITY ∈ {Private, Public}`.
- Exactly TWO `AskUserQuestion` calls were emitted in this phase.
