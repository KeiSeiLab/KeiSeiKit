---
name: msg
description: Read or write the cross-session mailbox by @id. Send a message to another Claude Code session (`/msg @name text`), read your own inbox (`/msg` with no args), broadcast to everyone (`/msg all text`), list the whole bus, or discover who is reachable. Thin wrapper over the `kei message` jsonl mailbox — messages land in the recipient's NEXT turn via the mailbox-inject hook (pull, not push). Use whenever the user wants sessions/agents to talk to each other.
argument-hint: "[@name] <message>  |  (empty = read inbox)  |  list  |  who"
---

# /msg — Inter-Session Mailbox

A persistent append-only bus so ANY Claude Code session can message ANY other —
not just Agent-Teams teammates, no tmux, no daemon. Backed by
`~/.claude/scripts/kei-message.sh` writing `~/.claude/mailbox/messages.jsonl`.
The `mailbox-inject.sh` UserPromptSubmit hook pulls each session's unread into
its context once per turn, so delivery is **pull** (arrives on the recipient's
next turn), not instant push.

## Identity model

- **Your address** = the basename of this session's working directory (`$PWD`).
  So a session running in `~/Projects/frontend` is reachable as `@frontend`.
- **`all`** is the broadcast channel — every session sees `to:"all"` messages.
- You can override the sender with `--from <name>` and the reader identity with
  `--me <name>` if a session's cwd basename isn't the name you want to use.

## Command map

Interpret `$ARGUMENTS` and run the matching command via Bash, then show its
output to the user. The launcher `kei message …` and the script path are
equivalent — prefer the script path (always present after install):

| User typed | Run |
|---|---|
| `/msg` (no args) | `~/.claude/scripts/kei-message.sh inbox` |
| `/msg @frontend ship it` | `~/.claude/scripts/kei-message.sh send @frontend ship it` |
| `/msg all standup in 5` | `~/.claude/scripts/kei-message.sh send all standup in 5` |
| `/msg list` | `~/.claude/scripts/kei-message.sh list` |
| `/msg who` (or `channels`) | `~/.claude/scripts/kei-message.sh channels` |

Rules for parsing `$ARGUMENTS`:

1. **Empty** → read inbox (`inbox`). Show the messages addressed to this
   session or to `all`.
2. **Starts with `@<name>`** → send to that recipient; the rest is the body.
   A `@x` that appears later in the body stays literal text.
3. **Starts with `all `** → broadcast; the rest is the body.
4. **`list`** → print the recent whole bus (every from→to line).
5. **`who` / `channels`** → print known recipient names (use this to discover
   who is reachable before sending the first message).
6. Anything else with no leading `@`/`all` → treat as a broadcast body, OR ask
   the user who the recipient is if it's ambiguous.

## Discovery (first-message problem)

A recipient only appears in `who` after it has sent or been sent a message, so
for the very first contact either broadcast with `all`, or ask the user for the
target session's cwd-basename. Don't invent a recipient name.

## Notes

- Sending never blocks and never notifies the recipient out-of-band — they see
  it on their next turn. For a time-sensitive ping, tell the user it's queued.
- This is plain files: `cat ~/.claude/mailbox/messages.jsonl` is the raw bus.
- Bypass the inject hook for a session with `KEI_MAILBOX_BYPASS=1`.
