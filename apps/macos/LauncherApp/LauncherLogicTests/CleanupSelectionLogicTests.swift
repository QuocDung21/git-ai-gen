import XCTest
@testable import GitAiMacLogic

final class CleanupSelectionLogicTests: XCTestCase {
    func testSelectedSizeAndPaths() {
        let items = [
            CleanupItem(path: "/tmp/a", target: "node_modules", sizeBytes: 10),
            CleanupItem(path: "/tmp/b", target: "build", sizeBytes: 25),
            CleanupItem(path: "/tmp/c", target: "cache", sizeBytes: 40),
        ]

        let selected: Set<String> = ["/tmp/a", "/tmp/c"]

        XCTAssertEqual(CleanupSelectionLogic.selectedSize(items: items, selected: selected), 50)
        XCTAssertEqual(CleanupSelectionLogic.selectedPaths(items: items, selected: selected), ["/tmp/a", "/tmp/c"])
    }
}
