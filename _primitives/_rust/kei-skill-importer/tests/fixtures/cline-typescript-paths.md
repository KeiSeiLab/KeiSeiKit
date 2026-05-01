---
description: TypeScript style guide for component files.
paths:
  - "**/*.ts"
  - "**/*.tsx"
tags: [typescript, react]
---

# TypeScript Component Rules

Every component file in this project must follow these conventions:

- Use named exports only. Default exports are forbidden.
- Props interfaces named `<Component>Props` and exported.
- No inline `any`; use `unknown` and narrow.
- Hooks live alongside the consuming component, not in a global hooks/ folder.

## Imports

- React imports first, then third-party, then internal.
- Absolute imports use the `~/` alias.
