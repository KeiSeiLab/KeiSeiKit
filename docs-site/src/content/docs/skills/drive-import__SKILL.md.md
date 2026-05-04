---
title: SKILL
path: drive-import/SKILL.md
dna_hash: sha256:0c06640654edcf27
language: markdown
size_loc: 95
generated: by-keidocs
---

# drive-import/SKILL.md

## Public API

- `/drive-import — Google Drive → Forgejo project import` — ---
- `When to use` — - Importing one or more Google Drive folders as private repos into a local Forgejo dev-hub.
- `Pre-flight (run once, idempotent)` — 1. `./install.sh --profile=full-hub --yes` — installs Forgejo + rclone + gitleaks, auto-creates admin user `$USER` with random password (Keychain `forgejo-admin-password`) and access token (Keychain `forgejo-api-token`), stamps `~/.claude/secrets/.env` with `KEI_FORGEJO_USER`, `KEI_FORGEJO_URL`, `KEI_DRIVE_REMOTE`, `RCLONE_CONFIG`. Implementation: [`install/lib-dev-hub-forgejo.sh::_dhf_bootstrap_admin_user`](../install/lib-dev-hub-forgejo.sh).
- `Pipeline — what happens per folder` — ```
- `Invocation` — When user says "import these folders from drive" / "пилотную папку из Google Drive в форджео":
- `comments OK, blank lines OK` — drive:foo/bar
- `Failure modes — diagnose` — | Symptom | Cause | Fix |
- `Don't` — - Don't try `--scan` on a 10k-folder root — preflight enumerates gdocs first which is slow
- `See also` — - Wizard source: `_templates/drive-import-wizard.sh.tmpl`

## Related

- parent: `drive-import`

## Discussion

<script src="https://giscus.app/client.js"
        data-repo="KeiSei84/KeiSeiKit-1.0"
        data-repo-id="PLACEHOLDER_REPO_ID"
        data-category="wiki-comments"
        data-category-id="PLACEHOLDER_CATEGORY_ID"
        data-mapping="pathname"
        data-strict="0"
        data-reactions-enabled="1"
        data-emit-metadata="0"
        data-input-position="bottom"
        data-theme="preferred_color_scheme"
        data-lang="en"
        data-loading="lazy"
        crossorigin="anonymous"
        async></script>
