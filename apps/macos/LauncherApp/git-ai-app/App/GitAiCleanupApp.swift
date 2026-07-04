import AppKit
#if !XCODE_DIRECT_BUILD
import GitAiMacLogic
#endif
import SwiftUI

final class AppDelegate: NSObject, NSApplicationDelegate {
    func applicationDidFinishLaunching(_ notification: Notification) {
        NSApp.setActivationPolicy(.regular)
        NSApp.activate(ignoringOtherApps: true)
    }
}

@main
struct GitAiCleanupApp: App {
    @NSApplicationDelegateAdaptor(AppDelegate.self) private var appDelegate

    var body: some Scene {
        WindowGroup {
            CleanupView()
                .frame(minWidth: 860, minHeight: 560)
        }
    }
}
