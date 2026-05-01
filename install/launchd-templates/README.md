# launchd plist templates — dev-hub bundle

Each `<service>.plist.tmpl` is a launchd LaunchAgent template with `${VAR}`
placeholders that are substituted at install time by `install/lib-launchd.sh`.

## Substitution variables

| Variable     | Source                              | Example                                       |
|--------------|-------------------------------------|-----------------------------------------------|
| `${HOME}`    | `$HOME` env var of installing user  | `/Users/alice`                                |
| `${USER}`    | `$USER` env var                     | `alice`                                       |
| `${BREW}`    | `$(brew --prefix)`                  | `/opt/homebrew` (Apple Silicon) or `/usr/local` |
| `${KIT}`     | `~/.claude/agents/_primitives`      | full path to kit primitives root              |
| `${LOGS}`    | `~/Library/Logs/keisei/<service>`   | per-service log dir (auto-created)            |
| `${DATA}`    | `~/Library/Application Support/keisei/<service>` | per-service data dir (auto-created) |

## Naming convention

- File:   `<service>.plist.tmpl`
- Label:  `com.keisei.<service>` (must match `Label` key inside)
- Output: `~/Library/LaunchAgents/com.keisei.<service>.plist` (rendered)

## Convention for new templates

1. Always `RunAtLoad=true` + `KeepAlive=true` for long-lived daemons
2. Always set `StandardOutPath` and `StandardErrorPath` under `${LOGS}/`
3. Always `WorkingDirectory=${DATA}` so service has a writeable cwd
4. Resource limits via `SoftResourceLimits` / `HardResourceLimits` dict
5. macOS arm64 only (kit doesn't ship Linux service-mgmt)

## Activation

Renderer (`install/lib-launchd.sh::render_plist`) reads `<svc>.plist.tmpl`,
substitutes vars, writes to `~/Library/LaunchAgents/com.keisei.<svc>.plist`,
runs `launchctl bootstrap gui/$(id -u) <path>` to activate. `brew services
start <svc>` is preferred when the service is brewed (it wraps launchctl).
