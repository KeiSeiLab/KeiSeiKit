---
title: main.rs
path: kei-provision/src/main.rs
dna_hash: sha256:9dee2d7adca646ea
language: rust
size_loc: 141
generated: by-keidocs
---

# kei-provision/src/main.rs

kei-provision — unified VPS provisioner CLI.

USAGE
kei-provision <backend> create  <name> [--type T] [--location L]
[--image I] [--ssh-key K]
[--firewall F] [--user-data PATH]
kei-provision <backend> status  <name>
kei-provision <backend> destroy <name> [--force]
kei-provision <backend> list

<backend>: hetzner | vultr

ENV (RULE 0.8 — secrets single source)
HCLOUD_TOKEN   — Hetzner API token
VULTR_API_KEY  — Vultr API key

Source via: `source ~/.claude/secrets/.env` before invocation.

## Public API

- Backend to use.
- Create a server (idempotent — returns existing IP if name/label taken).
- Server type / plan (hetzner: `cx22`; vultr: `vc2-1c-1gb`).
- Datacenter (hetzner: `fsn1`; vultr: `ams`).
- Image / OS (hetzner: `debian-12`; vultr: os-id string).
- SSH key id/name.
- Firewall id/name.
- cloud-init user-data file.
- Print server info (absent ⇒ "absent" line, exit 0).
- Destroy server (idempotent on absent).
- List all servers on this account.

## Related

- parent: `kei-provision/Cargo.toml`
- imports: clap, kei_provision, std

## Discussion

<div id="keicomments-mount" data-page=""></div>
<script type="module" src="/keicomments.js"></script>
