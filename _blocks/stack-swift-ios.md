# STACK — Swift iOS (UIKit / SwiftUI hybrid)

Use for platform-native iOS UI — this is the only sane choice for iOS.

**UIKit vs SwiftUI:**
- SwiftUI for new screens by default (iOS 16+ targets). Wrap UIKit views via `UIViewRepresentable` only when SwiftUI has no equivalent (AVKit camera, ARKit, MapKit gestures).
- UIKit required for: deep `UITextInput` custom protocols, scroll-view precise tracking, `UIPageViewController` paging animations < 60 fps on SwiftUI.

**App lifecycle:**
- `@main` struct App or `AppDelegate`/`SceneDelegate` pair. NOT both — pick one.
- `LaunchScreen.storyboard` required (Info.plist key `UILaunchStoryboardName`) — Apple rejects static image launch.

**Info.plist mandatory keys:**
- `NSCameraUsageDescription` / `NSPhotoLibraryUsageDescription` / `NSLocationWhenInUseUsageDescription` — if capability used; missing → runtime crash, not build error.
- `CFBundleURLTypes` for custom URL schemes (deeplinks).
- `NSAppTransportSecurity` — never set `NSAllowsArbitraryLoads=true` in prod (App Store rejection).
- `UIBackgroundModes` array for any background audio / location / BLE.

**Threading:** `@MainActor` for UI mutation; `actor` for shared mutable state; `Task { ... }` for async. NO `DispatchQueue.main.async` wrapping UI updates from Swift Concurrency code (defeats actor isolation).

**Forbidden:** `NSAllowsArbitraryLoads=true`, force-unwrapping `UIImage(named:)` (use failable init), hardcoded API keys in `.swift` sources (use `.xcconfig` + `Bundle.main.infoDictionary`).
