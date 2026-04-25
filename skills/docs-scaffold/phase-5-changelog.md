# Phase 5 — CHANGELOG via `kei-changelog`

Goal: initialize or refresh `<DIR>/CHANGELOG.md` from the repo's
conventional-commit history using the Rust primitive.

## 5a — Pick invocation mode (AskUserQuestion #5)

```json
{
  "questions": [
    {
      "question": "CHANGELOG action?",
      "header": "Changelog",
      "multiSelect": false,
      "options": [
        {"label": "Initialize — full history as v0.1.0", "description": "First run. Walks from root to HEAD, writes a single v0.1.0 section."},
        {"label": "Unreleased — since last tag",         "description": "Prepends an Unreleased block since the most recent annotated tag."},
        {"label": "Update — since explicit --from <ref>", "description": "User supplies a git ref in the next message (tag name or SHA)"},
        {"label": "Skip this phase",                      "description": "No CHANGELOG changes; final report only"}
      ]
    }
  ]
}
```

On `Skip` → `CHANGELOG_STATUS = skipped`, continue to the final report.

## 5b — Resolve the binary

The Rust primitive lives at `_primitives/_rust/kei-changelog/`. Build if
not yet built:

```bash
( cd _primitives/_rust/kei-changelog && cargo build --release --offline ) \
  || ( cd _primitives/_rust/kei-changelog && cargo build --release )
```

Binary path: `_primitives/_rust/kei-changelog/target/release/kei-changelog`.

If the build fails (missing `git2` system deps — on Linux needs
`libgit2-dev`), fall back to NO DOWNGRADE advice:

1. Install system dep: `apt install libgit2-dev` / `brew install libgit2`.
2. Re-run this phase after install.

## 5c — Run the binary

Map the click to CLI flags:

| Click | Command |
|---|---|
| Initialize | `kei-changelog --version v0.1.0 --update "$DIR/CHANGELOG.md" --repo "$DIR"` |
| Unreleased | `kei-changelog --unreleased --from "$(git -C "$DIR" describe --tags --abbrev=0)" --update "$DIR/CHANGELOG.md" --repo "$DIR"` |
| Update | `kei-changelog --from <user_ref> --version <user_version> --update "$DIR/CHANGELOG.md" --repo "$DIR"` |

If the `Unreleased` variant fails because there are no annotated tags,
fall back to `--version v0.1.0` and continue — print a short note.

## 5d — Verify the result

Read the first 30 lines of `<DIR>/CHANGELOG.md` and show them inline so
the user confirms the output. Set `CHANGELOG_STATUS` to `initialized`,
`updated`, or `skipped`.

## Verify-criterion

- The binary exited with status 0 (or the Skip branch was chosen).
- `<DIR>/CHANGELOG.md` exists and starts with `# CHANGELOG`.
- New content was prepended, not appended, when the file already existed.
- `CHANGELOG_STATUS` is set to one of the three values above.
