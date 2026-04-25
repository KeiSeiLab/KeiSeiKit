# STACK — FastAPI + async SQLAlchemy 2.0 + PostgreSQL

Use when the project is Python-locked (existing codebase) or needs Python-exclusive bindings. Justify on first touch.

**Core versions:** FastAPI ≥ 0.110, SQLAlchemy 2.0 async style (`AsyncSession`, `select()`, `await session.execute()` — NOT the legacy `Query` API), Pydantic v2 (NOT v1), Alembic for migrations, pytest-asyncio for tests.

**Session pattern:**
```python
async def get_db() -> AsyncIterator[AsyncSession]:
    async with async_session() as session:
        yield session        # FastAPI unwinds on response

@router.get("/x")
async def handler(db: Annotated[AsyncSession, Depends(get_db)]): ...
```
Dependency injection via `Depends()` — never thread a session through global state.

**Commit rule:** inside an `@asynccontextmanager` block, do NOT call `session.commit()` in the request path — let the context manager close the txn. Mixing the two causes the "RuntimeError: Session is already flushing" storm.

**Migrations:** Alembic only. No raw `ALTER TABLE` on prod. Migrations checked into git alongside the model change in the same commit.

**Common security-debt checklist:** on touch, fix the known issues — default SECRET_KEY, missing CSRF, rate-limit not applied, N+1 in paginated queries. Don't paper over.

**Deploy:** Docker + nginx reverse proxy (typical pattern: app container on internal port, nginx on public port). Shared-host coordination: check cross-project impact before apt/systemd/nginx changes.

**Forbidden:** `session.commit()` in request handler if `get_db` is contextmanager-based; raw SQL on prod; committing `.env` (DB credentials, API tokens); deprecated model aliases — pin the dated model string.
