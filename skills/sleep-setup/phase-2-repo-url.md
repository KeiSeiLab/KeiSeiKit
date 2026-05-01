# Phase 2 — Collect SSH repo URL

The one and only free-text field in the wizard. Everything else is a
click.

## 2a — Free-text prompt

Emit ONE `AskUserQuestion` with a `freeText` field:

```json
{
  "questions": [
    {
      "question": "Paste the SSH URL of the memory repo you created.",
      "header": "Repo URL",
      "multiSelect": false,
      "freeText": true,
      "placeholder": "git@github.com:you/kei-memory.git"
    }
  ]
}
```

## 2b — Validate

Regex: `^git@[A-Za-z0-9._-]+:[A-Za-z0-9._/-]+\.git$`

If the user's input does NOT match, print:

```
Invalid SSH URL. Expected shape: git@<host>:<org>/<repo>.git
Examples:
  git@github.com:alice/kei-memory.git
  git@gitlab.com:alice/devops/kei-memory.git
  git@forgejo.example.com:alice/kei-memory.git
```

Re-emit the same `AskUserQuestion`. Up to 3 attempts; on the 3rd failure
abort the wizard with a short "try again later with `/sleep-setup`"
message. Do not loop silently.

## 2c — Cross-check against Phase 1

Extract the host from the URL:

```
host = url.match(/^git@([^:]+):/)[1]
```

If `PROVIDER == "GitHub"` and `host != "github.com"`, print a soft
warning: "host <host> doesn't look like github.com — continuing anyway".
Same for GitLab → `gitlab.com`, Bitbucket → `bitbucket.org`. For
`Self-hosted`, skip this check.

Store the URL as `REPO_URL`.

## Verify-criterion

- `REPO_URL` matches the validation regex.
- Exactly ONE `AskUserQuestion` (or up to 3 if the user mistyped).
- No secret-like token accidentally pasted into the URL
  (regex rejects `@` outside the leading `git@`).
