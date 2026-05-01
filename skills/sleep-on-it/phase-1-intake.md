# Phase 1 — Task intake (one free-text field)

The single free-text field in the wizard. Everything else is a click.

## 1a — Prerequisite check

Before prompting, verify the v0.11 pipeline is live:

```bash
# Resolve sync-repo path from env or secrets file.
# shellcheck disable=SC1091
[ -f "${HOME}/.claude/secrets/.env" ] && . "${HOME}/.claude/secrets/.env"
REPO_PATH="${KEI_MEMORY_REPO_PATH:-}"
QUEUE_SH="${HOME}/.claude/agents/_primitives/kei-sleep-queue.sh"

if [ -z "$REPO_PATH" ] || [ ! -d "$REPO_PATH/.git" ] || [ ! -x "$QUEUE_SH" ]; then
    printf 'v0.11 sleep-sync not configured — run `/sleep-setup` first, then retry.\n'
    exit 0
fi
```

If either check fails, exit the wizard. Do not offer offline queue mode.

## 1b — Free-text prompt

Emit a plain chat message (NOT `AskUserQuestion` — a free-text message
is fine when it is the only typed field and has a trivial non-empty
validator):

> What are you sleeping on? One or two sentences — the nightly agent
> will read this verbatim. Examples:
>
> - "Should I pick a continuous-time net or a small transformer as the memory re-ranker?"
> - "Compare SvelteKit, Astro, and Next.js App Router for the kit's landing page."
> - "What pattern in recent audit-backlog entries has the highest fix-value-per-effort?"

Store the reply as `TASK_TEXT`.

## 1c — Validate

- Reject if `TASK_TEXT` is empty or only whitespace.
- Reject if `TASK_TEXT` > 4000 characters (keep queue files small; the
  agent has 15 minutes wall-clock anyway — a novella does not help).

On reject, print

```
Task description must be non-empty and <= 4000 chars. Try again?
```

and re-prompt. Up to 3 attempts; on the 3rd empty submission abort the
wizard with a short "try again with `/sleep-on-it`" message.

## Verify-criterion

- `TASK_TEXT` is non-empty and <= 4000 chars.
- The v0.11 sync pipeline is wired (REPO_PATH + QUEUE_SH exist).
- Exactly ZERO `AskUserQuestion` in this phase (free-text message only).
