# Rules-as-Blocks — Wave 14 Architectural Note

**Status:** infrastructure landed 2026-04-30 (Wave 14a + 14b). Manifest migration is per-install — public release ships with `rule_blocks = []` (default empty) on every manifest.

## What this enables

Treat user-defined rule files as composable substrate fragments. Each agent manifest declares which rule fragments it needs. The assembler injects them at build time, alongside existing substrate blocks.

This unifies two previously-separate concepts:
- **Substrate blocks** (`_blocks/*.md`) — composed by `_assembler` from manifest's `blocks = [...]` field
- **Rules** (user's `~/.claude/rules/*.md`) — previously global umbrella, loaded for every session regardless of agent

After Wave 14, rules are first-class blocks with their own DNA and per-agent selection.

## Pipeline

```
~/.claude/rules/*.md
  │
  │  kei-decompose decompose-rules
  ▼
~/.claude/registry.sqlite
  blocks(block_type='rule', name='<rule-slug>::<section-slug>',
         scope_sha8=sha8(rule-slug), body_sha8=sha8(body))
  │
  │  _assembler reads manifest
  ▼
_manifests/<agent>.toml
  rule_blocks = ["karpathy-behavioral::1-think-before-coding",
                 "code-style::code-style-constructor-pattern", ...]
  │
  │  assembler composes
  ▼
agents/<agent>.md
  ┌─────────────────────────────────┐
  │ frontmatter (name, tools, model)│
  │ ROLE                            │
  │ role-section (capabilities)     │
  │ <!-- RULE: karpathy::think... -->│
  │ <fragment body verbatim>        │
  │ <!-- RULE: code-style::... -->  │
  │ <fragment body verbatim>        │
  │ blocks (existing substrate)     │
  │ footer                          │
  └─────────────────────────────────┘
```

## Step-by-step migration (for a user's install)

### 1. Decompose your rules into fragments

```bash
kei-decompose decompose-rules
# → Decomposed N rule files into M fragments (M new, 0 superseded, 0 unchanged)
```

This walks `~/.claude/rules/*.md` (and `specialty/*.md` + `projects/*.md`), splits each on `## ` H2 headings, and registers each section as a `BlockType::Rule` row in `~/.claude/registry.sqlite`.

Re-running is idempotent: unchanged bodies are no-ops; changed bodies create supersede chain entries.

### 2. List available fragments

```bash
sqlite3 ~/.claude/registry.sqlite \
  "SELECT name FROM blocks WHERE block_type='rule' ORDER BY name"
```

Output is `<rule-slug>::<section-slug>` per line.

### 3. Pick fragments per agent category

Suggested mapping for the 37 ship manifests (refine to your rules corpus):

| Agent category | Suggested rule_blocks |
|---|---|
| **Universal** (all agents) | `karpathy-behavioral::*`, `code-style::*`, `numeric-claims-evidence::*` |
| **code-implementer-** (rust/swift/python/go/flutter/typescript) | + `dev-workflow::*`, `git-conventions::*`, `debugging::*`, `agent-git-model::*`, `orchestrator-branch-first::*`, `shipped-vs-functional::*` |
| **ml-implementer**, **ml-researcher** | + `ml-protocol::*`, `math-first-gate::*`, `observable-classification::*`, `manifold-tangent-sanity::*`, `pre-registration::*`, `specialized-node-training::*` |
| **security-auditor-** (differential/supply-chain/variant) | + `security::*`, `secrets-single-source::*`, `no-downgrade-constructive::*` |
| **critic-** (anti-pattern/bug/perf/tech-debt) | + `debugging::*` |
| **infra-implementer-** (cicd/container/iac/secrets) | + `dev-workflow::*`, `git-conventions::*`, `secrets-single-source::*`, `api-cost-guard::*` |
| **researcher-** (code/web/hybrid) | minimal — universal only + `numeric-claims-evidence::*` |
| **validator-** (api/benchmark/code-reality/doc/version) | minimal — universal only |
| **architect** | + `code-style::*`, `dev-workflow::*`, `debugging::*`, `no-downgrade-constructive::*` |
| **modal-runner**, **fal-ai-runner**, **cost-guardian** | + `api-cost-guard::*` |

`*` means "all fragments under that rule slug". You can also be precise by listing each section explicitly:

```toml
rule_blocks = [
    "karpathy-behavioral::1-think-before-coding",
    "karpathy-behavioral::2-surgical-changes",
    "karpathy-behavioral::3-goal-driven-execution",
    "code-style::code-style-constructor-pattern",
    "numeric-claims-evidence::the-rule",
]
```

### 4. Edit `_manifests/<agent>.toml` files

Add the field next to existing `blocks = [...]`:

```toml
substrate_role = "edit-local"
blocks = [
    "baseline",
    "evidence-grading",
    ...
]
rule_blocks = [
    "karpathy-behavioral::1-think-before-coding",
    "code-style::code-style-constructor-pattern",
]
```

`#[serde(default)]` means manifests without the field still work — no breakage during gradual migration.

### 5. Re-run the assembler

```bash
cd path/to/KeiSeiKit-public
./_assembler/target/release/assemble  # or `cargo run -p agent-assembler`
```

Generated `.md` files now contain `<!-- RULE: ... -->` markers around each injected fragment, between the substrate-role section and the existing blocks injection.

### 6. Verify

```bash
grep -c "RULE:" agents/code-implementer-rust.md
# → number of rule fragments injected
```

## Why this is better than the umbrella `CLAUDE.md` approach

Before Wave 14:
- All agents got the FULL rule corpus via global `CLAUDE.md` umbrella loading
- Explore (read-only) saw `specialized-node-training.md` (irrelevant)
- ml-implementer saw `patents.md` (irrelevant)
- No way to deduplicate `_blocks/baseline.md` (which summarizes rules) vs full rule files
- Token waste, signal dilution

After Wave 14:
- Each agent sees ONLY rule fragments in its `rule_blocks` list
- DNA-level identity per fragment enables supersede tracking + caching
- Same composition mechanism for substrate blocks AND rule blocks
- Public `_blocks/` remains the public substrate layer; user's `~/.claude/rules/` becomes the user's per-install overlay

## Technical references

- Parser: `_primitives/_rust/kei-decompose/src/parsers/rule.rs`
- CLI: `_primitives/_rust/kei-decompose/src/rules_cmd.rs` (subcommand `decompose-rules`)
- Registry: `_primitives/_rust/kei-registry/src/block.rs` (`BlockType::Rule`)
- Assembler injection: `_assembler/src/assembler.rs::write_rule_blocks`
- Validator: `_assembler/src/rule_blocks_check.rs`
- DNA wire format: `_primitives/_rust/kei-shared/src/dna.rs::compose_dna`

## Migration progress in this repo

**Public KeiSeiKit-public 1.0**: ships with `rule_blocks = []` on every manifest. Infrastructure ready, no opinionated default rules.

**Per-user install**: follow the steps above. `kei-decompose decompose-rules` is idempotent; manifest edits are forward-compatible.

## Lock

2026-04-30 (Wave 14 a+b infrastructure). Wave 14c manifest migration is per-install (no canonical commits in public).
