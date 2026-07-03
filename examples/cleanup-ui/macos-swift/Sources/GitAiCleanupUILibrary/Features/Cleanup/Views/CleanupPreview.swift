import SwiftUI

struct CleanupPreviewHost: View {
    var body: some View {
        CleanupView()
            .frame(width: 860, height: 560)
    }
}

#Preview("Cleanup") {
    CleanupPreviewHost()
}
