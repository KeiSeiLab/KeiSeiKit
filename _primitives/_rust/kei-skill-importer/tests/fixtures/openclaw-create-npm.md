---
name: create-npm-package
description: Scaffold a new npm package with TypeScript + tsup + vitest baseline.
tags: [npm, typescript, scaffold]
tools: [pnpm, git]
---

# Create NPM Package

Telegraph style. Scaffold an empty workspace member.

## Start

- Confirm target directory does not exist.
- Read root `package.json` to copy workspace conventions.

## Commands

```bash
pnpm init
pnpm add -D typescript tsup vitest
kei-task create "scaffold dist/ build pipeline"
```

## Code

- TS strict, ESM only.
- tsup config in `tsup.config.ts`.
- Vitest config inline in `package.json`.

## Gates

- `pnpm build` green before commit.
- `pnpm test` green before push.
