---
title: campaigns.rs
path: kei-content-store/src/campaigns.rs
dna_hash: sha256:297ecbae485819f6
language: rust
size_loc: 44
generated: by-keidocs
---

# kei-content-store/src/campaigns.rs

Campaigns + campaign_assets join.

`create_campaign` delegates to `kei_entity_store::verbs::create` under
`CAMPAIGNS_SCHEMA` — plain INTEGER-PK CRUD, engine-owned since
2026-04-23.

`attach_asset` / `campaign_assets` stay bespoke: `campaign_assets`
has a composite `(campaign_id, asset_id)` PK with no single-column
id, so it cannot be described as an `EntitySchema` (engine requires
exactly one PK field). The attach path also uses `INSERT OR IGNORE`
for idempotent joins, which the engine's plain-INSERT `create` verb
would not preserve.

## Related

- parent: `kei-content-store/Cargo.toml`
- imports: anyhow, crate, kei_entity_store, rusqlite, serde_json

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
