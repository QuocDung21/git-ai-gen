import AppKit
#if !XCODE_DIRECT_BUILD
import GitAiMacLogic
#endif
import SwiftUI

@main
struct GitAiApp: App {
    @NSApplicationDelegateAdaptor(AppDelegate.self) private var appDelegate

    var body: some Scene {
        WindowGroup {
            ContentView()
        }
    }
}
