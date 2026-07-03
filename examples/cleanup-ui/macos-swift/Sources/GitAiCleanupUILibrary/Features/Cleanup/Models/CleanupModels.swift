import Foundation

enum CleanupKind: String, CaseIterable, Identifiable {
    case nodeModules = "Node Modules"
    case buildFolders = "Build Folders"
    case devCleaner = "DevCleaner"

    var id: String { rawValue }
}

struct CleanupScanResponse: Decodable {
    let items: [CleanupItem]
}

struct CleanupItem: Decodable, Identifiable, Hashable {
    let path: String
    let target: String
    let sizeBytes: UInt64

    var id: String { path }

    var formattedSize: String {
        ByteCountFormatter.string(fromByteCount: Int64(sizeBytes), countStyle: .file)
    }

    var displayName: String {
        let url = URL(fileURLWithPath: path)
        let name = url.lastPathComponent
        if name.isEmpty {
            return path
        }
        if name == "node_modules", let parent = parentFolderName {
            return "\(parent) / node_modules"
        }
        return name
    }

    var groupName: String {
        if path.contains("/.cache/codex-runtimes/") {
            return "Codex Runtime Caches"
        }
        if path.contains("/.codex/.tmp/") {
            return "Codex Temporary Files"
        }
        if path.contains("/Documents/Wordspace/") {
            return "Workspace Projects"
        }
        if path.contains("/Library/Developer/Xcode/") {
            return "Xcode"
        }
        if path.contains("/Library/Developer/CoreSimulator/") {
            return "Simulator"
        }
        if path.contains("swiftpm") {
            return "SwiftPM Caches"
        }
        if path.contains("/.gradle/") {
            return "Gradle Caches"
        }
        if path.contains("/.npm") || path.contains("/.pnpm-store") {
            return "Package Manager Caches"
        }
        if path.contains("/Library/Caches/") {
            return "System Caches"
        }
        if path.contains("/.cache/") {
            return "User Caches"
        }
        if displayName == "node_modules" {
            return "Node Modules"
        }
        return target
    }

    var parentFolderName: String? {
        let parent = URL(fileURLWithPath: path).deletingLastPathComponent().lastPathComponent
        return parent.isEmpty ? nil : parent
    }

    enum CodingKeys: String, CodingKey {
        case path
        case target
        case sizeBytes = "size_bytes"
    }
}

struct CleanupItemGroup: Identifiable {
    let name: String
    let items: [CleanupItem]

    var id: String { name }

    var sizeBytes: UInt64 {
        items.map(\.sizeBytes).reduce(0, +)
    }

    var formattedSize: String {
        ByteCountFormatter.string(fromByteCount: Int64(sizeBytes), countStyle: .file)
    }
}

struct CleanupDeleteResponse: Decodable {
    let reports: [CleanupDeleteReport]
}

struct CleanupDeleteReport: Decodable, Identifiable {
    let path: String
    let deleted: Bool
    let error: String?

    var id: String { path }
}

struct CleanupErrorResponse: Decodable {
    let error: String
}
