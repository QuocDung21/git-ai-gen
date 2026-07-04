import SwiftUI

public struct ContentView: View {
    public init() {}

    public var body: some View {
        CleanupView()
            .frame(
                minWidth: AppConstants.minimumWindowWidth,
                minHeight: AppConstants.minimumWindowHeight
            )
    }
}
