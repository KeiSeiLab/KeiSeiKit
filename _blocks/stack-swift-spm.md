# STACK — Swift SPM executable (macOS)

Use for platform-native macOS UI. Requires some non-obvious incantations to avoid silent failures.

**Info.plist embed — each arg prefixed with `-Xlinker`:**
```
.unsafeFlags([
  "-Xlinker", "-sectcreate",
  "-Xlinker", "__TEXT",
  "-Xlinker", "__info_plist",
  "-Xlinker", "/abs/path/Info.plist",
])
```
Relative paths silently fail. `NSPrincipalClass=NSApplication` in Info.plist MANDATORY — without it the binary runs as a console tool, no menubar, no events.

**Codesign:** `codesign --force --sign - <path>/MyApp.app` — ad-hoc signature is enough for local use; Gatekeeper flags unsigned `.app` bundles as damaged.

**Menubar lifecycle (mandatory dance):**
1. `NSApp.setActivationPolicy(.regular)` at launch
2. Create `NSStatusItem` via `NSStatusBar.system.statusItem(withLength: .variable)`
3. `NSApp.setActivationPolicy(.accessory)` AFTER status item is attached

Skip any step → icon never appears, no error, silent failure.

**Broken / forbidden:**
- `MenuBarExtra` (SwiftUI) — does NOT work with SPM executables. Use `NSStatusItem` + SwiftUI popover.
- Notch overflow (MacBook Pro 14/16 M1+) — new status items hidden behind notch. Verify visibility post-install.

**LaunchAgent hygiene (learned from a real disk-bloat incident):** a duplicate LaunchAgent or a chatty sync daemon without log-silencing can fill the disk with tens of GB of log chatter. Check `launchctl list` before adding a LaunchAgent, and keep LaunchAgent stdout/stderr → `/dev/null`.
