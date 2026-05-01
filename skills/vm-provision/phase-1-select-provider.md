# Phase 1 — Select Provider + Region + Plan

> Goal: lock `PROVIDER`, `REGION`, `PLAN`, `ARCH` via two AskUserQuestion
> calls. No provisioning yet — this is pure decision.
> **Verify criterion:** all four variables set; provider credentials (one
> env-var name) identified in `~/.claude/secrets/.env`.

---

## 1.a — First AskUserQuestion (4 options max)

**Provider?** (single-select, stored as `PROVIDER`):

- **Hetzner Cloud** — cheapest EU, CX22 x86 / CAX11 ARM64 both €3.79/mo
  [VERIFIED `_blocks/deploy-hetzner-cloud.md`]. Requires `HCLOUD_TOKEN`.
- **Vultr** — broad region list, HF compute, $5-10/mo tiers. Requires
  `VULTR_API_KEY`.
- **DigitalOcean** — strong US presence, simple API. Requires
  `DIGITALOCEAN_TOKEN`. Uses `deploy-vps-generic.md` cloud-init.
- **UpCloud** — preferred for RU-routed workloads (Finnish ASN). Requires
  `UPCLOUD_USERNAME` + `UPCLOUD_PASSWORD`.

If the intent argument mentions a provider already, pre-select it.

**Credential check BEFORE the click:** read `~/.claude/secrets/.env`; if the
chosen provider's env var is absent, surface a ONE-line remediation:

> "Provider X needs `<VAR>` in `~/.claude/secrets/.env`. Add it and
> re-invoke — I don't accept tokens pasted into chat (RULE 0.8)."

Do NOT proceed until the token is in place.

---

## 1.b — Second AskUserQuestion (region + plan + arch, 3 Q's)

Send three questions in one `AskUserQuestion` call. Options are
provider-specific; generate them from the following matrix (do NOT
hallucinate codes — re-verify against the provider doc link on each run):

**Region** (stored as `REGION`):

- Hetzner: `fsn1` (Falkenstein DE), `nbg1` (Nürnberg DE), `hel1` (Helsinki
  FI), `ash` (Ashburn US), `hil` (Hillsboro US), `sin` (Singapore)
  [VERIFIED https://docs.hetzner.com/cloud/general/locations].
- Vultr: `ams` (Amsterdam), `fra` (Frankfurt), `ewr` (Newark), `lax`
  (LA), `nrt` (Tokyo), `sgp` (Singapore).
- DigitalOcean: `nyc1/2/3`, `sfo3`, `ams3`, `fra1`, `lon1`, `sgp1`.
- UpCloud: `de-fra1`, `fi-hel1`, `fi-hel2`, `us-nyc1`, `sg-sin1`.

Pick the closest region to the user's stated audience. Prefer the EU when
the user doesn't specify (lower GDPR exposure).

**Plan** (stored as `PLAN`):

- Hetzner x86: `cx22` (2 vCPU / 4 GB / 40 GB / €3.79/mo), `cx32` (4 vCPU /
  8 GB / €6.79/mo).
- Hetzner ARM: `cax11` (2 vCPU / 4 GB / €3.79/mo), `cax21` (4 vCPU / 8 GB
  / €6.49/mo).
- Vultr: `vc2-1c-1gb` ($6/mo), `vc2-2c-4gb` ($24/mo), `vhp-1c-2gb-amd`
  ($14/mo).
- DigitalOcean: `s-1vcpu-1gb` ($6/mo), `s-2vcpu-2gb` ($18/mo).
- UpCloud: `1xCPU-1GB`, `2xCPU-2GB`.

Quote only the plans you can verify against the provider's live pricing
at call-time; do not embed stale pricing as fact.

**Arch** (stored as `ARCH`):

- `x86_64` — default; works with every Debian 12 image.
- `arm64` — Hetzner `cax*`, AWS Graviton, Oracle Ampere. ~25% cheaper.
  Rust builds run natively; Node/Python binary wheels may need extra
  install steps.

---

## 1.c — Verify criterion

Before moving to Phase 2:

- [ ] `PROVIDER`, `REGION`, `PLAN`, `ARCH` all set.
- [ ] The provider credential env-var EXISTS in `~/.claude/secrets/.env`
      (we only read the env-var name, never the value).
- [ ] The user clicked OK, not "back".

Emit one-liner:

`Phase 1 done: PROVIDER=<x> REGION=<y> PLAN=<z> ARCH=<a>. Credentials ref: $<VAR>.`

Proceed to Phase 2.

---

## 1.d — Constructive-fail paths

If the user says "I don't know":

- **(A)** Default to Hetzner CX22 fsn1 x86 (cheapest EU). 1-click.
- **(B)** Clone an existing project's provider (ask which project,
  pattern-match from `~/.claude/projects/*/memory/*.md`).
- **(C)** Defer provisioning — emit a decision memo and exit cleanly.

Never pick silently.
