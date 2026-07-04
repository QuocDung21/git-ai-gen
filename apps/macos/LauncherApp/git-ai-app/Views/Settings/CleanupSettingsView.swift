import SwiftUI

struct CleanupSettingsView: View {
    @Binding var showsCustomOptions: Bool
    @Binding var cleanupKind: CleanupKind
    let isWorking: Bool

    @State private var selectedTab: CleanupSettingsTab = .targets
    @Environment(\.dismiss) private var dismiss

    var body: some View {
        VStack(spacing: 0) {
            TabView(selection: $selectedTab) {
                targetsTab
                    .tabItem { Text("Targets") }
                    .tag(CleanupSettingsTab.targets)

                safetyTab
                    .tabItem { Text("Safety") }
                    .tag(CleanupSettingsTab.safety)

                aboutTab
                    .tabItem { Text("About") }
                    .tag(CleanupSettingsTab.about)
            }
            .padding(.horizontal, 20)
            .padding(.top, 18)

            Divider()

            HStack {
                Text(isWorking ? "Settings are locked while scanning." : activeTargetSummary)
                    .font(.caption)
                    .foregroundStyle(.secondary)

                Spacer()

                Button("Done") {
                    dismiss()
                }
                .keyboardShortcut(.defaultAction)
            }
            .padding(16)
        }
        .frame(width: 520, height: 360)
    }

    private var targetsTab: some View {
        Form {
            Section {
                Toggle("Use custom cleanup target", isOn: customOptionsBinding)
                    .disabled(isWorking)

                Picker("Cleanup target", selection: $cleanupKind) {
                    ForEach(CleanupKind.allCases) { kind in
                        Text(kind.rawValue).tag(kind)
                    }
                }
                .pickerStyle(.radioGroup)
                .disabled(!showsCustomOptions || isWorking)
            } header: {
                Text("Scan Target")
            } footer: {
                Text("Default scans only node_modules. Enable custom target when you want broader cleanup options.")
            }
        }
        .formStyle(.grouped)
        .padding(.top, 8)
    }

    private var safetyTab: some View {
        Form {
            Section {
                LabeledContent("Deletion mode", value: "Manual selection")
                LabeledContent("Default confirmation", value: "Required")
                LabeledContent("Scan behavior", value: "Streams results")
            } header: {
                Text("Protection")
            } footer: {
                Text("Future safety options can live here without crowding the main cleanup screen.")
            }
        }
        .formStyle(.grouped)
        .padding(.top, 8)
    }

    private var aboutTab: some View {
        Form {
            Section {
                LabeledContent("Rust core", value: "FFI static library")
                LabeledContent("Default target", value: CleanupKind.nodeModules.rawValue)
                LabeledContent("Custom targets", value: "Build Folders, DevCleaner")
            } header: {
                Text("Cleanup UI")
            } footer: {
                Text("Settings are split into tabs so more cleanup features can be added without changing the main workflow.")
            }
        }
        .formStyle(.grouped)
        .padding(.top, 8)
    }

    private var activeTargetSummary: String {
        let target = showsCustomOptions ? cleanupKind.rawValue : CleanupKind.nodeModules.rawValue
        return "Active target: \(target)"
    }

    private var customOptionsBinding: Binding<Bool> {
        Binding(
            get: { showsCustomOptions },
            set: { enabled in
                showsCustomOptions = enabled
                if !enabled {
                    cleanupKind = .nodeModules
                }
            }
        )
    }
}

private enum CleanupSettingsTab: Hashable {
    case targets
    case safety
    case about
}
