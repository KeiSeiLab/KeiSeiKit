# Publishing to keigit.com

> Publish KeiSeiKit agents, skills, and primitives as scoped npm packages
> on the community registry at <https://keigit.com>. OAuth-only login,
> per-user PAT for `npm publish`, no email/password registration.

## Overview

`keigit.com` is the community-facing companion to KeiSeiKit: a public
Forgejo instance plus an npm-compatible package registry, run by the
project. Anyone with a GitHub or Google account can sign up in one
click, publish scoped packages under their own namespace, and share
agents / skills / hooks the same way they would on
`registry.npmjs.org` — but inside the KeiSeiKit ecosystem so package
DNAs cross-reference your existing substrate. Free, no quotas at
launch; quotas may be introduced if traffic justifies it.

Use it when you want others to `npm install @you/your-skill` against
your published bundle without depending on the public npm registry
or on private GitHub Packages.

## Sign up

1. Open <https://keigit.com>.
2. Click **Sign in with GitHub** or **Sign in with Google**.
3. Approve the OAuth scope on the provider page. You're back on
   keigit.com signed in; your username is your provider username
   (lowercased, special chars stripped).

There is no email/password registration form — OAuth is the only
path. If both GitHub and Google share the same primary email, the
first login wins and the second provider can be linked later under
**Settings → Linked Accounts**.

## Generate a PAT

`npm publish` needs a personal access token (PAT) with package-write
permission. Web cookies don't authenticate the npm CLI.

1. Click your avatar (top-right) → **Settings** → **Applications**.
2. Scroll to **Generate New Token**.
3. Name it (e.g. `npm-publish-laptop`).
4. Tick scopes:
   - `write:package` — required for `npm publish`
   - `write:repository` — only if you also push git repos with the
     same token; otherwise leave unchecked
5. Click **Generate Token**. Copy it now — you won't see it again.

Tokens are revocable from the same page. Rotate every 90 days or
immediately on suspected compromise.

## Configure npm

Two settings in your global `~/.npmrc`: the scope-to-registry mapping
and the auth token. Per-user scope means each publisher writes to
their own URL prefix — there is no shared `@keisei` namespace.

```bash
# Replace 'alice' with your keigit.com username.
npm config set @alice:registry https://keigit.com/api/packages/alice/npm/
npm config set //keigit.com/:_authToken <token-from-step-above>
```

Resulting `~/.npmrc`:

```ini
@alice:registry=https://keigit.com/api/packages/alice/npm/
//keigit.com/:_authToken=ghp_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx
```

Notes on portability:

- The `@scope:registry` line is what `npm` uses to route reads and
  writes for that scope. It must match the scope in your
  `package.json` `name` field exactly.
- The `//keigit.com/:_authToken` line is registry-host-scoped, so
  the same token covers any `@scope:registry` URL on `keigit.com`.
- For per-project overrides (CI, multi-account machines), put the
  same two lines into a `.npmrc` next to your `package.json`.
  Project-level beats user-level.

## Publish a package

Skeleton, using `@alice/keigit-smoke` (the shape proven by the
2026-05-01 smoke test):

```bash
mkdir keigit-smoke && cd keigit-smoke
npm init -y
```

Edit `package.json` so `name`, `version`, and `main` are set:

```json
{
  "name": "@alice/keigit-smoke",
  "version": "0.1.0",
  "description": "Smoke test of keigit.com npm publish path.",
  "main": "index.js",
  "license": "Apache-2.0",
  "repository": "https://keigit.com/alice/keigit-smoke"
}
```

```bash
echo "module.exports = { ok: true };" > index.js
npm publish
```

Expected output:

```
npm notice
npm notice 📦  @alice/keigit-smoke@0.1.0
npm notice === Tarball Contents ===
npm notice 33B  index.js
npm notice 220B package.json
npm notice === Tarball Details ===
npm notice name:          @alice/keigit-smoke
npm notice version:       0.1.0
npm notice ...
+ @alice/keigit-smoke@0.1.0
```

Re-publishing the same `version` is rejected — bump the version in
`package.json` (`npm version patch` does it for you) before each
`npm publish`.

## Install someone else's package

To consume `@alice/keigit-smoke` from a different machine or project:

```bash
# Once per machine — point the @alice scope at keigit
npm config set @alice:registry https://keigit.com/api/packages/alice/npm/

# Then install normally
npm install @alice/keigit-smoke
```

You don't need a PAT to install public packages — only to publish
and to install private packages. If `alice` later marks the package
private, install starts requiring an auth token under
`//keigit.com/:_authToken`.

## Verify on the web UI

Every publish lands on:

```
https://keigit.com/<user>/-/packages
```

For the example above:
<https://keigit.com/alice/-/packages>

Click the package name to see versions, tarball size, README (if
your `package.json` has `"readme"` or there's a `README.md` at the
root of the publish), and the `dist-tags` (default `latest`).

## Limits

- **Free tier, no quotas at launch.** Free for everyone, no per-user
  byte cap, no per-day publish-count cap. May be introduced when
  traffic warrants — current users will be grandfathered with at
  least 30 days notice.
- **Tarball size soft-limit ~50 MB per version.** Hard limits not
  enforced today; expect them later. If you have a >50 MB artefact,
  ship it as a release attachment on the git side, not as an npm
  tarball.
- **No mirroring of `registry.npmjs.org`.** Packages on keigit.com
  are independent. Don't `npm publish` a name you don't control on
  the public registry — it'll confuse downstream installs that have
  no `@scope:registry` override.

## Troubleshooting

**`401 Unauthorized` on `npm publish`** — PAT is wrong, expired, or
not in `~/.npmrc`. Check:

```bash
npm config get //keigit.com/:_authToken
```

If the output is `null` or doesn't match the token in **Settings →
Applications**, re-set it:

```bash
npm config set //keigit.com/:_authToken <fresh-token>
```

**`403 Forbidden` on `npm publish`** — token is valid but missing the
`write:package` scope, or the package name's scope doesn't match
your username. The scope segment of the package name (`@alice/...`)
must equal your keigit.com username. Re-check the token's scopes
on the **Applications** page; regenerate if needed.

**`404 Not Found` on `npm install`** — most often the
`@scope:registry` mapping is missing or wrong. The first time you
install from a new scope, run:

```bash
npm config get @alice:registry
```

Should print `https://keigit.com/api/packages/alice/npm/`. If it's
empty or points at `https://registry.npmjs.org/`, fix it with
`npm config set` per the **Configure npm** section.

**`E_PUBLISH_VERSION_EXISTS` / `Cannot publish over the previously
published versions`** — bump the `version` in `package.json` and
republish. keigit.com refuses overwrites of existing versions for
the same reason `registry.npmjs.org` does.

## Related docs

- [`docs/IMPORT-RUNTIME.md`](./IMPORT-RUNTIME.md) — `kei-import` for
  ingesting third-party Rust / TS / Python / Go repos
- [`docs/INSTALL.md`](./INSTALL.md) — kit-wide install paths
- [`docs/SECURITY.md`](./SECURITY.md) — kit-wide security posture,
  including how PATs and OAuth scopes are handled
