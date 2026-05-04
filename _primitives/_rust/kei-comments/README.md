# kei-comments

> Sovereign threaded comment store. SQLite-backed primitive that
> replaces Giscus + GitHub Discussions as the comment backend for
> KeiWiki and any other surface served via the cortex daemon.

## What it does

- Persists comments per-page as a flat SQLite table with
  `parent_id` self-references for arbitrary-depth threading.
- Soft-delete: only the original `author` may delete; body is
  wiped to empty string but the row remains so the thread shape
  survives.
- Reactions: many-to-many `(comment_id, author, emoji)` with
  composite primary key. `react` is idempotent, `unreact` is a
  no-op on missing rows.
- Hard 10 KB cap on body bytes (`MAX_BODY_BYTES`) — empty / whitespace
  bodies are rejected up-front.

## What it does NOT do

- Authentication. The `author` field is treated as an opaque
  attested identifier supplied by the caller. The cortex daemon
  is responsible for verifying the bearer (Apple / Google /
  WebAuthn / magic-link via the sibling `kei-auth-*` providers)
  before invoking this store.
- Federation, signing, replication. Single-process SQLite. For
  multi-node sync, layer `kei-ledger-sign` ed25519 attestation on
  top and replicate the row stream out-of-band.
- Markdown rendering. Bodies are stored verbatim; the rendering
  surface (KeiWiki) is responsible for sanitisation and rendering.

## CLI

```sh
kei-comments migrate
kei-comments post --page intro --author alice --body "hello"
kei-comments post --page intro --author bob --body "re: hello" --parent <id>
kei-comments list --page intro                                    # JSON array
kei-comments react --id <comment_id> --author bob --emoji 👍
kei-comments unreact --id <comment_id> --author bob --emoji 👍
kei-comments reactions --id <comment_id>                          # JSON map
kei-comments delete --id <comment_id> --author alice              # ok | denied
```

Default DB: `~/.keisei/comments.sqlite` (override `--db` or
`$KEI_COMMENTS_DB`).

## Library

```rust
use kei_comments::CommentStore;
let store = CommentStore::open(&db_path)?;
store.migrate()?;
let id = store.post("intro", "alice", "hello", None)?;
let thread = store.list("intro")?;
```

## Cortex integration

The cortex daemon mounts five HTTP routes that proxy to this
store after the bearer has been verified:

```
POST   /api/comments/:page          # body { author, body, parent_id? }
GET    /api/comments/:page          # → [Comment]
DELETE /api/comments/:id            # author from session
POST   /api/comments/:id/reactions  # body { emoji }
GET    /api/comments/:id/reactions  # → { emoji: [author...] }
```
