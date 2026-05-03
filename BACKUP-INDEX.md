# Backup Index — 3-way merge 2026-04-29

> Альтернативные дизайны, не выбранные в финальный merge — сохранены
> на случай если основной выбор покажет проблемы и придётся откатиться.
>
> Все три тэга на forgejo (`origin`, `<private-forgejo>/<user>/<repo>`).
> Author keeps the kit on a private remote.

---

## Финальный merge

| Что | Где |
|---|---|
| Merge commit | `e8481b9` на `main` → запушен в forgejo origin/main (`b6a36ac` HEAD) |
| Integration branch | `integration/2026-04-29-merge-3way` (forgejo) |
| PR-URL | `<private-forgejo>/<user>/<repo>/compare/<base>...<head>` |

## Backup tags (forgejo origin)

### `backup/audit-wave5-6-smoke-2026-04-29`

Сохранил: 5 коммитов на `audit/wave5-6-smoke` от base `b26ac053`.
- 4 audit-snapshots (wave5+6, 7, 8, 9) — добавляют 20 крейтов Hosted Sleep
- 1 fix commit `b8f91dc` — H-2 token-usage capture + M-2/M-3/M-4 cleanup
- HEAD: `cead3db`

Зачем: backup_path для H-2 implementation (cherry-picked в integration).

### `backup/wave55c-verified-pricing-final-2026-04-29`

Сохранил: 15 коммитов на `fix/wave55c-verified-pricing-final` от base `b26ac053`.
- W55b stages 2+3+4 — kei-cortex/kei-router MODEL → kei_model::resolve()
- W55c verified pricing 2026-04-28 + selectors role mappings
- P1.1.c+e — responses.rs + chat_completions streaming wiring
- P1.1.d — runs.rs/run_agent.rs → real LLM via stream_events
- P1.1.f — wiremock fixture + docs
- 7 новых крейтов (svc-systemd, llm-bridge-mlx, llm-router, compute-baremetal, compute-vultr, compute-linode, kei-model)
- Token tracker
- HEAD: `91c0a55`

**Альтернативный streaming-дизайн в этом backup'е**: `spawn_tee_persist` тiирует upstream channel в downstream и асинхронно
персистит на Done. Финальный merge ИСПОЛЬЗУЕТ этот дизайн (`tee` выиграл).

### `backup/n1-n4-cleanup-residue-oneshot-2026-04-29`

Сохранил: 1 коммит `8dd4fdf` поверх `audit/wave5-6-smoke`.
- N-1 + N-4 + L-2 + M-4-B fixes
- HEAD: `8dd4fdf`

**Альтернативный streaming-дизайн в этом backup'е**: `oneshot::Receiver<(text, usage)>` — forwarder отдаёт persist-callback'у
буфер текста + Usage когда Done. Финальный merge **НЕ выбрал** этот вариант (предпочёл wave55c's tee). 

**Когда полезно**: если tee design покажет проблемы (race conditions при медленных downstream, потеря событий при channel
backpressure), переключиться на oneshot — один консьюмер, синхронная persist логика, проще для отладки.

---

## Восстановление альтернативного streaming-дизайна

```bash
# Полный rollback на oneshot design
git checkout backup/n1-n4-cleanup-residue-oneshot-2026-04-29
git checkout -b fix/restore-oneshot-streaming-2026-XX-XX
# Reapply остальные фиксы поверх...

# Cherry-pick конкретного файла
git checkout backup/n1-n4-cleanup-residue-oneshot-2026-04-29 -- \
    _primitives/_rust/kei-cortex/src/routes/openai/chat_completions.rs \
    _primitives/_rust/kei-cortex/src/routes/openai/stream_forwarder.rs
```

---

## Stash queue (12 оставшихся)

После merge'а 3 stash'a удалены как provably-merged. 12 stash'ей остались
от других веток — отдельный housekeeping pass:

```
stash@{0}: fix/wave55c-verified-pricing-final  wave55c-extras
stash@{1}: fix/wave55c-verified-pricing-final  wave9-net-wip
stash@{2}: fix/wave55c-verified-pricing        token-wire WIP
stash@{3-11}: разные feature ветки от 2026-04-22…04-28
```

Не блокирует ничего; разобрать когда будет время.

---

## Worktrees (process scratch)

`.claude/worktrees/agent-*` — 42 директории от прошлых kei-spawn вызовов.
Stale agent worktrees — должны быть GC'нуты через `kei-fork gc` или вручную.
Не блокирует merge; отдельный housekeeping.

`tasks/ag-edit-shared-*/` + 6 `tasks/*.toml` task-spec файлов — process
scratch, оставлены untracked. Можно gitignore или закоммитить как audit
trail если важно сохранить.

---

## Date lock

2026-04-29. Все 3 backup tag'а pushed на forgejo `origin` 2026-04-28.
Удалять только если дизайн tee-persist подтвержден стабильным под
production нагрузкой (≥30 дней, ≥1M запросов).
