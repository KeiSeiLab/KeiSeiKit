# Pure-Click Contract

> Shared rule block — referenced by pipeline and multi-phase skills
> (ci-scaffold, auth-setup, observability-setup, docs-scaffold,
> schema-design, self-audit, sleep-on-it, and others).

## Rule

Every decision in the skill is made via `AskUserQuestion` (option-picker UI,
NOT free-text). The ONLY permitted typed input is intake — a one-line or
one-paragraph description — which is immediately classified into options on
the next phase.

## What counts as "intake" (typed input allowed)

- Phase 1 one-line description of the target repo / app / service.
- Phase 2 entity list (for `/schema-design`) — typed list of table names.
- Free-text reason for a user-declared bypass or override.

## What MUST be a click (AskUserQuestion)

- Every binary yes/no decision.
- Every "pick one of N" decision (platform, ORM, motion-tier, auth-flow).
- Every "pick subset of N" decision (sections, providers, dashboards).
- Every approve / iterate / switch / abort prompt.
- Every per-finding fix / skip / defer prompt in verify/audit phases.

## Why

- Click-driven flows are replayable and auditable — the option taken is in
  the transcript, not inferred from free-form text.
- Options constrain the decision space to what the skill actually handles,
  preventing silent scope creep.
- `AskUserQuestion` is the tool the harness renders as a proper picker UI;
  free-text prompts degrade to plain chat.

## Non-compliance

If a skill prompts the user for a value that IS in a closed enum (e.g.
"which framework?") but does NOT use `AskUserQuestion`, that is a
contract violation. Fix by replacing the prompt with an `AskUserQuestion`
call whose `options` array lists the enum values.
