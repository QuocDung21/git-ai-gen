import Foundation

public enum CleanupFormatting {
    public static func formatSize(_ sizeBytes: UInt64) -> String {
        ByteCountFormatter.string(fromByteCount: Int64(sizeBytes), countStyle: .file)
    }
}
