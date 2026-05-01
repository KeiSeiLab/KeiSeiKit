# DOMAIN — Secrets handling

Project stores credentials / API keys / private keys / tunnel keys. Treat every leaked byte as irrecoverable.

**Storage convention:**
- Path: `<repo>/secrets/*.env` — NEVER checked in.
- `.gitignore` has `secrets/` **before any secret is written into the tree**. Verify with `git check-ignore secrets/foo.env` (should print the path).
- File permissions `chmod 600` on every secret file.

**Reference by path only in reports / logs / chats:**
> "Using keys from `secrets/nodes.env`" — GOOD.
> "Using key `abc123xyz...`" — FORBIDDEN.

Never echo secret values in:
- Agent output / tool reports
- Chat messages back to user
- Stdout / stderr of running processes
- Commit messages, PR descriptions
- Error messages (log the CODE path, not the token)

**Loading at runtime:**
- Rust: `dotenvy` or plain `std::env::var` after `direnv allow`.
- Python: `python-dotenv` at startup, NEVER inline literals.
- Node/Next: `.env.local` (`.gitignore`), platform vars in prod.
- Shell: `source secrets/foo.env` → `export` inside, never commit the export line.

**Rotation:** when a secret is suspected leaked — rotate at provider → update `secrets/*.env` → restart services → verify old key rejected. Do not "wait and see".

**Forbidden:** committing `.env` / `secrets/` (even once — git history persists); echoing values in reports; literal API keys in `lib/` / `src/` / `Cargo.toml` / `package.json`; `git add -A` in a repo that has secrets (use explicit file paths); copying secret values into chat to "show" user what's there.
