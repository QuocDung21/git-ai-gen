import SwiftUI

struct CleanupBroadScanWarningView: View {
    @Binding var dontAskAgain: Bool
    let onCancel: () -> Void
    let onContinue: () -> Void

    var body: some View {
        VStack(alignment: .leading, spacing: 18) {
            Text("Scan a broad folder?")
                .font(.title2)
                .fontWeight(.semibold)

            Text("The selected root is broad and may take a long time. The scanner will skip heavy personal folders where possible, but disk usage can still spike during size calculation.")
                .foregroundStyle(.secondary)
                .fixedSize(horizontal: false, vertical: true)

            Toggle("Don't ask again", isOn: $dontAskAgain)

            HStack {
                Spacer()

                Button("Cancel") {
                    onCancel()
                }
                .keyboardShortcut(.cancelAction)

                Button("Scan Anyway") {
                    onContinue()
                }
                .keyboardShortcut(.defaultAction)
            }
        }
        .padding(24)
        .frame(width: 440)
    }
}
