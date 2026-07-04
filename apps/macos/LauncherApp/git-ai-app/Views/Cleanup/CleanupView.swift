import AppKit
import SwiftUI

public struct CleanupView: View {
    @State private var rootPath = FileManager.default.homeDirectoryForCurrentUser.path
    @State private var cleanupKind: CleanupKind = .nodeModules
    @State private var showsCustomOptions = false
    @State private var items: [CleanupItem] = []
    @State private var selected = Set<String>()
    @State private var reports: [CleanupDeleteReport] = []
    @State private var isWorking = false
    @State private var status = "Ready"
    @State private var errorMessage: String?
    @State private var showsSettings = false
    @State private var showsDeleteConfirmation = false
    @State private var showsBroadScanConfirmation = false
    @State private var broadScanDontAskAgainDraft = false
    @State private var expandedGroups = Set<String>()
    @State private var scanCancellationToken: ScanCancellationToken?
    @AppStorage("cleanup.skipBroadScanWarning") private var skipsBroadScanWarning = false

    public init() {}

    public var body: some View {
        VStack(spacing: 0) {
            toolbar
            Divider()
            content
            Divider()
            footer
        }
        .alert("Cleanup Error", isPresented: errorBinding) {
            Button("OK") { errorMessage = nil }
        } message: {
            Text(errorMessage ?? "")
        }
        .alert(deleteConfirmationTitle, isPresented: $showsDeleteConfirmation) {
            Button("Cancel", role: .cancel) {}
            Button("Delete", role: .destructive) {
                deleteSelected()
            }
        } message: {
            Text(deleteConfirmationMessage)
        }
        .sheet(isPresented: $showsBroadScanConfirmation) {
            CleanupBroadScanWarningView(
                dontAskAgain: $broadScanDontAskAgainDraft,
                onCancel: {
                    showsBroadScanConfirmation = false
                },
                onContinue: {
                    skipsBroadScanWarning = broadScanDontAskAgainDraft
                    showsBroadScanConfirmation = false
                    DispatchQueue.main.async {
                        startScan()
                    }
                }
            )
        }
        .sheet(isPresented: $showsSettings) {
            CleanupSettingsView(
                showsCustomOptions: $showsCustomOptions,
                cleanupKind: $cleanupKind,
                isWorking: isWorking
            )
        }
    }

    private var toolbar: some View {
        VStack(alignment: .leading, spacing: 12) {
            HStack(spacing: 8) {
                TextField("Root folder", text: $rootPath)
                    .textFieldStyle(.roundedBorder)

                Button("Browse") {
                    chooseFolder()
                }

                Button(isWorking ? "Cancel Scan" : "Scan") {
                    if isWorking {
                        cancelScan()
                    } else {
                        scan()
                    }
                }
                .keyboardShortcut("r", modifiers: [.command])

                Button("Settings") {
                    showsSettings = true
                }
                .disabled(isWorking)

                if isWorking {
                    ProgressView()
                        .controlSize(.small)
                }
            }

            HStack {
                Button("Select All") {
                    selected = Set(items.map(\.id))
                }
                .disabled(items.isEmpty)

                Button("Clear Selection") {
                    selected.removeAll()
                }
                .disabled(selected.isEmpty)

                Spacer()

                Button("Delete Selected", role: .destructive) {
                    showsDeleteConfirmation = true
                }
                .disabled(selected.isEmpty || isWorking)
            }
        }
        .padding(16)
    }

    private var content: some View {
        HSplitView {
            CleanupResultsView(
                items: items,
                selected: $selected,
                expandedGroups: $expandedGroups
            )

            VStack(alignment: .leading, spacing: 8) {
                Text("Delete Reports")
                    .font(.headline)
                ScrollView {
                    LazyVStack(alignment: .leading, spacing: 8) {
                        ForEach(reports) { report in
                            VStack(alignment: .leading, spacing: 4) {
                                Text(report.deleted ? "Deleted" : "Failed")
                                    .font(.caption)
                                    .foregroundStyle(report.deleted ? .green : .red)
                                Text(report.path)
                                    .font(.system(.caption, design: .monospaced))
                                if let error = report.error {
                                    Text(error)
                                        .font(.caption)
                                        .foregroundStyle(.secondary)
                                }
                            }
                            .frame(maxWidth: .infinity, alignment: .leading)
                            .padding(8)
                            .background(Color(nsColor: .controlBackgroundColor))
                            .clipShape(RoundedRectangle(cornerRadius: 6))
                        }
                    }
                    .padding(12)
                }
            }
            .padding(16)
            .frame(minWidth: 280)
        }
    }
    private var footer: some View {
        HStack {
            Text(status)
                .foregroundStyle(.secondary)
            Spacer()
            Text("\(items.count) found")
            Text(activeCleanupKind.rawValue)
            Text("\(selected.count) selected")
            Text("\(formattedTotalSize) total")
            Text("\(formattedSelectedSize) selected")
        }
        .font(.caption)
        .padding(.horizontal, 16)
        .padding(.vertical, 10)
    }

    private var errorBinding: Binding<Bool> {
        Binding(
            get: { errorMessage != nil },
            set: { if !$0 { errorMessage = nil } }
        )
    }

    private var formattedTotalSize: String {
        CleanupFormatting.formatSize(items.map(\.sizeBytes).reduce(0, +))
    }

    private var formattedSelectedSize: String {
        CleanupFormatting.formatSize(CleanupSelectionLogic.selectedSize(items: items, selected: selected))
    }

    private var deleteConfirmationMessage: String {
        if CleanupBroadScanPolicy.isBroadScanPath(rootPath) {
            return "This scan started from a broad root: \(rootPath).\n\nYou are about to permanently delete \(selected.count) selected folder(s), freeing about \(formattedSelectedSize). Review each selected path carefully. This cannot be undone."
        }

        return "This will permanently delete \(selected.count) selected folder(s), freeing about \(formattedSelectedSize). This cannot be undone."
    }

    private var deleteConfirmationTitle: String {
        CleanupBroadScanPolicy.isBroadScanPath(rootPath) ? "Delete results from broad root?" : "Delete selected folders?"
    }

    private func chooseFolder() {
        let panel = NSOpenPanel()
        panel.canChooseFiles = false
        panel.canChooseDirectories = true
        panel.allowsMultipleSelection = false
        panel.directoryURL = URL(fileURLWithPath: rootPath)

        if panel.runModal() == .OK, let url = panel.url {
            rootPath = url.path
        }
    }

    private func scan() {
        if CleanupBroadScanPolicy.isBroadScanPath(rootPath) && !skipsBroadScanWarning {
            broadScanDontAskAgainDraft = skipsBroadScanWarning
            showsBroadScanConfirmation = true
            return
        }

        startScan()
    }

    private func startScan() {
        isWorking = true
        status = "Scanning..."
        items = []
        reports = []
        selected.removeAll()
        expandedGroups.removeAll()

        let scanPath = rootPath
        let scanKind = activeCleanupKind
        let cancellationToken = ScanCancellationToken()
        scanCancellationToken = cancellationToken

        DispatchQueue.global(qos: .userInitiated).async {
            do {
                try RustCleanupBridge.scanStreaming(
                    path: scanPath,
                    kind: scanKind,
                    cancellationToken: cancellationToken
                ) { item in
                    DispatchQueue.main.async {
                        guard !cancellationToken.isCancelled else { return }
                        items.append(item)
                        status = "Scanning... \(items.count) found"
                    }
                }

                DispatchQueue.main.async {
                    guard scanCancellationToken === cancellationToken else { return }
                    items.sort { $0.path < $1.path }
                    status = cancellationToken.isCancelled ? "Scan cancelled" : "Scan complete"
                    isWorking = false
                    scanCancellationToken = nil
                }
            } catch {
                DispatchQueue.main.async {
                    guard scanCancellationToken === cancellationToken else { return }
                    errorMessage = error.localizedDescription
                    status = "Scan failed"
                    isWorking = false
                    scanCancellationToken = nil
                }
            }
        }
    }

    private func cancelScan() {
        scanCancellationToken?.cancel()
        status = "Cancelling scan..."
    }

    private func deleteSelected() {
        isWorking = true
        status = "Deleting..."

        let paths = CleanupSelectionLogic.selectedPaths(items: items, selected: selected)

        do {
            reports = try RustCleanupBridge.delete(paths: paths)
            let deleted = Set(reports.filter(\.deleted).map(\.path))
            items.removeAll { deleted.contains($0.path) }
            selected.subtract(deleted)
            status = "Delete complete"
        } catch {
            errorMessage = error.localizedDescription
            status = "Delete failed"
        }

        isWorking = false
    }

    private var activeCleanupKind: CleanupKind {
        showsCustomOptions ? cleanupKind : .nodeModules
    }
}
