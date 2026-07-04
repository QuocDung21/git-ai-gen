import Foundation

public enum CleanupSelectionLogic {
    public static func selectedSize(items: [CleanupItem], selected: Set<String>) -> UInt64 {
        items
            .filter { selected.contains($0.id) }
            .map(\.sizeBytes)
            .reduce(0, +)
    }

    public static func selectedPaths(items: [CleanupItem], selected: Set<String>) -> [String] {
        items
            .filter { selected.contains($0.id) }
            .map(\.path)
    }
}
