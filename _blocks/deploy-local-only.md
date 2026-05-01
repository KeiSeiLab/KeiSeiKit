# DEPLOY — LOCAL ONLY (sensitive / pre-disclosure project)

Use this block for any project that CANNOT be publicly deployed — typical triggers: proprietary ML weights/architectures you don't want in public training corpora, security tooling that burns its own usefulness on exposure, kernel-level code, client-confidential codebases.

**Hard forbidden (no matter how small the change):**
- Public-URL share pages / static HTML dumps to public hosting
- Vercel / Netlify / GitHub Pages / Cloudflare Pages public deploy
- `gh repo create` public, `gh repo edit --visibility public`
- `git push` to a public remote (GitHub, public GitLab)
- Publishing architecture diagrams with node counts, param totals, or training configs
- Public benchmark tables naming this project

**Allowed:**
- Private remotes (self-hosted Forgejo/Gitea over SSH on a private network)
- Tailscale-only internal services
- Local-only `127.0.0.1` / LAN dev servers
- `.app` / `.dmg` distribution via private channels

**Double-confirmation override (both phrases required, in order, exact wording):**
1. "yes, deploy"
2. "I confirm publication"

No approximations. Informal variants do NOT count. If either phrase is absent, refuse.

**Example categories that typically require local-only:** censorship-circumvention tooling (public push burns exit-node IPs), ML ensembles with trained weights, control / guidance algorithms, offensive security research.

**Report field:** "Public-deploy surface touched: none | <explicit surface> — double-confirm obtained yes/no."
