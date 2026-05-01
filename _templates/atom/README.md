# Atom template

Used by `scripts/new-atom.sh <crate> <verb> [kind]` to scaffold a new atom. Placeholder substitution map:

| Placeholder | Example | Source |
|---|---|---|
| `__CRATE__` | `kei-task` | argv 1 (kebab-case) |
| `__CRATE_SNAKE__` | `kei_task` | argv 1 → underscores |
| `__VERB__` | `add-dependency` | argv 2 (kebab-case) |
| `__VERB_SNAKE__` | `add_dependency` | argv 2 → underscores |
| `__KIND__` | `command` | argv 3 or default `command` |
| `__DESCRIPTION__` | free-form one-liner | prompted at runtime |

Schema SSoT: [SUBSTRATE-SCHEMA.md](../../docs/SUBSTRATE-SCHEMA.md).

Template covers the 4 files a new atom always needs:

- `atoms/<verb>.md` — human doc + YAML frontmatter (machine-parsed by kei-sage + kei-runtime)
- `atoms/schemas/<verb>-input.json` — JSON Schema draft-07
- `atoms/schemas/<verb>-output.json` — JSON Schema draft-07
- `src/atoms/<verb>.rs` — Rust impl skeleton with Input/Output/Error + `pub fn run`
- `tests/<verb>_smoke.rs` — smoke test placeholder

Postconditions the generator enforces:

1. `cargo check -p <crate>` passes (skeleton compiles)
2. `kei-schema-lint <crate>` passes (frontmatter + schema paths valid)
3. New atom appears in `kei-runtime list-atoms --crate <crate>`

If any postcondition fails, the generator rolls back (deletes the generated files) so there is no half-scaffolded state.
