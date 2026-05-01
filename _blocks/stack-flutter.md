# STACK — Flutter + Riverpod + Clean Architecture

Use for cross-platform mobile UI (iOS + Android from one codebase).

**State:** Riverpod (`flutter_riverpod` ≥ 2.x) — NOT Provider, NOT GetX, NOT Bloc by default. Narrow providers (one responsibility each), `autoDispose` unless state is genuinely session-wide.

**Layout — Feature-First + Clean Architecture:**
```
lib/
  core/           shared utils, error handling, network, Result type
  features/
    <feature>/
      data/        DTOs, repositories impl, API clients
      domain/      entities, use cases, repository interfaces
      presentation/widgets, screens, providers
```
`features/<A>` CANNOT import `features/<B>` directly — cross-feature goes through `core/` or a use case.

**Pre-commit gate (MANDATORY):**
```
flutter analyze   # zero warnings
flutter test      # all green
```
Both must pass. No commit without both. `pubspec.lock` is committed to git.

**Merge-base gotcha:** when merging multiple API timelines of different lengths (e.g. 15-day + 16-day feeds), use the LONGER timeline as base — otherwise day N+1 silently drops. Merge logic lives in exactly ONE use case (Single Source of Truth).

**Secrets:** `--dart-define=KEY=value` at build, or `.env` loaded at startup via `flutter_dotenv`. NEVER literal in `lib/`. `.env` in `.gitignore`.

**Forbidden:** Provider + Riverpod mixed, cross-feature imports, committing `build/` or `.env`, file > 200 LOC / function > 30 LOC, merge logic duplicated across screens.
